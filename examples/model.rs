use std::f32::consts::PI;

use raylib::prelude::*;

struct Light {
    enabled: i32,
    ty: i32,
    position: Vector3,
    target: Vector3,
    color: Vector4,

    u_enabled: Uniform,
    u_ty: Uniform,
    u_position: Uniform,
    u_target: Uniform,
    u_color: Uniform
}

impl Light {
    fn new(shader: &Shader, idx: i32) -> Self {
        let u_enabled = shader.get_uniform(&format!("lights[{idx}].enabled"));
        let u_ty = shader.get_uniform(&format!("lights[{idx}].type"));
        let u_position = shader.get_uniform(&format!("lights[{idx}].position"));
        let u_target = shader.get_uniform(&format!("lights[{idx}].target"));
        let u_color = shader.get_uniform(&format!("lights[{idx}].color"));

        Self {
            enabled: 0,
            ty: 0,
            position: Vector3::ZERO,
            target: Vector3::ZERO,
            color: Vector4::ONE,
            u_enabled,
            u_ty,
            u_position,
            u_target,
            u_color
        }
    }

    fn send(&self, shader: &Shader) {
        shader.set_uniform_value(self.u_enabled, self.enabled);
        shader.set_uniform_value(self.u_ty, self.ty);
        shader.set_uniform_value(self.u_target, self.target);
        shader.set_uniform_value(self.u_position, self.position);
        shader.set_uniform_value(self.u_color, self.color);
    }
}

fn main() {
    let mut rl = Raylib::init_window(800, 800, "", 60);
    rl.set_window_state(ConfigFlags::FLAG_WINDOW_RESIZABLE);
    let shader = Shader::load(&mut rl, Some("assets/light.vs"), Some("assets/light.fs")).unwrap();
    let u_ambient = shader.get_uniform("ambient");
    shader.set_uniform_value(u_ambient, vec4(0.05, 0.05, 0.1, 1.0));

    let mut mat = Material::load_default(&mut rl);
    let mesh = Mesh::gen_cube(1.0, 1.0, 1.0);

    let mut lights = [
        Light::new(&shader, 0),
        Light::new(&shader, 1),
        Light::new(&shader, 2),
        Light::new(&shader, 3)
    ];

    lights[0].color = vec4(0.6, 0.0, 0.0, 1.0);
    lights[1].color = vec4(0.2, 0.0, 0.6, 1.0);
    lights[2].color = vec4(0.2, 0.7, 0.0, 1.0);
    lights[3].color = vec4(0.6, 0.2, 0.8, 1.0);

    let rng = || {
        (rl.get_random_value(0, 10000) as f32) / 10000.0
    };

    for light in &mut lights {
        light.enabled = 1;
        light.position = vec3(rng()*10.0 - 5.0, rng()*10.0 - 5.0, rng()*10.0 - 5.0);
        light.ty = 1;
        light.send(&shader);
    }
    mat.set_shader(&shader);

    let transforms: Vec<_> = (0..10).map(|_| {
        // Matrix::scale(vec3(0.1, 0.1, 0.1)) *
        Matrix::translation(vec3(rng()*10.0 - 5.0, rng()*10.0 - 5.0, rng()*10.0 - 5.0)) *
        Matrix::rotation(vec3(rng()*2.0-1.0, rng()*2.0-1.0, rng()*2.0-1.0).normalize(), rng()*2.0*PI)
    }).collect();

    let light_transforms: Vec<_> = lights.iter().map(|l| (l.color, Matrix::translation(l.position))).collect();
    let mut light_mat = Material::load_default(&mut rl);
    let light_mesh = Mesh::gen_sphere(0.1, 16, 16);

    let mut camera = Camera3D {
        up: Vector3::Y,
        fovy: 70.0,
        target: Vector3::ZERO,
        position: vec3(0.0, 6.0, -8.0),
        projection: CameraProjection::Perspective as i32
    };

    let loc_view = shader.get_uniform("viewPos");

    while !rl.window_should_close() {
        camera.update_camera(CameraMode::Orbital);
        shader.set_uniform_value(loc_view, camera.position);

        rl.begin_drawing(|rl| {
            rl.clear_background(Color::BLACK);
            rl.begin_mode3d(camera, |rl| {
                for transform in &transforms {
                    rl.mesh(&mesh, &mat, *transform);
                }
                for &(color, transform) in &light_transforms {
                    light_mat.set_color(MaterialMapIndex::Albedo, Color::color_from_normalized(color));
                    rl.mesh(&light_mesh, &light_mat, transform);
                }
            });
        });
    }
}
