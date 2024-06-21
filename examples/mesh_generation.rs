use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Mesh Generation", 60);

    let mut camera = Camera {
        position: vec3(0.0, 5.0, 3.0),
        target: vec3(0.0, 3.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 80.0,
        projection: CameraProjection::Perspective as i32
    };

    let mut helix = Vec::new();

    // add a face in both clockwise and counter-clockwise order
    let mut add_two = |a, b, c| {
        helix.push(a);
        helix.push(b);
        helix.push(c);

        helix.push(c);
        helix.push(b);
        helix.push(a);
    };

    let mut add_spiral = |offset: f32| {
        for i in 0..100 {
            let h = (i as f32) / 10.0;

            let pos = vec3((h + offset).cos(), h/2.0, (h + offset).sin());
            let beneath_pos = pos - vec3(0.0, 0.05, 0.0);

            let h = h + 0.1;
            let next = vec3((h + offset).cos(), h/2.0, (h + offset).sin());
            let beneath_next = next - vec3(0.0, 0.05, 0.0);

            add_two(pos, next, beneath_next);
            add_two(pos, next, beneath_pos);
        }
    };

    add_spiral(0.0);
    add_spiral(std::f32::consts::PI);

    let helix = MeshBuilder::new(&helix, &helix.iter().map(|_| vec2(0.0, 0.0)).collect::<Vec<_>>())
        .build();

    let mut mat = Material::load_default(rl);
    mat.set_color(MaterialMapIndex::Albedo, Color::RED);

    while !window_should_close(rl) {
        camera.update_camera(CameraMode::Orbital);

        rl.begin_drawing(|rl| {
            rl.clear_background(Color::WHITE);

            rl.begin_mode3d(camera, |rl| {
                rl.mesh(&helix, &mat, Matrix::IDENTITY);
            });
        });
    }
}
