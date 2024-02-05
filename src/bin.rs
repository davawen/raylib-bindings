use std::ffi::CString;

use raylib::*;

fn main() {
    let title = CString::new("Window title").unwrap();
    let mut rl = Raylib::init_window(800, 800, &title);

    while !rl.window_should_close() {
        let rl = rl.begin_drawing();
        rl.clear_background(Color::ORANGE);
        rl.end_drawing();
    }
}
