use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Font size", 60);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    let mut atlases: Vec<_> = (1..=20).map(|px| px as f32).map(|px| font.atlas(&mut rl, px)).collect();

    while !rl.window_should_close() {
        rl.begin_drawing(|_, draw| {
            draw.clear_background(Color::RAYWHITE);
            let mut y = 30.0;
            for atlas in atlases.iter_mut().rev() {
                let size = atlas.size();
                draw.text(atlas, &format!("({size}) 0x012345 abcdefg Hello world! Test!"), vec2(10.0, y), size, Color::BLACK);
                y += size + 5.0;
            }
        });
    }
}
