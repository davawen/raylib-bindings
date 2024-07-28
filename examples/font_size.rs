use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Font size", 60);

    let font = load_font_bytes(rl, include_bytes!("../assets/iosevka-medium.ttc").as_slice());

    while !window_should_close(rl) {
        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);
            let mut y = 30.0;
            for size in (1..=20).rev() {
                let size = size as f32;
                draw_text(rl, &font, &format!("({size}) 0x012345 abcdefg Hello world! Test!"), vec2(10.0, y), size, Color::BLACK);
                y += size + 5.0;
            }
        });
    }
}
