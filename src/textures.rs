use crate::ffi;

/// A raylib texture.
/// Use `Raylib::load_texture` to create one.
/// 
/// Textures are stored on the GPU in VRAM.
/// If you need to interact with graphical data from the CPU, prefer using an `Image`.
pub struct Texture(ffi::Texture);

/// A raylib render texture.
/// Use `Raylib::load_render_texture` to create one.
///
/// Render textures work in the same way as textures, except that you can use them as render targets.
pub struct RenderTexture(ffi::RenderTexture);

/// A raylib image.
/// Use `Raylib::load_image` to create one.
/// 
/// Images are stored on the CPU in RAM.
/// If you need to draw images many times, or if you need to use them in shaders, prefer using a `Texture`.
pub struct Image(ffi::Image);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.0) }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRenderTexture(self.0) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.0) }
    }
}

impl Texture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::Texture {
        self.0
    }
}

impl RenderTexture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::RenderTexture {
        self.0
    }
}

impl Image {
    pub unsafe fn get_ffi_image(&self) -> ffi::Image {
        self.0
    }
}
