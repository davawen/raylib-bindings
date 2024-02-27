use raylib::{prelude::*, text::Font};

fn main() {
    let mut rl = Raylib::init_window(1800, 1100, "Rust text!", 60);

    // let font = rusttype::Font::try_from_bytes(include_bytes!("/usr/share/fonts/TTF/iosevka-medium.ttc")).unwrap();
    let font = rusttype::Font::try_from_bytes(include_bytes!("/home/davawen/Downloads/symbola-font/Symbola-AjYx.ttf")).unwrap();
    let font = Font::new(font);

    let a = std::time::Instant::now();
    let rendered = rl.render_font(&font, 20.0);
    let b = std::time::Instant::now().duration_since(a);

    println!("font rendering took: {:.3} ms", (b.as_micros() as f32) / 1000.0);

    let mut size = 20.0;
    let mut rendered_size = rl.render_font(&font, size);

    while !rl.window_should_close() {
        size += rl.get_mouse_wheel_move();
        if rendered_size.size() != size {
            rendered_size = rl.render_font(&font, size);
        }

        let mut draw = rl.begin_drawing();
        draw.clear_background(Color::RAYWHITE);

        draw.text(&rendered, &format!("size: {size}"), Vector2::new(20.0, 20.0), Color::BLACK);
        for i in 0..20 {
            draw.text(&rendered, "Hello, world! I LOVE UUUUU HOW ARE YOU DOING???", Vector2::new(20.0, (i as f32*(size + 10.0)) + 50.0), Color::BLACK);
        }
        // unsafe {
        //     let c = CString::new(format!("size: {size}")).unwrap();
        //     raylib::ffi::DrawText(c.as_ptr(), 20, 20, 20, Color::BLACK); 
        // }
        // let s = cstr!("Hello, world! I LOVE UUUUU HOW ARE YOU DOING???");
        // for i in 0..20 {
        //     unsafe {
        //         raylib::ffi::DrawText(s.as_ptr(), 20, (i*(size as i32 + 10)) + 50, size as i32, Color::BLACK);
        //     }
        // }

        // draw.fps(Vector2::new(700.0, 20.0));

        draw.circle_v(rl.get_mouse_pos(), 10.0, Color::RED);
    }
}
