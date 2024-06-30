use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Window title", 60);

    let shader = Shader::load(rl, None, Some("assets/sine.glsl")).unwrap();
    let time_uniform = shader.get_uniform("time");

    while !window_should_close(rl) {
        begin_drawing(rl, |rl| {
            clear_background(rl, Color::WHITE);

            shader.set_uniform_value(time_uniform, rl.get_time());
            begin_shader_mode(rl, &shader, |rl| rl.rectangle(50.0, 50.0, 700.0, 700.0, Color::WHITE));
        });
    }
}
