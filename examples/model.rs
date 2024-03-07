use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "", 60);
    let mut mat = Material::load_default(&mut rl);
    let mut mesh = Mesh::gen_cube(1.0, 1.0, 1.0);

    let mut camera = Camera3D {
        up: Vector3::Y,
        fovy: 70.0,
        target: Vector3::ZERO,
        position: vec3(0.0, 2.0, -5.0),
        projection: CameraProjection::Perspective as i32
    };

    while !rl.window_should_close() {
        camera.update_camera(CameraMode::Orbital);

        rl.begin_drawing(|rl, draw| {
            draw.clear_background(Color::BLACK);
            draw.begin_mode3d(camera, |draw| {
                draw.mesh(&mesh, &mat, Matrix::IDENTITY);
            });
        });
    }
}
