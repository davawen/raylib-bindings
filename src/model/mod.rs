use crate::{ffi, prelude::{DrawHandle, Camera3D}};

mod shapes;
mod model;
mod mesh;
mod material;

pub use model::*;
pub use mesh::*;
pub use material::*;

/// A handle created with a camera that allows 3d drawing.
/// ```
/// # use raylib::prelude::*;
/// # let mut rl = Raylib::init_window(800, 800, "3d", 60);
/// let camera = Camera {
///     position: vec3(0.0, 0.0, 1.0),
///     target: vec3(0.0, 0.0, 0.0),
///     up: vec3(0.0, 1.0, 0.0),
///     fovy: 90.0,
///     projection: CameraProjection::Perspective as i32
/// };
/// 
/// rl.begin_drawing(|_, draw| {
///     draw.begin_mode3d(camera, |draw| {
///         // draw a mesh or a model here
///     });
/// })
/// ```
pub struct DrawHandle3D {
    _private: ()
}

impl DrawHandle {
    // Begin 3D mode with custom camera (3D)
    pub fn begin_mode3d(&mut self, camera: Camera3D, f: impl FnOnce(&mut DrawHandle3D)) {
        unsafe { ffi::BeginMode3D(camera) }
        let mut d = DrawHandle3D { _private: () };
        f(&mut d);
        unsafe { ffi::EndMode3D() }
    }
}

