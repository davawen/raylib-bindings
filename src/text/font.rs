use std::num::NonZeroU16;
use hashbrown::HashMap;

use crate::prelude::{Texture, Rectangle, PixelFormat, Raylib};

use super::atlas::{FontAtlas, LineMetrics, Metrics};

/// A font created from a `.tff` or `.otf` file.
pub struct TrueTypeFont(fontdue::Font);

impl TrueTypeFont {
    /// Create a font from binary `ttf` or `otf` data.
    /// # Examples
    /// Load a font present in the project directory and draw text with it:
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(800, 800, "Font rendering", 60);
    /// let font = TrueTypeFont::from_bytes(include_bytes!("../../assets/TerminusTTF.ttf").as_slice()).unwrap();
    /// let mut atlas = rl.atlas_font(&font, 32.0);
    /// while !rl.window_should_close() {
    ///     let mut draw = rl.begin_drawing();
    ///     draw.clear_background(Color::RAYWHITE);
    ///     draw.text(&mut atlas, "Hello, Terminus!", Vector2::new(20.0, 20.0), Color::BLACK);
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

pub struct TrueTypeFontAtlas<'f> {
    texture: Texture,
    size: f32,
    recs: Vec<(bool, Rectangle)>,
    font: &'f fontdue::Font
}

impl Raylib {
    /// Creates a texture to hold the rendered font
    pub fn atlas_font<'a, 'f>(&'a mut self, font: &'f TrueTypeFont, size: f32) -> TrueTypeFontAtlas<'f> {
        let recs = vec![];
        let texture = self.load_texture_empty(512, 512, PixelFormat::UncompressedGrayAlpha).unwrap();
        let mut empty = TrueTypeFontAtlas { texture, size, recs, font: font.inner() };
        empty.reatlas(self, size);
        empty
    }
}

impl TrueTypeFontAtlas<'_> {
    /// Recalculate texture size for a new font size.
    /// This re-uses the already created texture if it is big enough. 
    pub fn reatlas(&mut self, rl: &mut Raylib, size: f32) {
        self.size = size;
        self.recs.clear();

        let mut size = self.texture.width() as usize;

        let mut offset_x = 0;
        let mut offset_y = 0;

        let mut min_x = 0;
        let mut max_y = 0;

        let mut pot_y = size;

        for idx in 0..self.font.glyph_count() {
            let metrics = self.font.metrics_indexed(idx, self.size);
            
            // went over the right-most edge, go back to the start of the line
            if offset_x + metrics.width >= size {
                offset_x = min_x;
                offset_y = max_y;

                // went to the bottom of the screen, resize and use the new part to the right
                if offset_y + metrics.height >= size {
                    min_x = size;
                    max_y = 0;
                    pot_y = size;

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

            self.recs.push((false, Rectangle::new(offset_x as f32, offset_y as f32, metrics.width as f32, metrics.height as f32)));

            offset_x += metrics.width;
            max_y = max_y.max(offset_y + metrics.height);
        }

        if size as u32 > self.texture.width() {
            self.texture = rl.load_texture_empty(size as u32, size as u32, PixelFormat::UncompressedGrayAlpha).unwrap();
        }
    }

    // A reference to the font used to create this atlas.
    pub fn font(&self) -> &fontdue::Font {
        self.font
    }
}

impl FontAtlas for TrueTypeFontAtlas<'_> {
    fn codepoints(&self) -> &HashMap<char, NonZeroU16> { self.font.chars() }
    fn glyph_count(&self) -> u16 { self.font.glyph_count() }
    fn line_metrics(&self) -> Option<LineMetrics> {
        self.font.horizontal_line_metrics(self.size).map(|m| LineMetrics {
            ascent: m.ascent, descent: m.descent, line_gap: m.line_gap
        })
    }

    fn metrics_indexed(&self, index: u16) -> Metrics {
        let m = self.font.metrics_indexed(index, self.size);
        Metrics {
            xmin: m.bounds.xmin, ymin: m.bounds.ymin, width: m.bounds.width, height: m.bounds.height,
            advance_width: m.advance_width
        }
    }
    fn kern_indexed(&self, left: u16, right: u16) -> Option<f32> { self.font.horizontal_kern_indexed(left, right, self.size) }

    fn texture(&self) -> &Texture { &self.texture }
    fn size(&self) -> f32 { self.size }
    fn get_glyph(&mut self, index: u16) -> Rectangle {
        let (rendered, rec) = &mut self.recs[index as usize];
        // skip if glyph was already rendered
        if *rendered { return *rec }

        let (_, rasterized) = self.font.rasterize_indexed(index, self.size);
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
