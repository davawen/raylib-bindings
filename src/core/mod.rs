pub mod window;
pub mod cursor;
pub mod draw;
pub mod shader;
pub mod vr;

use crate::ffi;

pub struct Raylib {
    // disallows initialization from outside
    _private: ()
}

impl Drop for Raylib {
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}

