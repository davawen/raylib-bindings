use std::mem::ManuallyDrop;

use hashbrown::HashMap;

use crate::{text::bitmap::{BitmapFontAtlas, BitmapGlyph}, prelude::{Color, PixelFormat, Rectangle, Metrics, LineMetrics, Image}};

use super::Raylib;

impl Raylib {
    /// Taken from https://github.com/raysan5/raylib/blob/dc7f81a7b0cbc666812c870841313e0c15a87a0c/src/rtext.c#L220
    pub(super) fn load_default_font(&mut self) {
        const DATA: &[u8; 2048] = include_bytes!("default_font_data");
        const CHARS_WIDTH: [u8; 224] = [
            3, 1, 4, 6, 5, 7, 6, 2, 3, 3, 5, 5, 2, 4, 1, 7, 5, 2, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 3, 4, 3, 6,
            7, 6, 6, 6, 6, 6, 6, 6, 6, 3, 5, 6, 5, 7, 6, 6, 6, 6, 6, 6, 7, 6, 7, 7, 6, 6, 6, 2, 7, 2, 3, 5,
            2, 5, 5, 5, 5, 5, 4, 5, 5, 1, 2, 5, 2, 5, 5, 5, 5, 5, 5, 5, 4, 5, 5, 5, 5, 5, 5, 3, 1, 3, 4, 4,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 5, 5, 5, 7, 1, 5, 3, 7, 3, 5, 4, 1, 7, 4, 3, 5, 3, 3, 2, 5, 6, 1, 2, 2, 3, 5, 6, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 3, 3, 3, 3, 7, 6, 6, 6, 6, 6, 6, 5, 6, 6, 6, 6, 6, 6, 4, 6,
            5, 5, 5, 5, 5, 5, 9, 5, 5, 5, 5, 5, 2, 2, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5
        ];
        const HEIGHT: u32 = 10;
        const PADDING: u32 = 1;

        let mut image = Image::gen_color(self, 128, 128, Color::BLANK);
        image.convert_format(PixelFormat::UncompressedGrayAlpha);
        let num_pixels = image.width() as usize * image.height() as usize;
        let data = image.data_mut();
        for (count, i) in (0..num_pixels).step_by(8).enumerate() {
            for j in (0..8).rev() {
                if (DATA[count] >> j) & 0b1 == 0b1 {
                    let idx = (i+j)*2;
                    data[idx] = 0xff;
                    data[idx+1] = 0xff;
                }
            }
        }

        let glyph_count: u16 = 224;
        let mut glyphs = Vec::with_capacity(glyph_count as usize);
        let mut codepoints = HashMap::new();

        // reserve space for unknown glyph
        glyphs.push(BitmapGlyph { rec: Rectangle::new(0.0, 0.0, 0.0, 0.0), metrics: Metrics::default() });

        let mut offset_x = PADDING;
        let mut offset_y = PADDING;
        for i in 0..glyph_count {
            let width = CHARS_WIDTH[i as usize] as u32;

            if offset_x + width + PADDING >= image.width() {
                offset_x = PADDING;
                offset_y += HEIGHT + PADDING;
            }

            let rec = Rectangle::new(offset_x as f32, offset_y as f32, width as f32, HEIGHT as f32);
            let metrics = Metrics {
                xmin: 0.0, ymin: 0.0, width: width as f32, height: HEIGHT as f32, advance_width: (width + PADDING) as f32
            };

            glyphs.push(BitmapGlyph { metrics, rec });
            codepoints.insert(char::from_u32(i as u32 + 32).unwrap(), (i+1).try_into().unwrap());

            offset_x += width + PADDING;
        }

        // set unknown character to '?' glyph
        glyphs[0] = glyphs[u16::from(codepoints[&'?']) as usize];

        let line_metrics = LineMetrics {
            descent: 0.0,
            ascent: 10.0,
            line_gap: 15.0
        };

        let atlas = BitmapFontAtlas::load(self, &image, codepoints, glyphs, line_metrics, HEIGHT as f32);
        self.default_font = ManuallyDrop::new(Some(atlas));
    }

    pub fn default_font(&self) -> &BitmapFontAtlas {
        self.default_font.as_ref().unwrap()
    }
}
