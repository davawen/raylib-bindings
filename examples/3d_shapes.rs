use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 450, "Shapes", 60);
    set_window_state(rl, WindowFlags::RESIZABLE);
    set_exit_key(rl, KeyboardKey::Null);

    let mut camera = Camera3D {
        position: vec3(0.0, 10.0, 10.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 90.0,
        projection: CameraProjection::Perspective as i32
    };

    disable_cursor(rl);

    let mut scale_factor = 1.0;
    let mut target = RenderTexture::load(rl, 800, 450).unwrap(); // render at quarter resolution
    while !window_should_close(rl) {
        if is_key_pressed(rl, KeyboardKey::Escape) {
            enable_cursor(rl);
        } else if !is_cursor_hidden(rl) && is_mouse_button_pressed(rl, MouseButton::Left) {
            disable_cursor(rl);
        }

        if is_cursor_hidden(rl) {
            camera.update_camera(CameraMode::Free);
        }

        let prev_scale_factor = scale_factor;
        if is_key_pressed(rl, KeyboardKey::C) && scale_factor > 1.0 {
            scale_factor -= 1.0;
        } else if is_key_pressed(rl, KeyboardKey::V) && scale_factor < 16.0 {
            scale_factor += 1.0;
        }

        let size = get_render_size(rl) / scale_factor;
        if is_window_resized(rl) || prev_scale_factor != scale_factor {
            target = RenderTexture::load(rl, size.x as u32, size.y as u32).unwrap();
        }

        begin_texture_mode(rl, &mut target, |rl| {
            clear_background(rl, Color::RAYWHITE);
            rl.begin_mode3d(camera, |rl| {
                rl.cube(vec3(-4.0, 0.0, 2.0), 2.0, 5.0, 2.0, Color::RED);
                rl.cube_wires(vec3(-4.0, 0.0, 2.0), 2.0, 5.0, 2.0, Color::GOLD);
                rl.cube_wires(vec3(-4.0, 0.0, -2.0), 3.0, 6.0, 2.0, Color::MAROON);

                rl.sphere(vec3(-1.0, 0.0, -2.0), 1.0, Color::GREEN);
                rl.sphere_wires(vec3(1.0, 0.0, 2.0), 2.0, Color::LIME);

                rl.cylinder(vec3(4.0, 0.0, -2.0), 1.0, 2.0, 3.0, 4, Color::SKYBLUE);
                rl.cylinder_wires(vec3(4.0, 0.0, -2.0), 1.0, 2.0, 3.0, 4, Color::DARKBLUE);
                rl.cylinder_wires(vec3(4.5, -1.0, 2.0), 1.0, 1.0, 2.0, 6, Color::BROWN);

                rl.cylinder(vec3(1.0, 0.0, -4.0), 0.0, 1.5, 3.0, 8, Color::GOLD);
                rl.cylinder_wires(vec3(1.0, 0.0, -4.0), 0.0, 1.5, 3.0, 8, Color::PINK);

                rl.capsule(vec3(-3.0, 1.5, -4.0), vec3(-4.0, -1.0, -4.0), 1.2, 8, 8, Color::VIOLET);
                rl.capsule_wires(vec3(-3.0, 1.5, -4.0), vec3(-4.0, -1.0, -4.0), 1.2, 8, 8, Color::PURPLE);
                rl.grid(20, 1.0);
            })
        });

        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);
            let src = Rectangle::new(0.0, 0.0, size.x, -size.y);
            let dest = Rectangle::from_vecs(Vector2::ZERO, get_render_size(rl));
            draw_texture_pro(rl, target.texture(), src, dest, Vector2::ZERO, 0.0, Color::WHITE);
            draw_text(rl, rl.default_font(), &format!("Scale factor: {scale_factor} (press C and V!)"), Vector2::splat(10.0), 20.0, Color::BLACK);

            let right_text = "Press ESC to unlock cursor";
            let right_text_len = measure_text(rl.default_font(), right_text, 20.0).x;
            let pos = vec2(get_render_width(rl) - right_text_len - 10.0, 10.0);
            draw_text(rl, rl.default_font(), right_text, pos, 20.0, Color::BLACK);
        });
    }
}
