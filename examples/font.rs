use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(1800, 1100, "Rust text!", 60);
    rl.set_window_state(ConfigFlags::FLAG_WINDOW_RESIZABLE);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    // let font = TrueTypeFont::from_bytes(include_bytes!("../assets/TerminusTTF.ttf").as_slice()).unwrap();
    let mut rendered = font.atlas(&mut rl, 20.0);

    let mut scroll = 0;
    let mut size = 20.0;

    while !rl.window_should_close() {
        rl.begin_drawing(|rl| {
            rl.clear_background(Color::RAYWHITE);

            if rl.is_key_down(KeyboardKey::LeftShift) {
                size += rl.get_mouse_wheel_move();
                size = size.max(1.0);
                if size != rendered.size() {
                    rendered.reatlas(rl, size);
                }
            } else {
                scroll += rl.get_mouse_wheel_move() as i32;
            }

            if rl.is_key_down(KeyboardKey::A) {
                rl.texture(rendered.texture(), 0.0, 0.0, Color::BLACK);
                return
            }

            let left_size = rl.measure_text(&mut rendered, "0x0000", size).x;

            let offset = size*1.5;

            let (w, h) = (rl.get_render_width() as i32, rl.get_render_height() as i32);
            let (nw, nh) = ((w - 50 - left_size as i32 - 25)/(offset as i32), (h-100)/(offset as i32));

            for i in 0..rendered.glyph_count() as u16 {
                let x = i as i32 % nw;
                let y = (i as i32 / nw) + scroll;

                if y < 0 { continue }
                if y >= nh { break }

                if x == 0 {
                    rl.text(&mut rendered, &format!("0x{:04x}", i), vec2(5.0, y as f32*offset + 50.0), size, Color::BLACK);
                }
                rl.glyph(&mut rendered, i, vec2(x as f32*offset + left_size + 25.0, y as f32*offset + 50.0), size, Color::BLACK);
            }

            rl.fps(vec2(5.0, 5.0));
        });
    }
}
