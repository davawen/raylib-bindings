use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(1800, 1100, "Rust text!", 60);
    set_window_state(rl, WindowFlags::RESIZABLE);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    let font = load_font_ex(rl, font);

    let mut scroll = 0;
    let mut size = 20.0;

    while !window_should_close(rl) {
        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);

            if is_key_down(rl, Key::LeftShift) {
                size += get_mouse_wheel_move(rl);
                size = size.max(1.0);
            } else {
                scroll += get_mouse_wheel_move(rl) as i32;
            }

            let left_size = measure_text(&font, "0x0000", size).x;

            let offset = size*1.5;

            let (w, h) = (get_render_width(rl) as i32, get_render_height(rl) as i32);
            let (nw, nh) = ((w - 50 - left_size as i32 - 25)/(offset as i32), (h-100)/(offset as i32));

            for i in 0..font.glyph_count() as u16 {
                let x = i as i32 % nw;
                let y = (i as i32 / nw) + scroll;

                if y < 0 { continue }
                if y >= nh { break }

                if x == 0 {
                    draw_text(rl, &font, &format!("0x{:04x}", i), vec2(5.0, y as f32*offset + 50.0), size, Color::BLACK);
                }
                draw_glyph(rl, &font, i, vec2(x as f32*offset + left_size + 25.0, y as f32*offset + 50.0), size, Color::BLACK);
            }

            draw_fps(rl, vec2(5.0, 5.0));
        });
    }
}
