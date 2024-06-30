use raylib::prelude::*;

fn main() {
    let mut rl = &mut init_window(800, 800, "test", 60);

    let texture = Texture::load_empty(&mut rl, 500, 500, PixelFormat::UncompressedGrayAlpha).unwrap();

    let mut b = vec![0; 100*100*2];
    for i in 0..100*100 {
        if i % 5 == 0 {
            b[i*2] = 255;
            b[i*2+1] = 255;
        } else {
            b[i*2+1] = 0;
        }
    }
    let rec = Rectangle::new(300.0, 300.0, 100.0, 100.0);
    texture.update_rec_raw(rec, &b).unwrap();

    while !window_should_close(rl) {
        begin_drawing(rl, |rl| {
            clear_background(rl, Color::GRAY);
            rl.circle(300.0, 300.0, 20.0, Color::RED);
            rl.texture(&texture, 100.0, 100.0, Color::WHITE);
        });
    }
}
