//! Pure rust reimplementation of a text module.

use ffi::{PixelFormat, Rectangle};
use rusttype::PositionedGlyph;

use crate::{prelude::{DrawHandle, Vector2, Color, Texture, Raylib}, ffi};

#[derive(Debug, Clone)]
pub struct Font<'f>(rusttype::Font<'f>);

impl<'f> Font<'f> {
    pub fn new(font: rusttype::Font<'f>) -> Self {
        Font(font)
    }
}

pub struct RenderedFont<'f> {
    texture: Texture,
    vmetrics: rusttype::VMetrics,
    scale: rusttype::Scale,
    pub recs: Vec<Rectangle>,
    font: &'f Font<'f>
}

impl Raylib {
    /// Renders every glyph in a font to a texture at a certain size
    pub fn render_font<'a, 'f>(&'a mut self, font: &'f Font<'f>, size: f32) -> RenderedFont<'f> {
        let scale = rusttype::Scale::uniform(size);

        let mut size = 256;
        let mut image = self.gen_image_color(size, size, Color::BLANK);
        self.image_format(&mut image, PixelFormat::UncompressedGrayAlpha);

        let mut recs = vec![];

        let mut offset_x = 0;
        let mut offset_y = 0;

        let mut min_x = 0;
        let mut max_y = 0;

        let mut pot_y = size;

        for idx in 0..font.0.glyph_count() {
            let glyph = font.0.glyph(rusttype::GlyphId(idx as u16)).scaled(scale).positioned(rusttype::point(0.0, 0.0));
            
            let Some(bounds) = glyph.pixel_bounding_box() else { 
                recs.push(Rectangle::new(0.0, 0.0, 1.0, 1.0));
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
                    self.image_resize_canvas(&mut image, size, size, 0, 0, Color::BLANK);
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

            glyph.draw(|x, y, v| {
                let x = (x + offset_x) as f32;
                let y = (y + offset_y) as f32;
                image.draw_pixel(x, y, Color::graya(255, (v*255.0) as u8));
            });

            recs.push(Rectangle::new(offset_x as f32, offset_y as f32, bounds.width() as f32, bounds.height() as f32));

            offset_x += bounds.width() as u32;
            max_y = max_y.max(offset_y + bounds.height() as u32);

        }

        let texture = self.load_texture_from_image(&image).unwrap();

        let vmetrics = font.0.v_metrics(scale);

        RenderedFont {
            texture, vmetrics, scale, recs, font
        }
    }
}

impl RenderedFont<'_> {
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Size at which the font was rendered
    pub fn size(&self) -> f32 {
        self.scale.y
    }
}

impl<P> DrawHandle<'_, P> {
    pub fn text(&mut self, font: &RenderedFont<'_>, text: &str, pos: Vector2, color: Color) {
        let metrics = font.vmetrics;
        let layout: Vec<_> = font.font.0.layout(text, font.scale, rusttype::point(pos.x, pos.y + metrics.ascent)).collect();
        for glyph in layout {
            self.glyph(font, glyph, color);
        }
    }
    //
    // pub fn codepoint(&mut self, font: &mut Font<'_>, rl: &mut Raylib, codepoint: char, pos: Vector2, size: f32, color: Color) {
    //     let scale = rusttype::Scale::uniform(size.abs());
    //     let metrics = font.font.v_metrics(scale);
    //     let glyph = font.font.glyph(codepoint).scaled(scale).positioned(rusttype::point(pos.x, pos.y + metrics.ascent));
    //     self.glyph(font, rl, glyph, color);
    // }
    //
    #[inline]
    pub fn glyph<'f>(&mut self, font: &RenderedFont<'f>, glyph: PositionedGlyph<'f>, color: Color) {
        let rec = font.recs[glyph.id().0 as usize];
        let bounds = glyph.pixel_bounding_box().unwrap_or_default();
        self.texture_rec(&font.texture, rec, Vector2::new(bounds.min.x as f32, bounds.min.y as f32), color);
    }

    pub fn fps(&mut self, pos: Vector2) {
        unsafe { ffi::DrawFPS(pos.x as i32, pos.y as i32) }
    }
}
