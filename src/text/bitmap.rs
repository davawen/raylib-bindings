
// /// A font possessing the already rasterized glyphs.
// pub struct BitmapFont {
//     /// Buffer containing the bitmap information of every glyph (UncompressedGrayAlpha)
//     data: Vec<u8>,
//     /// Vector mapping glyph index to a width, a height
//     glyphs: Vec<BitmapGlyph>,
//     /// Available glyphs
//     codepoints: HashMap<char, NonZeroU16>
// }

use std::num::NonZeroU16;

use hashbrown::HashMap;

use crate::prelude::{draw_texture_pro, Color, DrawHandle, Image, Raylib, Rectangle, Texture, Vector2};

use super::cache::{Metrics, LineMetrics, FontCache};

pub struct BitmapFontAtlas {
    texture: Texture,
    codepoints: HashMap<char, NonZeroU16>,
    glyph_count: u16,
    /// A map from glyph index to rectangle
    glyphs: Vec<BitmapGlyph>,
    line_metrics: LineMetrics,
    /// Pixel height of the font
    size: f32
}

#[derive(Debug, Clone, Copy)]
pub struct BitmapGlyph {
    /// Rectangle where it is in the texture
    pub rec: Rectangle,
    pub metrics: Metrics
}

impl BitmapFontAtlas {
    pub fn load(rl: &mut Raylib, image: &Image, codepoints: HashMap<char, NonZeroU16>, glyphs: Vec<BitmapGlyph>, line_metrics: LineMetrics, size: f32) -> BitmapFontAtlas {
        let texture = Texture::load_from_image(rl, image).unwrap();
        BitmapFontAtlas {
            texture, codepoints, glyph_count: glyphs.len() as u16, glyphs, line_metrics, size
        }
    }
}

impl FontCache for BitmapFontAtlas {
    fn codepoints(&self) -> &HashMap<char, NonZeroU16> { &self.codepoints }
    fn glyph_count(&self) -> u16 { self.glyph_count }
    fn line_metrics(&self, size: f32) -> Option<LineMetrics> {
        Some(self.line_metrics.scaled(size/self.size))
    }

    fn metrics_indexed(&self, index: u16, size: f32) -> Metrics {
        self.glyphs[index as usize].metrics.scaled(size/self.size)
    }
    fn kern_indexed(&self, _left: u16, _right: u16, _size: f32) -> Option<f32> { None }

    fn draw_glyph(&self, rl: &DrawHandle, index: u16, _: f32, dest: Rectangle, color: Color) {
        draw_texture_pro(rl, &self.texture, self.glyphs[index as usize].rec, dest, Vector2::ZERO, 0.0, color);
    }
}
