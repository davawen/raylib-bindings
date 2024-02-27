//! Pure rust reimplementation of a text module.

use ffi::{PixelFormat, Rectangle};
use rusttype::{PositionedGlyph, GlyphId};

use crate::{prelude::{DrawHandle, Vector2, Color, Texture, Raylib}, ffi};

#[derive(Debug, Clone)]
pub struct Font<'f>(rusttype::Font<'f>);

impl<'f> Font<'f> {
    pub fn new(font: rusttype::Font<'f>) -> Self {
        Font(font)
    }
}

impl<'a> std::ops::Deref for Font<'a> {
    type Target = rusttype::Font<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FontAtlas<'f> {
    texture: Texture,
    vmetrics: rusttype::VMetrics,
    scale: rusttype::Scale,
    pub recs: Vec<(bool, Rectangle)>,
    font: &'f Font<'f>
}

impl Raylib {
    /// Creates a texture to hold the rendered font
    pub fn atlas_font<'a, 'f>(&'a mut self, font: &'f Font<'f>, size: f32) -> FontAtlas<'f> {
        let scale = rusttype::Scale::uniform(size);
        let vmetrics = font.0.v_metrics(scale);
        let recs = vec![];
        let texture = self.load_texture_empty(512, 512, PixelFormat::UncompressedGrayAlpha).unwrap();
        let mut empty = FontAtlas { texture, vmetrics, scale, recs, font };
        empty.reatlas(self, size);
        empty
    }
}

impl FontAtlas<'_> {
    pub fn render_glyph(&mut self, glyph: GlyphId) {
        let (rendered, rec) = &mut self.recs[glyph.0 as usize];
        // skip if glyph was already rendered
        if *rendered { return }

        let glyph = self.font.0.glyph(glyph).scaled(self.scale).positioned(rusttype::point(0.0, 0.0));
        let bounds = glyph.pixel_bounding_box().unwrap(); // glyph without bounds are already rendered
        let w = bounds.width() as usize;
        let h = bounds.height() as usize;
        let size = w*h;
        let mut data: Vec<u8> = vec![0; size*2];
        glyph.draw(|x, y, v| {
            let idx = y as usize * w + x as usize;
            data[idx*2] = 255;
            data[idx*2 + 1] = (v*255.0) as u8;
        });

        self.texture.update_rec_raw(*rec, &data).unwrap();
    }

    /// Recalculate texture size for a new font size.
    /// This re-uses the already created texture if it is big enough. 
    pub fn reatlas(&mut self, rl: &mut Raylib, size: f32) {
        self.scale = rusttype::Scale::uniform(size);
        self.vmetrics = self.font.v_metrics(self.scale);
        self.recs.clear();

        let mut size = self.texture.width();

        let mut offset_x = 0;
        let mut offset_y = 0;

        let mut min_x = 0;
        let mut max_y = 0;

        let mut pot_y = size;

        for idx in 0..self.font.glyph_count() {
            let glyph = self.font.glyph(rusttype::GlyphId(idx as u16)).scaled(self.scale).positioned(rusttype::point(0.0, 0.0));
            
            let Some(bounds) = glyph.pixel_bounding_box() else { 
                self.recs.push((true, Rectangle::new(0.0, 0.0, 1.0, 1.0)));
                continue
            };

            // went over the right-most edge, go back to the start of the line
            if offset_x + bounds.width() as u32 >= size {
                offset_x = min_x;
                offset_y = max_y;

                // went to the bottom of the screen, resize and use the new part to the right
                if offset_y + bounds.height() as u32 >= size {
                    min_x = size;
                    max_y = 0;
                    pot_y = size;

                    offset_x = size;
                    offset_y = 0;

                    size *= 2;
                }

                // finished using the new right part, use lower half of resized
                if offset_y + bounds.height() as u32 >= pot_y {
                    offset_x = 0;
                    offset_y = pot_y;

                    min_x = 0;
                    max_y = pot_y;
                    pot_y = size;
                }

            }

            self.recs.push((false, Rectangle::new(offset_x as f32, offset_y as f32, bounds.width() as f32, bounds.height() as f32)));

            offset_x += bounds.width() as u32;
            max_y = max_y.max(offset_y + bounds.height() as u32);
        }

        if size > self.texture.width() {
            self.texture = rl.load_texture_empty(size, size, PixelFormat::UncompressedGrayAlpha).unwrap();
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Size at which the font was rendered
    pub fn size(&self) -> f32 {
        self.scale.y
    }
}

impl<P> DrawHandle<'_, P> {
    pub fn text(&mut self, atlas: &mut FontAtlas<'_>, text: &str, pos: Vector2, color: Color) {
        let metrics = atlas.vmetrics;
        let layout: Vec<_> = atlas.font.layout(text, atlas.scale, rusttype::point(pos.x, pos.y + metrics.ascent)).collect();
        for glyph in layout {
            self.glyph(atlas, glyph, color);
        }
    }

    pub fn codepoint(&mut self, atlas: &mut FontAtlas<'_>, codepoint: char, pos: Vector2, color: Color) {
        let glyph = atlas.font.glyph(codepoint).scaled(atlas.scale).positioned(rusttype::point(pos.x, pos.y + atlas.vmetrics.ascent));
        self.glyph(atlas, glyph, color);
    }

    pub fn glyph_id(&mut self, atlas: &mut FontAtlas<'_>, id: GlyphId, pos: Vector2, color: Color) {
        let glyph = atlas.font.glyph(id).scaled(atlas.scale).positioned(rusttype::point(pos.x, pos.y + atlas.vmetrics.ascent));
        self.glyph(atlas, glyph, color);
    }

    /// Draw a glyph of the given font.
    /// Caches the glyph if it wasn't previously rendered.
    #[inline]
    pub fn glyph<'f>(&mut self, atlas: &mut FontAtlas<'f>, glyph: PositionedGlyph<'f>, color: Color) {
        atlas.render_glyph(glyph.id());
        let (_, rec) = atlas.recs[glyph.id().0 as usize];
        let bounds = glyph.pixel_bounding_box().unwrap_or_default();
        self.texture_rec(&atlas.texture, rec, Vector2::new(bounds.min.x as f32, bounds.min.y as f32), color);
    }

    pub fn fps(&mut self, pos: Vector2) {
        unsafe { ffi::DrawFPS(pos.x as i32, pos.y as i32) }
    }
}
