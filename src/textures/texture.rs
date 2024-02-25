use crate::ffi;

/// A raylib texture.
/// Use `Raylib::load_texture` to create one.
/// 
/// Textures are stored on the GPU in VRAM.
/// If you need to interact with graphical data from the CPU, prefer using an `Image`.
pub struct Texture(ffi::Texture);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.0) }
    }
}

impl Texture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::Texture {
        self.0
    }
}

