use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Shapes", 60);

    let camera = Camera3D {
        position: vec3(0.0, 10.0, 10.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: CameraProjection::Perspective as i32
    };

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing();
        draw.clear_background(Color::RAYWHITE);

        let mut draw = draw.begin_mode3d(camera);
        draw.cube(Vector3::ZERO, 3.0, 3.0, 5.0, Color::RED);
        draw.grid(10, 1.0);
    }
}
