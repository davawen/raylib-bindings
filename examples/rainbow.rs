use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Window title", 60);

    let shader = rl.load_shader::<&str>(None, Some("assets/sine.glsl")).unwrap();
    let time_uniform = shader.get_uniform("time");

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing();
        draw.clear_background(Color::WHITE);

        shader.set_uniform_value(time_uniform, unsafe { raylib::ffi::GetTime() as f32 });
        let mut shader = draw.begin_shader_mode(&shader);
        shader.rectangle(50.0, 50.0, 700.0, 700.0, Color::WHITE);
    }
}
