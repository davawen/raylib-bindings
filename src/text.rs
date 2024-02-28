//! Pure rust reimplementation of a text module.

use ffi::{PixelFormat, Rectangle};

use crate::{prelude::{DrawHandle, Vector2, Color, Texture, Raylib}, ffi};

#[derive(Debug, Clone)]
pub struct Font(fontdue::Font);

impl Font {
    pub fn new(font: fontdue::Font) -> Self {
        Font(font)
    }
}

impl std::ops::Deref for Font {
    type Target = fontdue::Font;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FontAtlas<'f> {
    texture: Texture,
    size: f32,
    pub recs: Vec<(bool, Rectangle)>,
    font: &'f Font
}

impl Raylib {
    /// Creates a texture to hold the rendered font
    pub fn atlas_font<'a, 'f>(&'a mut self, font: &'f Font, size: f32) -> FontAtlas<'f> {
        let recs = vec![];
        let texture = self.load_texture_empty(512, 512, PixelFormat::UncompressedGrayAlpha).unwrap();
        let mut empty = FontAtlas { texture, size, recs, font };
        empty.reatlas(self, size);
        empty
    }
}

impl FontAtlas<'_> {
    pub fn render_glyph(&mut self, glyph_index: u16) {
        let (rendered, rec) = &mut self.recs[glyph_index as usize];
        // skip if glyph was already rendered
        if *rendered { return }

        let (_, rasterized) = self.font.rasterize_indexed(glyph_index, self.size);

        // convert to GrayAlpha
        let mut data = Vec::with_capacity(rasterized.len()*2);
        for alpha in rasterized {
            data.push(255);
            data.push(alpha);
        }

        self.texture.update_rec_raw(*rec, &data).unwrap();
        *rendered = true;
    }

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

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Size at which the font was rendered
    pub fn size(&self) -> f32 {
        self.size
    }
}

impl<P> DrawHandle<'_, P> {
    pub fn text(&mut self, atlas: &mut FontAtlas<'_>, text: &str, mut pos: Vector2, color: Color) {
        let mut previous = None;
        for char in text.chars() {
            let glyph_index = atlas.font.lookup_glyph_index(char);
            if let Some(previous) = previous {
                pos.x += atlas.font.horizontal_kern_indexed(previous, glyph_index, atlas.size).unwrap_or_default();
            }
            self.glyph(atlas, glyph_index, pos, color);
            pos.x += atlas.font.metrics_indexed(glyph_index, atlas.size).advance_width;
            previous = Some(glyph_index);
        }
    }

    pub fn codepoint(&mut self, atlas: &mut FontAtlas<'_>, codepoint: char, pos: Vector2, color: Color) {
        let glyph_index = atlas.font.lookup_glyph_index(codepoint);
        self.glyph(atlas, glyph_index, pos, color);
    }

    /// Draw a glyph of the given font.
    /// Caches the glyph if it wasn't previously rendered.
    #[inline]
    pub fn glyph<'f>(&mut self, atlas: &mut FontAtlas<'f>, glyph_index: u16, mut pos: Vector2, color: Color) {
        atlas.render_glyph(glyph_index);
        let metrics = atlas.font.metrics_indexed(glyph_index, atlas.size);
        pos.y += (-metrics.bounds.height - metrics.bounds.ymin).floor();
        let (_, rec) = atlas.recs[glyph_index as usize];
        self.texture_rec(&atlas.texture, rec, pos, color);
    }

    pub fn fps(&mut self, pos: Vector2) {
        unsafe { ffi::DrawFPS(pos.x as i32, pos.y as i32) }
    }
}
