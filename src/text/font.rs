use std::{cell::RefCell, hash::Hash, num::NonZeroU16, path::Path};
use hashbrown::HashMap;

use crate::prelude::{draw_texture_pro, Color, DrawHandle, PixelFormat, Raylib, Rectangle, Texture, Vector2};

use super::cache::{FontCache, LineMetrics, Metrics};

/// A font created from a `.tff` or `.otf` file.
pub struct TrueTypeFont(fontdue::Font);

impl TrueTypeFont {
    /// Create a font from binary `ttf` or `otf` data.
    /// # Examples
    /// Load a font present in the project directory and draw text with it:
    /// ```
    /// # use raylib::prelude::*;
    /// # let rl = &mut init_window(800, 800, "Font rendering", 60);
    /// let font = TrueTypeFont::from_bytes(include_bytes!("../../assets/TerminusTTF.ttf").as_slice()).unwrap();
    /// let mut atlas = font.atlas(rl, 32.0);
    /// while !window_should_close(rl) {
    ///     begin_drawing(rl, |rl| {
    ///         clear_background(rl, Color::RAYWHITE);
    ///         draw_text(rl, &mut atlas, "Hello, Terminus!", vec2(20.0, 20.0), 32.0, Color::BLACK);
    ///     });
    ///     # break
    /// }
    /// ```
    pub fn from_bytes(bytes: impl std::ops::Deref<Target = [u8]>) -> Result<Self, &'static str> {
        Ok(TrueTypeFont(fontdue::Font::from_bytes(bytes, fontdue::FontSettings::default())?))
    }

    /// Create a font from an already created `fontdue` font.
    pub fn from_fontdue(font: fontdue::Font) -> Self {
        TrueTypeFont(font)
    }

    /// Get access to the inner fontdue object.
    pub fn inner(&self) -> &fontdue::Font {
        &self.0
    }
}

#[derive(Clone, Copy)]
struct FontSizeKey(f32);

impl PartialEq for FontSizeKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 ||
        (self.0.is_nan() && other.0.is_nan())
    }
}
impl Eq for FontSizeKey {}

impl Hash for FontSizeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits());
    }
}

pub fn load_font(rl: &Raylib, path: impl AsRef<Path>) -> std::io::Result<TrueTypeFontCache> {
    use std::io::{Error, ErrorKind};

    let bytes = std::fs::read(path)?;
    let font = TrueTypeFont::from_bytes(bytes.as_slice()).map_err(|s| Error::new(ErrorKind::InvalidData, s))?;
    Ok(load_font_ex(rl, font))
}

pub fn load_font_ex(_rl: &Raylib, font: TrueTypeFont) -> TrueTypeFontCache {
    TrueTypeFontCache {
        font: font.0,
        atlases: RefCell::default()
    }
}

pub struct TrueTypeFontCache {
    font: fontdue::Font,
    atlases: RefCell<HashMap<FontSizeKey, TrueTypeFontAtlas>>
}

impl FontCache for TrueTypeFontCache {
    fn codepoints(&self) -> &HashMap<char, NonZeroU16> { self.font.chars() }
    fn glyph_count(&self) -> u16 { self.font.glyph_count() }
    fn line_metrics(&self, size: f32) -> Option<LineMetrics> {
        self.font.horizontal_line_metrics(size).map(|m| LineMetrics {
            ascent: m.ascent, descent: m.descent, line_gap: m.line_gap
        })
    }

    fn metrics_indexed(&self, index: u16, size: f32) -> Metrics {
        let m = self.font.metrics_indexed(index, size);
        Metrics {
            xmin: m.xmin as f32,
            ymin: m.ymin as f32,
            width: m.width as f32,
            height: m.height as f32,
            advance_width: m.advance_width
        }
    }
    fn kern_indexed(&self, left: u16, right: u16, size: f32) -> Option<f32> { self.font.horizontal_kern_indexed(left, right, size) }

    fn draw_glyph(&self, rl: &DrawHandle, index: u16, size: f32, dest: Rectangle, color: Color) {
        let key = FontSizeKey(size);
        let mut atlases = self.atlases.borrow_mut();

        let atlas = if let Some(atlas) = atlases.get_mut(&key) {
            atlas
        } else {
            let atlas = TrueTypeFontAtlas::new(rl, &self.font);
            // SAFETY: we just checked above that `key` was not yet inserted into the map
            let (_, atlas) = atlases.insert_unique_unchecked(key, atlas);
            atlas
        };

        let rec = atlas.get_glyph(&self.font, index, size);

        draw_texture_pro(rl, atlas.texture(), rec, dest, Vector2::ZERO, 0.0, color);
    }
}

struct TrueTypeFontAtlas {
    texture: Texture,
    recs: Vec<Option<Rectangle>>,
    atlas_rectangle: AtlasRectangle
}

struct AtlasRectangle {
    offset_x: usize,
    offset_y: usize,
    min_x: usize,
    max_y: usize,
    pot_y: usize
}

impl TrueTypeFontAtlas {
    fn new(rl: &Raylib, font: &fontdue::Font) -> Self {
        Self {
            texture: Texture::load_empty(rl, 128, 128, PixelFormat::UncompressedGrayAlpha).unwrap(),
            recs: vec![None; font.glyph_count() as usize],
            atlas_rectangle: AtlasRectangle::new(128)
        }
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }

    fn get_glyph(&mut self, font: &fontdue::Font, index: u16, size: f32) -> Rectangle {
        let rec = &mut self.recs[index as usize];
        // skip if glyph was already rendered
        if let Some(rec) = rec { return *rec }

        let (_, rasterized) = font.rasterize_indexed(index, size);
        // convert to GrayAlpha
        let mut data = Vec::with_capacity(rasterized.len()*2);
        for alpha in rasterized {
            data.push(255);
            data.push(alpha);
        }

        let new_rec = self.atlas_rectangle.get_new_rect(&mut self.texture, font.metrics_indexed(index, size));
        self.texture.update_rec_raw(new_rec, &data).unwrap();
        *rec = Some(new_rec);
        new_rec
    }
}

impl AtlasRectangle {
    fn new(texture_size: usize) -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            min_x: 0,
            max_y: 0,
            pot_y: texture_size
        }
    }

    /// Asks for a new rectangle in the texture atlas.
    /// May resize the given texture.
    /// `metrics` can be obtained using [`fontdue::Font::metrics_indexed`].
    /// Returns the position of the glyph in the texture.
    fn get_new_rect(&mut self, texture: &mut Texture, metrics: fontdue::Metrics) -> Rectangle {
        let mut size = texture.width() as usize;
           
        // went over the right-most edge, go back to the start of the line
        if self.offset_x + metrics.width >= size {
            self.offset_x = self.min_x;
            self.offset_y = self.max_y;
        }

        // went to the bottom of the screen, resize and use the new part to the right
        if self.offset_y + metrics.height >= size {
            self.pot_y = self.max_y;

            self.min_x = size;
            self.max_y = 0;

            self.offset_x = size;
            self.offset_y = 0;

            // resize texture
            size *= 2;
            texture.resize_canvas(texture.width()*2, texture.height()*2, 0, 0, Color::BLANK);
        }

        // finished using the new right part, use lower half of resized
        if self.offset_y + metrics.height >= self.pot_y {
            self.offset_x = 0;
            self.offset_y = self.pot_y;

            self.min_x = 0;
            self.max_y = self.pot_y;
            self.pot_y = size;
        }

        let rec = Rectangle::new(self.offset_x as f32, self.offset_y as f32, metrics.width as f32, metrics.height as f32);

        self.offset_x += metrics.width;
        self.max_y = self.max_y.max(self.offset_y + metrics.height);

        rec
    }

}
