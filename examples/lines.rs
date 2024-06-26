use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(800, 800, "Lines!", 60);

    let mut points = Vec::new();

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::Left) {
            points.push(rl.get_mouse_pos());
        } else if rl.is_mouse_button_pressed(MouseButton::Right) {
            points.retain(|&point| point.distance_sqr(rl.get_mouse_pos()) > 5.0*5.0);
        }

        rl.begin_drawing(|rl| {
            rl.clear_background(Color::RAYWHITE);
            rl.line_strip(&points, Color::BLACK);
            for &point in &points {
                rl.circle_v(point, 5.0, Color::RED);
            }
        });
    }
}
