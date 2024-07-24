use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Test input", 60);
    let mut s = String::new();

    let mut cursor_pos = 0;

    while !window_should_close(rl) {
        while let Some(c) = get_char_pressed(rl) {
            s.insert(cursor_pos, c);
            cursor_pos += c.len_utf8();
        }
        while let Some(k) = get_key_pressed(rl) {
            match k {
                Key::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        while cursor_pos > 0 && !s.is_char_boundary(cursor_pos) {
                            cursor_pos -= 1;
                        }
                        s.remove(cursor_pos);
                    }
                },
                Key::Left => if cursor_pos > 0 && cursor_pos <= s.len() {
                    cursor_pos -= 1;
                    while cursor_pos > 0 && !s.is_char_boundary(cursor_pos) {
                        cursor_pos -= 1;
                    }
                },
                Key::Right => if cursor_pos < s.len() {
                    let c = s[cursor_pos..].chars().next().unwrap();
                    cursor_pos += c.len_utf8() 
                },
                _ => ()
            }
        }

        if is_mouse_button_pressed(rl, MouseButton::Left) {
            let mut idx = s.len();
            let mut bounds = measure_text(rl.default_font(), &s, 20.0).x;
            if get_mouse_x(rl) < (bounds as f32 + 20.0) {
                while get_mouse_x(rl) < (bounds as f32 + 20.0) {
                    if s.is_empty() { break }

                    idx -= 1;
                    while idx > 0 && !s.is_char_boundary(idx) {
                        idx -= 1;
                    }

                    bounds = measure_text(rl.default_font(), &s[..idx], 20.0).x;
                }
                cursor_pos = idx;
            }
        }

        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);

            draw_text(rl, rl.default_font(), &s, Vector2::splat(20.0), 20.0, Color::BLACK);

            let bounds = measure_text(rl.default_font(), &s[..cursor_pos], 20.0).x;

            let time = get_time(rl);
            let color = ((time*2.0*std::f32::consts::PI).sin()*128.0 + 128.0) as u8;
            let color = Color::rgba(0, 0, 0, color);

            draw_rectangle(rl, 18.0 + bounds, 20.0, 2.0, 20.0, color);
        });
    }
}
