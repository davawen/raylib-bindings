use std::f32::consts::PI;

use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 450;

    let mut rl = &mut init_window(width as i32, height as i32, "Procedural textures", 60);

    let images = [
        Image::gen_gradient_linear(&mut rl, width, height, 0.0, Color::RED, Color::BLUE),
        Image::gen_gradient_linear(&mut rl, width, height, PI / 2.0, Color::RED, Color::BLUE),
        Image::gen_gradient_linear(&mut rl, width, height, PI / 4.0, Color::RED, Color::BLUE),
        Image::gen_gradient_radial(&mut rl, width, height, 0.0, Color::WHITE, Color::BLACK),
        Image::gen_gradient_square(&mut rl, width, height, 0.0, Color::WHITE, Color::BLACK),
        Image::gen_checked(&mut rl, width, height, 32, 32, Color::RED, Color::BLUE),
        Image::gen_white_noise(&mut rl, width, height, 0.5),
        Image::gen_perlin_noise(&mut rl, width, height, 50, 50, 4.0),
        Image::gen_cellular(&mut rl, width, height, 32)
    ];
    let textures = images.map(|image| Texture::load_from_image(&mut rl, &image).unwrap());
    let names = [
        ("VERTICAL GRADIENT", Color::RAYWHITE), 
        ("HORIZONTAL GRADIENT", Color::RAYWHITE), 
        ("DIAGONAL GRADIENT", Color::RAYWHITE), 
        ("RADIAL GRADIENT", Color::LIGHTGRAY), 
        ("SQUARE GRADIENT", Color::LIGHTGRAY), 
        ("CHECKED", Color::RAYWHITE), 
        ("WHITE NOISE", Color::RED), 
        ("PERLIN NOISE", Color::RED), 
        ("CELLULAR", Color::RAYWHITE), 
    ];

    let mut current = 0;

    while !window_should_close(rl) {
        if rl.is_mouse_button_pressed(MouseButton::Left) {
            current += 1;
            if current >= textures.len() {
                current = 0;
            }
        }

        rl.begin_drawing(|rl| {
            rl.texture(&textures[current], 0.0, 0.0, Color::WHITE);
            rl.text(rl.default_font(), names[current].0, vec2(10.0, 10.0), 20.0, names[current].1);
        });
    }
}
