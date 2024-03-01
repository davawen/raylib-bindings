use std::f32::consts::PI;

use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 450;

    let mut rl = Raylib::init_window(width as i32, height as i32, "Procedural textures", 60);

    let images = [
        rl.gen_image_gradient_linear(width, height, 0.0, Color::RED, Color::BLUE),
        rl.gen_image_gradient_linear(width, height, PI / 2.0, Color::RED, Color::BLUE),
        rl.gen_image_gradient_linear(width, height, PI / 4.0, Color::RED, Color::BLUE),
        rl.gen_image_gradient_radial(width, height, 0.0, Color::WHITE, Color::BLACK),
        rl.gen_image_gradient_square(width, height, 0.0, Color::WHITE, Color::BLACK),
        rl.gen_image_checked(width, height, 32, 32, Color::RED, Color::BLUE),
        rl.gen_image_white_noise(width, height, 0.5),
        rl.gen_image_perlin_noise(width, height, 50, 50, 4.0),
        rl.gen_image_cellular(width, height, 32)
    ];
    let textures = images.map(|image| rl.load_texture_from_image(&image).unwrap());
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

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::Left) {
            current += 1;
            if current >= textures.len() {
                current = 0;
            }
        }

        let mut draw = rl.begin_drawing();
        draw.texture(&textures[current], 0.0, 0.0, Color::WHITE);
        draw.text(rl.default_font(), names[current].0, Vector2::new(10.0, 10.0), names[current].1);
    }
}
