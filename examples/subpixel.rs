use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(400, 400, "Subpixel positioning", 60);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    let mut rendered = font.atlas(&mut rl, 40.0);

    let mut pos = 0.0;

    let mut size = 40.0;

    while !rl.window_should_close() {
        size += rl.get_mouse_wheel_move();
        if size != rendered.size() {
            rendered.reatlas(&mut rl, size);
        }

        if rl.is_key_down(KeyboardKey::Right) {
            pos += 0.05;
        } else if rl.is_key_down(KeyboardKey::Left) {
            pos -= 0.05;
        }

        rl.begin_drawing(|rl| {
            rl.clear_background(Color::RAYWHITE);

            rl.text(&mut rendered, &format!("Pos: {pos}"), vec2(20.0, 40.0), size, Color::BLACK);
            rl.circle(20.0, 40.0, 5.0, Color::RED);
        });
    }
}
