use std::f32::consts::PI;

use raylib::{prelude::*, cstr};

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
        (cstr!("VERTICAL GRADIENT"), Color::RAYWHITE), 
        (cstr!("HORIZONTAL GRADIENT"), Color::RAYWHITE), 
        (cstr!("DIAGONAL GRADIENT"), Color::RAYWHITE), 
        (cstr!("RADIAL GRADIENT"), Color::LIGHTGRAY), 
        (cstr!("SQUARE GRADIENT"), Color::LIGHTGRAY), 
        (cstr!("CHECKED"), Color::RAYWHITE), 
        (cstr!("WHITE NOISE"), Color::RED), 
        (cstr!("PERLIN NOISE"), Color::RED), 
        (cstr!("CELLULAR"), Color::RAYWHITE), 
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
        unsafe { raylib::ffi::DrawText(names[current].0.as_ptr(), 10, 10, 20, names[current].1) };
    }
}
