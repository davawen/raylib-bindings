use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(400, 400, "Subpixel positioning", 60);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    let mut rendered = font.atlas(rl, 40.0);

    let mut pos = 0.0;

    let mut size = 40.0;

    while !window_should_close(rl) {
        size += get_mouse_wheel_move(rl);
        if size != rendered.size() {
            rendered.reatlas(rl, size);
        }

        if is_key_down(rl, Key::Right) {
            pos += 0.05;
        } else if is_key_down(rl, Key::Left) {
            pos -= 0.05;
        }

        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);

            draw_text(rl, &mut rendered, &format!("Pos: {pos}"), vec2(20.0, 40.0), size, Color::BLACK);
            draw_circle(rl, 20.0, 40.0, 5.0, Color::RED);
        });
    }
}
