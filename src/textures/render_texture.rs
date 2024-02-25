use crate::ffi;

/// A raylib render texture.
/// Use `Raylib::load_render_texture` to create one.
///
/// Render textures work in the same way as textures, except that you can use them as render targets.
pub struct RenderTexture(ffi::RenderTexture);

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRenderTexture(self.0) }
    }
}

impl RenderTexture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::RenderTexture {
        self.0
    }
}
