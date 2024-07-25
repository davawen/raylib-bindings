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

        let atlas = if let Some(atlas) = atlases.get(&key) {
            atlas
        } else {
            let (texture_size, recs) = TrueTypeFontAtlas::map_texture(&self.font, size);
            let texture = Texture::load_empty(rl, texture_size as u32, texture_size as u32, PixelFormat::UncompressedGrayAlpha).unwrap();
            let atlas = TrueTypeFontAtlas { texture: texture.into(), size, recs: recs.into() };
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
    size: f32,
    recs: RefCell<Vec<(bool, Rectangle)>>
}

impl TrueTypeFontAtlas {
    /// Returns the size of the texture expected and the vec containing the position of the glyphs.
    fn map_texture(font: &fontdue::Font, px: f32) -> (usize, Vec<(bool, Rectangle)>) {
        let mut size = 128;
        let mut recs = Vec::with_capacity(font.glyph_count().into());

        let mut offset_x = 0;
        let mut offset_y = 0;

        let mut min_x = 0;
        let mut max_y = 0;

        let mut pot_y = size;

        for idx in 0..font.glyph_count() {
            let metrics = font.metrics_indexed(idx, px);
            
            // went over the right-most edge, go back to the start of the line
            if offset_x + metrics.width >= size {
                offset_x = min_x;
                offset_y = max_y;

                // went to the bottom of the screen, resize and use the new part to the right
                if offset_y + metrics.height >= size {
                    pot_y = max_y;

                    min_x = size;
                    max_y = 0;

                    offset_x = size;
                    offset_y = 0;

                    size *= 2;
                }

                // finished using the new right part, use lower half of resized
                if offset_y + metrics.height >= pot_y {
                    offset_x = 0;
                    offset_y = pot_y;

                    min_x = 0;
                    max_y = pot_y;
                    pot_y = size;
                }

            }

            recs.push((false, Rectangle::new(offset_x as f32, offset_y as f32, metrics.width as f32, metrics.height as f32)));

            offset_x += metrics.width;
            max_y = max_y.max(offset_y + metrics.height);
        }

        (size, recs)
    }

    fn texture(&self) -> &Texture {
        &self.texture
    }

    fn get_glyph(&self, font: &fontdue::Font, index: u16, _size: f32) -> Rectangle {
        let (rendered, rec) = &mut self.recs.borrow_mut()[index as usize];
        // skip if glyph was already rendered
        if *rendered { return *rec }

        let (_, rasterized) = font.rasterize_indexed(index, self.size);
        // convert to GrayAlpha
        let mut data = Vec::with_capacity(rasterized.len()*2);
        for alpha in rasterized {
            data.push(255);
            data.push(alpha);
        }
        self.texture.update_rec_raw(*rec, &data).unwrap();
        *rendered = true;
        *rec
    }
}

