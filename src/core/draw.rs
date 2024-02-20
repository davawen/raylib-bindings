use crate::{ffi::{self, Color, Camera2D, Camera3D, RenderTexture2D, VrStereoConfig, BlendMode}, core::shader::Shader};
use super::Raylib;

pub enum RenderMode {
    Drawing,
    Mode2D,
    Mode3D,
    TextureMode,
    ShaderMode,
    BlendMode,
    VrStereoMode,
    ScissorMode
}

pub struct DrawHandle<'a, P> {
    mode: RenderMode,
    _parent: Option<&'a mut P>
}

impl Raylib {
    /// Setup canvas (framebuffer) to start drawing
    pub fn begin_drawing(&mut self) -> DrawHandle<'static, ()> {
        unsafe { ffi::BeginDrawing() }
        DrawHandle { mode: RenderMode::Drawing, _parent: None }
    }
}

impl<P> DrawHandle<'_, P> {
    // Begin 2D mode with custom camera (2D)
    pub fn begin_mode2d(&mut self, camera: Camera2D) -> DrawHandle<Self> {
        unsafe { ffi::BeginMode2D(camera) }
        DrawHandle { mode: RenderMode::Mode2D, _parent: Some(self) }
    }
    // Begin 3D mode with custom camera (3D)
    pub fn begin_mode3d(&mut self, camera: Camera3D) -> DrawHandle<Self> {
        unsafe { ffi::BeginMode3D(camera) }
        DrawHandle { mode: RenderMode::Mode3D, _parent: Some(self) }
    }
    // Begin drawing to render texture
    pub fn begin_texture_mode(&mut self, target: RenderTexture2D) -> DrawHandle<Self> {
        unsafe { ffi::BeginTextureMode(target) }
        DrawHandle { mode: RenderMode::TextureMode, _parent: Some(self) }
    }
    // Begin custom shader drawing
    pub fn begin_shader_mode(&mut self, shader: &Shader) -> DrawHandle<Self> {
        unsafe { ffi::BeginShaderMode(shader.get_ffi_shader()) }
        DrawHandle { mode: RenderMode::ShaderMode, _parent: Some(self) }
    }
    // Begin blending mode (alpha, additive, multiplied, subtract, custom) 
    pub fn begin_blend_mode(&mut self, mode: BlendMode) -> DrawHandle<Self> {
        unsafe { ffi::BeginBlendMode(mode as i32) }
        DrawHandle { mode: RenderMode::BlendMode, _parent: Some(self) }
    }
    // Begin stereo rendering (requires VR simulator)
    pub fn begin_vr_stereo_mode(&mut self, config: VrStereoConfig) -> DrawHandle<Self> {
        unsafe { ffi::BeginVrStereoMode(config) }
        DrawHandle { mode: RenderMode::VrStereoMode, _parent: Some(self) }
    }
    // Begin scissor mode (define screen area for following drawing)
    pub fn begin_scissor_mode(&mut self, x: i32, y: i32, width: i32, height: i32) -> DrawHandle<Self> {
        unsafe { ffi::BeginScissorMode(x, y, width, height) }
        DrawHandle { mode: RenderMode::ScissorMode, _parent: Some(self) }
    }
}

impl<P> DrawHandle<'_, P> {
    pub fn end(self) { }

    pub fn clear_background(&mut self, color: Color) { unsafe { ffi::ClearBackground(color) } }
}

impl<P> Drop for DrawHandle<'_, P> {
    fn drop(&mut self) {
        unsafe { match self.mode {
            RenderMode::Drawing => ffi::EndDrawing(),
            RenderMode::Mode2D => ffi::EndMode2D(),
            RenderMode::Mode3D => ffi::EndMode3D(),
            RenderMode::TextureMode => ffi::EndTextureMode(),
            RenderMode::ShaderMode => ffi::EndShaderMode(),
            RenderMode::BlendMode => ffi::EndBlendMode(),
            RenderMode::VrStereoMode => ffi::EndVrStereoMode(),
            RenderMode::ScissorMode => ffi::EndScissorMode()
        } }
    }
}

