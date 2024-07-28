use std::{cell::RefCell, hash::Hash, num::NonZeroU16, path::Path};
use hashbrown::HashMap;

use crate::prelude::{draw_texture_pro, Color, DrawHandle, PixelFormat, Raylib, Rectangle, Texture, TextureFilter, Vector2};

use super::cache::{FontCache, LineMetrics, Metrics};

/// A font created from a `.tff` or `.otf` file.
pub struct TrueTypeFont(fontdue::Font);

impl TrueTypeFont {
    /// Create a font from binary `ttf` or `otf` data.
    /// 
    /// # Examples
    /// Load a font present in the project directory and draw text with it:
    /// ```
    /// # use raylib::prelude::*;
    /// # let rl = &mut init_window(800, 800, "Font rendering", 60);
    /// let font = TrueTypeFont::from_bytes(include_bytes!("../../assets/TerminusTTF.ttf").as_slice()).unwrap();
    /// let font = load_font_ex(rl, font);
    /// while !window_should_close(rl) {
    ///     begin_drawing(rl, |rl| {
    ///         clear_background(rl, Color::RAYWHITE);
    ///         draw_text(rl, &font, "Hello, Terminus!", vec2(20.0, 20.0), 32.0, Color::BLACK);
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

/// Loads a font into raylib from a path, with default [`FontParams`].
/// 
/// Supports `.ttf` and `.otf` data. 
/// Characters are rendered at the right resolution when calls to [`draw_text`][`super::cache::draw_text`] are issued. 
/// See [`load_font_ex`] and [`TrueTypeFont`] to have more control over font loading. 
/// 
/// To use the default raylib font, see [`Raylib::default_font`].
///
/// # Examples
/// ```
/// # use raylib::prelude::*;
/// # let mut rl = &mut init_window(100, 100, "load_font", 60);
/// let font = load_font(rl, "TerminusTTF.ttf").unwrap();
/// begin_drawing(rl, |rl| {
///     draw_text(rl, &font, "Hello, world!", vec2(20.0, 60.0), Color::BLACK);
/// });
/// ```
pub fn load_font(rl: &Raylib, path: impl AsRef<Path>) -> std::io::Result<TrueTypeFontCache> {
    use std::io::{Error, ErrorKind};

    let bytes = std::fs::read(path)?;
    let font = TrueTypeFont::from_bytes(bytes.as_slice()).map_err(|s| Error::new(ErrorKind::InvalidData, s))?;
    Ok(load_font_ex(rl, font, FontParams::default()))
}

/// Loads a font into raylib from bytes.
/// 
/// # Panics
/// Panics if the given bytes are not a valid font. 
/// Use [`TrueTypeFont::from_bytes`] and [`load_font_ex`] if you need to handle a potential error. 
/// 
/// # Examples
/// Include a font in the project directory in the executable:
/// ```
/// # use raylib::prelude::*;
/// # let mut rl = &mut init_window(100, 100, "font from bytes");
/// let font = load_font_bytes(rl, include_bytes!("assets/TerminusTTF.ttf").as_slice());
/// begin_drawing(rl, |rl| {
///     draw_text(rl, &font, "Hello, Terminus!", vec2(20.0, 20.0), 20.0, Color::BLACK);
/// });
/// ```
pub fn load_font_bytes(rl: &Raylib, bytes: &[u8]) -> TrueTypeFontCache {
    let font = TrueTypeFont::from_bytes(bytes).expect("a valid ttf or otf font in bytes");
    load_font_ex(rl, font, FontParams::default())
}

/// Loads a [`TrueTypeFont`] into raylib with the given [`FontParams`].
pub fn load_font_ex(_rl: &Raylib, font: TrueTypeFont, params: FontParams) -> TrueTypeFontCache {
    TrueTypeFontCache {
        font: font.0,
        params,
        atlases: RefCell::default()
    }
}

/// Parameters for loading a [`TrueTypeFont`] into a [`TrueTypeFontCache`].
/// 
/// See [`load_font_ex`]. 
/// For default values (used in [`load_font`] and [`load_font_bytes`]), see [`FontParams::default`]. 
/// This struct may be extended in the future. 
pub struct FontParams {
    /// The minimum size at which the font will be rendered.
    /// For sizes smaller than this, the texture will get scaled down.
    pub min_size: f32,
    /// The maximum size at which the font will be rendered.
    /// For sizes bigger than this, the texture will get scaled up.
    /// This is set to prevent arbitrarly large texture allocations.
    pub max_size: f32,
    /// Whether to always round font sizes.
    /// This is on by default, because in most cases fractional pixel-size fonts are not wanted and might cause unnecessary texture allocations.
    pub integer_size: bool,
    /// The texture filter used by the texture atlases. 
    pub texture_filter: TextureFilter
}

impl Default for FontParams {
    /// Those values are pretty arbitrary, but should be good for common usecases.
    fn default() -> Self {
        Self {
            min_size: 6.0,
            max_size: 48.0,
            integer_size: true,
            texture_filter: TextureFilter::Bilinear
        }
    }
}

/// A cache for [`TrueTypeFont`] glyphs, rendered at various sizes.
pub struct TrueTypeFontCache {
    font: fontdue::Font,
    params: FontParams,
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
        let mut size = size.clamp(self.params.min_size, self.params.max_size);
        if self.params.integer_size {
            size = size.round();
        }

        let key = FontSizeKey(size);
        let mut atlases = self.atlases.borrow_mut();

        let atlas = if let Some(atlas) = atlases.get_mut(&key) {
            atlas
        } else {
            let atlas = TrueTypeFontAtlas::new(rl, &self.font, self.params.texture_filter);
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
    fn new(rl: &Raylib, font: &fontdue::Font, texture_filter: TextureFilter) -> Self {
        let mut texture = Texture::load_empty(rl, 128, 128, PixelFormat::UncompressedGrayAlpha).unwrap();
        texture.set_texture_filter(texture_filter);
        Self {
            texture,
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
