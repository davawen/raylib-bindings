use std::num::NonZeroU16;
use hashbrown::HashMap;

use crate::{ffi, prelude::{vec2, Color, DrawHandle, Rectangle, Vector2}};

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

impl Metrics {
    pub fn scaled(self, s: f32) -> Self {
        Self {
            xmin: self.xmin * s, ymin: self.ymin * s,
            width: self.width * s, height: self.height * s,
            advance_width: self.advance_width * s
        }
    }
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

impl LineMetrics {
    pub fn scaled(self, s: f32) -> Self {
        Self {
            ascent: self.ascent * s,
            descent: self.descent * s,
            line_gap: self.line_gap * s
        }
    }
}

/// An atlas is a collection of glyphs rendered to a texture at a specific size.
///
/// A font cache contains the necessary data to create atlases for a font.
/// This trait defines everything needed by the text module to draw and layout the glyphs to the screen.
/// 
/// To create a [`FontCache`], you can either:
/// - load a [`TrueTypeFont`][`super::font::TrueTypeFont`] directly using [`load_font`][`super::font::load_font`]
/// - create a [`TrueTypeFont`][`super::font::TrueTypeFont`] and load it using [`load_font_ex`][`super::font::load_font_ex`],
/// - create a [`BitmapFontAtlas`][`super::bitmap::BitmapFontAtlas`] directly.
/// 
/// You can implement this trait yourself if you which to use another font backend or create one yourself.
pub trait FontCache {
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
    fn line_metrics(&self, size: f32) -> Option<LineMetrics>;
    /// Additional (or inverse) spacing between two characters specified by the font.
    /// Returns `None` if there was no information about it.
    fn kern_indexed(&self, left: u16, right: u16, size: f32) -> Option<f32>;
    /// Information about the size of a specific glyph.
    fn metrics_indexed(&self, index: u16, size: f32) -> Metrics;

    /// Draws the given glyph to the specified place.
    fn draw_glyph(&self, rl: &DrawHandle, index: u16, size: f32, dest: Rectangle, color: Color);
}

/// Returns the width and height occupied by the given text in the given font, drawn at the given size.
pub fn measure_text<F: FontCache>(cache: &F, text: &str, size: f32) -> Vector2 {
    let mut pos = Vector2::ZERO;
    let mut previous = None;
    for char in text.chars() {
        let glyph_index = cache.glyph_index(char);
        if let Some(previous) = previous {
            pos.x += cache.kern_indexed(previous, glyph_index, size).unwrap_or_default();
        }
        pos.x += cache.metrics_indexed(glyph_index, size).advance_width;
        previous = Some(glyph_index);
    }
    pos
}

/// Draws some text at the specified location using the given font at the given size.
/// 
/// Using a [`TrueTypeFontAtlas`](super::font::TrueTypeFontAtlas) or a [`BitmapFontAtlas`](super::bitmap::BitmapFontAtlas), if the given size is different than the original render size,
/// the glyph will be scaled using interpolation.
/// For best text quality, prefer creating the font atlas at the same size that will be used for drawing.
/// 
/// Returns the coordinates of the last characters .
pub fn draw_text<F: FontCache>(rl: &DrawHandle, cache: &F, text: &str, mut pos: Vector2, size: f32, color: Color) -> Vector2 {
    let mut previous = None;
    for char in text.chars() {
        let glyph_index = cache.glyph_index(char);
        if let Some(previous) = previous {
            pos.x += cache.kern_indexed(previous, glyph_index, size).unwrap_or_default();
        }
        draw_glyph(rl, cache, glyph_index, pos, size, color);
        pos.x += cache.metrics_indexed(glyph_index, size).advance_width;
        previous = Some(glyph_index);
    }
    pos
}

/// Draws a single character at the specified location.
pub fn draw_codepoint<F: FontCache>(rl: &DrawHandle, atlas: &F, codepoint: char, pos: Vector2, size: f32, color: Color) {
    let glyph_index = atlas.glyph_index(codepoint);
    draw_glyph(rl, atlas, glyph_index, pos, size, color);
}

/// Draw a glyph of the given font.
/// Caches the glyph if it wasn't previously rendered.
#[inline]
pub fn draw_glyph<F: FontCache>(rl: &DrawHandle, atlas: &F, glyph_index: u16, pos: Vector2, size: f32, color: Color) {
    let metrics = atlas.metrics_indexed(glyph_index, size);
    let line = atlas.line_metrics(size).unwrap_or_default();
    let pos = vec2((pos.x + metrics.xmin).floor(), (pos.y - metrics.ymin - metrics.height + line.ascent).floor());
    let dest = Rectangle::new(
        pos.x, pos.y,
        metrics.width, metrics.height
    );

    atlas.draw_glyph(rl, glyph_index, size, dest, color);
}

pub fn draw_fps(_rl: &DrawHandle, pos: Vector2) {
    unsafe { ffi::DrawFPS(pos.x as i32, pos.y as i32) }
}
