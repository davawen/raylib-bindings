use raylib::{prelude::*, text::Font};

fn main() {
    let mut rl = Raylib::init_window(1800, 1100, "Rust text!", 60);
    rl.set_window_state(ConfigFlags::FLAG_WINDOW_RESIZABLE);

    let font = fontdue::Font::from_bytes(include_bytes!("/usr/share/fonts/TTF/iosevka-medium.ttc").as_slice(), Default::default()).unwrap();
    // let font = fontdue::Font::from_bytes(include_bytes!("/home/davawen/Downloads/symbola-font/Symbola-AjYx.ttf").as_slice(), fontdue::FontSettings::default()).unwrap();
    let font = Font::new(font);

    let mut rendered = rl.atlas_font(&font, 20.0);

    let mut scroll = 0;
    let mut size = 20.0;

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing();
        draw.clear_background(Color::RAYWHITE);

        if rl.is_key_down(KeyboardKey::LeftShift) {
            size += rl.get_mouse_wheel_move();
            size = size.max(1.0);
            if size != rendered.size() {
                rendered.reatlas(&mut rl, size);
            }
        } else {
            scroll += rl.get_mouse_wheel_move() as i32;
        }

        if rl.is_key_down(KeyboardKey::A) {
            draw.texture(rendered.texture(), 0.0, 0.0, Color::BLACK);
            continue
        }

        let offset = size*1.5;

        let (w, h) = (rl.get_render_width() as i32, rl.get_render_height() as i32);
        let (nw, nh) = ((w - 100)/(offset as i32), (h-100)/(offset as i32));

        for i in 0..font.glyph_count() as u16 {
            let x = i as i32 % nw;
            let y = (i as i32 / nw) + scroll;

            if y < 0 { continue }
            if y >= nh { break }

            if x == 0 {
                draw.text(&mut rendered, &format!("0x{:04x}", i), Vector2::new(5.0, y as f32*offset + 50.0), Color::BLACK);
            }
            draw.glyph(&mut rendered, i, Vector2::new(x as f32*offset + 80.0, y as f32*offset + 50.0), Color::BLACK);
        }

        draw.fps(Vector2::new(5.0, 5.0));
    }
}
