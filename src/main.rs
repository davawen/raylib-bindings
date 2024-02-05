use std::ffi::CString;

mod ffi;

fn main() {
    unsafe {
        let title = CString::new("Window title").unwrap();
        ffi::InitWindow(800, 800, title.as_ptr());

        while !ffi::WindowShouldClose() {
            ffi::BeginDrawing();
            ffi::ClearBackground(ffi::Color { r: 200, g: 200, b: 200, a: 255 });
            let text = CString::new("YAYYYY FFI!!!!!").unwrap();
            ffi::DrawText(text.as_ptr(), 190, 200, 20, ffi::Color { r: 0, g: 0, b: 0, a: 255 });
            ffi::EndDrawing();
        }

        ffi::CloseWindow();
    }
}
