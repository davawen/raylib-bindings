//! Drawing functions.
//! See: [`DrawHandle`].
//!
//! See: [`[rshapes]`][`crate::shapes`] to draw basic shapes.  
//! See: [`[rtextures]`][`crate::textures`] to handle and draw textures.  
//! See: [`[rmodels]`][`crate::model`] to handle and draw 3D models.  

use std::ops::{Deref, DerefMut};

use crate::{ffi::{self, Camera2D, VrStereoConfig, BlendMode}, prelude::{Color, RenderTexture, Shader}};
use super::Raylib;

pub struct DrawHandle<'a> {
    pub(crate) rl: &'a mut Raylib
}

/// Setup canvas (framebuffer) to start drawing.
/// # Usage
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// begin_drawing(rl, |rl| {
///     rl.clear_background(Color::WHITE);
///     rl.rectangle(10.0, 10.0, 100.0, 100.0, Color::RED);
/// });
/// ```
pub fn begin_drawing(rl: &mut Raylib, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginDrawing() }
    let mut d = DrawHandle { rl };
    f(&mut d);
    unsafe { ffi::EndDrawing() }
}

/// Start drawing to a render texture
/// # Usage
/// ```
/// use std::f32::consts::PI;
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// let mut frame = RenderTexture::load(rl, 800, 800).unwrap();
/// while !window_should_close(rl) {
///     begin_texture_mode(rl, &mut frame, |rl| {
///         rl.clear_background(Color::WHITE);
///         rl.rectangle(10.0, 10.0, 100.0, 100.0, Color::RED);
///     });
///     // ...
///     begin_drawing(rl, |rl| {
///         rl.texture_ex(frame.texture(), vec2(40.0, 70.0), PI/2.0, 2.0, Color::WHITE);
///    })
///     # break;
/// }
/// ```
pub fn begin_texture_mode(rl: &mut Raylib, target: &mut RenderTexture, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginTextureMode(target.get_ffi_texture()) }
    let mut d = DrawHandle { rl };
    f(&mut d);
    unsafe { ffi::EndTextureMode() }
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

// Begin 2D mode with custom camera (2D)
pub fn begin_mode2d(rl: &mut DrawHandle, camera: Camera2D, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginMode2D(camera) }
    f(rl);
    unsafe { ffi::EndMode2D() }
}
// Begin custom shader drawing
pub fn begin_shader_mode(rl: &mut DrawHandle, shader: &Shader, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginShaderMode(*shader.get_ffi()) }
    f(rl);
    unsafe { ffi::EndShaderMode() }
}
// Begin blending mode (alpha, additive, multiplied, subtract, custom) 
pub fn begin_blend_mode(rl: &mut DrawHandle, mode: BlendMode, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginBlendMode(mode as i32) }
    f(rl);
    unsafe { ffi::EndBlendMode() }
}
// Begin stereo rendering (requires VR simulator)
pub fn begin_vr_stereo_mode(rl: &mut DrawHandle, config: VrStereoConfig, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginVrStereoMode(config) }
    f(rl);
    unsafe { ffi::EndVrStereoMode() }
}
// Begin scissor mode (define screen area for following drawing)
pub fn begin_scissor_mode(rl: &mut DrawHandle, x: i32, y: i32, width: i32, height: i32, f: impl FnOnce(&mut DrawHandle)) {
    unsafe { ffi::BeginScissorMode(x, y, width, height) }
    f(rl);
    unsafe { ffi::EndScissorMode() }
}

pub fn clear_background(_rl: &DrawHandle, color: Color) { unsafe { ffi::ClearBackground(color) } }
