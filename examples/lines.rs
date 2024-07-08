use raylib::prelude::*;

fn main() {
    let rl = &mut init_window(800, 800, "Lines!", 60);

    let mut points = Vec::new();

    while !window_should_close(rl) {
        if is_mouse_button_pressed(rl, MouseButton::Left) {
            points.push(get_mouse_pos(rl));
        } else if is_mouse_button_pressed(rl, MouseButton::Right) {
            points.retain(|&point| point.distance_sqr(get_mouse_pos(rl)) > 5.0*5.0);
        }

        begin_drawing(rl, |rl| {
            clear_background(rl, Color::RAYWHITE);
            draw_line_strip(rl, &points, Color::BLACK);
            for &point in &points {
                draw_circle_v(rl, point, 5.0, Color::RED);
            }
        });
    }
}
