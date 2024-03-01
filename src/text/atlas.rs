use std::num::NonZeroU16;
use hashbrown::HashMap;

use crate::{prelude::{DrawHandle, Vector2, Color, Texture, Rectangle}, ffi};

// TODO: Support vertical text

#[derive(Debug, Clone, Copy, Default)]
pub struct Metrics {
    /// The offset relative to the left border of the font. Can be negative.
    pub xmin: f32,
    /// The offset relative to the baseline of the font. Can be negative.
    pub ymin: f32,
    /// The width of the glyph.
    pub width: f32,
    /// The height of the glyph.
    pub height: f32,
    /// How far another character should be positioned after this one.
    pub advance_width: f32
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LineMetrics {
    /// The highest point that any glyph in the font extends to above the baseline.
    pub ascent: f32,
    /// The lowest point that any glyph in the font extends to below the baseline.
    pub descent: f32,
    /// The gap between the descent of one line and the ascent of the next.
    pub line_gap: f32
}

/// An atlas is a collection of glyphs rendered to a texture at a specific size.
/// This trait defines everything needed by the text module to draw and layout the glyphs to the screen.
/// 
/// To create a `FontAtlas`, you can either load a true type font and render it at specific size using `rl.atlas_font()`,
/// or you can load a bitmap font atlas directly.
/// 
/// You can implement this trait yourself if you which to use another font backend or create one yourself.
pub trait FontAtlas {
    /// Lists all available codepoints in the font.
    fn codepoints(&self) -> &HashMap<char, NonZeroU16>;
    /// Returns the index corresponding to that codepoint in the font, or 0 if it is not present.
    fn glyph_index(&self, codepoint: char) -> u16 {
        self.codepoints().get(&codepoint).copied().map(u16::from).unwrap_or_default()
    }
    /// Returns the number of glyphs in the font.
    /// Valid indices are always contained in `0..self.glyph_count()`.
    fn glyph_count(&self) -> u16;

    /// The metrics specifying how glyphs should be placed relative to the font baseline,
    /// and how to space lines apart.
    fn line_metrics(&self) -> Option<LineMetrics>;
    /// Additional (or inverse) spacing between two characters specified by the font.
    /// Returns `None` if there was no information about it.
    fn kern_indexed(&self, left: u16, right: u16) -> Option<f32>;
    /// Information about the size of a specific glyph.
    fn metrics_indexed(&self, index: u16) -> Metrics;

    /// The size at which the atlas was rendered
    fn size(&self) -> f32;
    /// The texture used by the font atlas.
    fn texture(&self) -> &Texture;
    /// Get the rectangle in the texture associated to the given glyph index.
    fn get_glyph(&mut self, index: u16) -> Rectangle;
}

impl<P> DrawHandle<'_, P> {
    /// Returns the width and height occupied by the given text in the given font.
    pub fn measure_text<F: FontAtlas>(&mut self, atlas: &mut F, text: &str) -> Vector2 {
        let mut size = Vector2::ZERO;
        let mut previous = None;
        for char in text.chars() {
            let glyph_index = atlas.glyph_index(char);
            if let Some(previous) = previous {
                size.x += atlas.kern_indexed(previous, glyph_index).unwrap_or_default();
            }
            size.x += atlas.metrics_indexed(glyph_index).advance_width;
            previous = Some(glyph_index);
        }
        size
    }

    /// Draws some text at the specified location using the given font.
    /// Returns the coordinates of the last characters 
    pub fn text<F: FontAtlas>(&mut self, atlas: &mut F, text: &str, mut pos: Vector2, color: Color) -> Vector2 {
        let mut previous = None;
        for char in text.chars() {
            let glyph_index = atlas.glyph_index(char);
            if let Some(previous) = previous {
                pos.x += atlas.kern_indexed(previous, glyph_index).unwrap_or_default();
            }
            self.glyph(atlas, glyph_index, pos, color);
            pos.x += atlas.metrics_indexed(glyph_index).advance_width;
            previous = Some(glyph_index);
        }
        pos
    }

    /// Draws a single character at the specified location.
    pub fn codepoint<F: FontAtlas>(&mut self, atlas: &mut F, codepoint: char, pos: Vector2, color: Color) {
        let glyph_index = atlas.glyph_index(codepoint);
        self.glyph(atlas, glyph_index, pos, color);
    }

    /// Draw a glyph of the given font.
    /// Caches the glyph if it wasn't previously rendered.
    #[inline]
    pub fn glyph<F: FontAtlas>(&mut self, atlas: &mut F, glyph_index: u16, mut pos: Vector2, color: Color) {
        let rec = atlas.get_glyph(glyph_index);
        let metrics = atlas.metrics_indexed(glyph_index);
        pos.y -= metrics.height + metrics.ymin;
        pos.x = pos.x.floor();
        pos.y = pos.y.floor();
        self.texture_rec(atlas.texture(), rec, pos, color);
    }

    pub fn fps(&mut self, pos: Vector2) {
        unsafe { ffi::DrawFPS(pos.x as i32, pos.y as i32) }
    }
}
