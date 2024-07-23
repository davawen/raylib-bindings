use std::ops::{Deref, DerefMut};

use crate::{ffi, prelude::{Camera3D, DrawHandle, Raylib}};

pub mod shapes;
pub mod model;
pub mod mesh;
pub mod material;

/// A handle created with a camera that allows 3d drawing.
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(800, 800, "3d", 60);
/// let camera = Camera {
///     position: vec3(0.0, 0.0, 1.0),
///     target: vec3(0.0, 0.0, 0.0),
///     up: vec3(0.0, 1.0, 0.0),
///     fovy: 90.0,
///     projection: CameraProjection::Perspective as i32
/// };
/// 
/// begin_drawing(rl, |rl| {
///     begin_mode3d(rl, camera, |rl| {
///         // draw a mesh or a model here
///     });
/// })
/// ```
pub struct DrawHandle3D<'a> {
    rl: &'a mut Raylib
}

impl Deref for DrawHandle3D<'_> {
    type Target = Raylib;
    fn deref(&self) -> &Self::Target {
        self.rl
    }
}

impl DerefMut for DrawHandle3D<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.rl
    }
}

// Begin 3D mode with custom camera (3D)
pub fn begin_mode3d(rl: &mut DrawHandle, camera: Camera3D, f: impl FnOnce(&mut DrawHandle3D)) {
    unsafe { ffi::BeginMode3D(camera) }
    let mut d = DrawHandle3D { rl };
    f(&mut d);
    unsafe { ffi::EndMode3D() }
}
