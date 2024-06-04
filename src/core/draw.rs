use std::ops::{Deref, DerefMut};

use crate::{ffi::{self, Camera2D, VrStereoConfig, BlendMode}, prelude::{Color, RenderTexture, Shader}};
use super::Raylib;

pub struct DrawHandle<'a> {
    pub(crate) rl: &'a mut Raylib
}

/// # Drawing (module: `rcore`)
/// See: [`DrawHandle`]
///
/// ---
impl Raylib {
    /// Setup canvas (framebuffer) to start drawing
    pub fn begin_drawing(&mut self, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginDrawing() }
        let mut d = DrawHandle { rl: self };
        f(&mut d);
        unsafe { ffi::EndDrawing() }
    }

    /// Begin drawing to render texture
    pub fn begin_texture_mode(&mut self, target: &mut RenderTexture, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginTextureMode(target.get_ffi_texture()) }
        let mut d = DrawHandle { rl: self };
        f(&mut d);
        unsafe { ffi::EndTextureMode() }
    }
}

impl Deref for DrawHandle<'_> {
    type Target = Raylib;
    fn deref(&self) -> &Self::Target {
        self.rl
    }
}

impl DerefMut for DrawHandle<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rl
    }
}

/// # Begin modes (module: `rcore`)
/// 
/// ---
impl DrawHandle<'_> {
    // Begin 2D mode with custom camera (2D)
    pub fn begin_mode2d(&mut self, camera: Camera2D, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginMode2D(camera) }
        f(self);
        unsafe { ffi::EndMode2D() }
    }
    // Begin custom shader drawing
    pub fn begin_shader_mode(&mut self, shader: &Shader, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginShaderMode(*shader.get_ffi()) }
        f(self);
        unsafe { ffi::EndShaderMode() }
    }
    // Begin blending mode (alpha, additive, multiplied, subtract, custom) 
    pub fn begin_blend_mode(&mut self, mode: BlendMode, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginBlendMode(mode as i32) }
        f(self);
        unsafe { ffi::EndBlendMode() }
    }
    // Begin stereo rendering (requires VR simulator)
    pub fn begin_vr_stereo_mode(&mut self, config: VrStereoConfig, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginVrStereoMode(config) }
        f(self);
        unsafe { ffi::EndVrStereoMode() }
    }
    // Begin scissor mode (define screen area for following drawing)
    pub fn begin_scissor_mode(&mut self, x: i32, y: i32, width: i32, height: i32, f: impl FnOnce(&mut DrawHandle)) {
        unsafe { ffi::BeginScissorMode(x, y, width, height) }
        f(self);
        unsafe { ffi::EndScissorMode() }
    }
}

impl DrawHandle<'_> {
    pub fn clear_background(&self, color: Color) { unsafe { ffi::ClearBackground(color) } }
}
