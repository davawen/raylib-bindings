use crate::{ffi, prelude::{DrawHandle, Camera3D}};

mod shapes;
mod model;
mod mesh;
mod material;

pub use model::*;
pub use mesh::*;
pub use material::*;

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

