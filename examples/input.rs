use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Test input", 60);
    let mut s = String::new();

    let mut cursor_pos = 0;

    while !rl.window_should_close() {
        while let Some(c) = rl.get_char_pressed() {
            s.insert(cursor_pos, c);
            cursor_pos += c.len_utf8();
        }
        while let Some(k) = rl.get_key_pressed() {
            match k {
                KeyboardKey::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        while cursor_pos > 0 && !s.is_char_boundary(cursor_pos) {
                            cursor_pos -= 1;
                        }
                        s.remove(cursor_pos);
                    }
                },
                KeyboardKey::Left => if cursor_pos > 0 && cursor_pos <= s.len() {
                    cursor_pos -= 1;
                    while cursor_pos > 0 && !s.is_char_boundary(cursor_pos) {
                        cursor_pos -= 1;
                    }
                },
                KeyboardKey::Right => if cursor_pos < s.len() {
                    let c = s[cursor_pos..].chars().next().unwrap();
                    cursor_pos += c.len_utf8() 
                },
                _ => ()
            }
        }

        if rl.is_mouse_button_pressed(MouseButton::Left) {
            let mut idx = s.len();
            let mut c = std::ffi::CString::new(s.as_str()).unwrap();
            let mut bounds = unsafe { raylib::ffi::MeasureText(c.as_ptr(), 20) };
            if rl.get_mouse_x() < (bounds as f32 + 20.0) {
                while rl.get_mouse_x() < (bounds as f32 + 20.0) {
                    let mut v = c.into_bytes();
                    if v.is_empty() { break }

                    v.pop();
                    idx -= 1;
                    c = std::ffi::CString::new(v).unwrap();
                    bounds = unsafe { raylib::ffi::MeasureText(c.as_ptr(), 20) };
                }
                cursor_pos = idx;
            }
        }

        rl.begin_drawing(|rl, draw| {
            draw.clear_background(Color::RAYWHITE);

            let c = std::ffi::CString::new(s.as_str()).unwrap();
            unsafe { raylib::ffi::DrawText(c.as_ptr(), 20, 20, 20, Color::BLACK) };
            let mut c = c.into_bytes();
            c.truncate(cursor_pos);
            let c = std::ffi::CString::new(c).unwrap();

            let bounds = unsafe { raylib::ffi::MeasureText(c.as_ptr(), 20) };

            let time = rl.get_time();
            let color = ((time*2.0*std::f32::consts::PI).sin()*128.0 + 128.0) as u8;
            let color = Color::rgba(0, 0, 0, color);

            unsafe { raylib::ffi::DrawRectangle(22 + bounds, 20, 2, 20, color) };
        });
    }
}
