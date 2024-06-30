use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Font size", 60);

    let font = TrueTypeFont::from_bytes(include_bytes!("../assets/iosevka-medium.ttc").as_slice()).unwrap();
    let mut atlases: Vec<_> = (1..=20).map(|px| px as f32).map(|px| font.atlas(rl, px)).collect();

    while !window_should_close(rl) {
        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);
            let mut y = 30.0;
            for atlas in atlases.iter_mut().rev() {
                let size = atlas.size();
                rl.text(atlas, &format!("({size}) 0x012345 abcdefg Hello world! Test!"), vec2(10.0, y), size, Color::BLACK);
                y += size + 5.0;
            }
        });
    }
}
