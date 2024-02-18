use raylib::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Window title", 60);

    let shader = rl.load_shader::<&str>(None, Some("assets/sine.glsl")).unwrap();
    let time_uniform = shader.get_uniform("time");

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing();
        draw.clear_background(Color::WHITE);

        shader.set_uniform_value(time_uniform, unsafe { raylib::ffi::GetTime() as f32 });
        let shader = draw.begin_shader_mode(&shader);
        unsafe { raylib::ffi::DrawRectangle(50, 50, 700, 700, Color::WHITE) }
        shader.end();

        draw.end();
    }
}
