use std::ffi::CStr;

use crate::{ffi, prelude::{Vector3, Color}};
use super::DrawHandle3D;

#[derive(Debug)]
pub struct Model(ffi::Model);

impl Model {
    /// Load a model from a file (meshes and materials).
    ///
    /// Returns `None` if the given file does not exist,
    /// if its not in a valid format, if it is empty, or if the given path is not a valid C string.
    ///
    /// Supports the following formats (if they were compiled into raylib): `obj`, `iqm`, `gltf`, `vox` and `m3d`.
    pub fn load(path: impl AsRef<std::path::Path>) -> Option<Model> {
        let cstr = std::ffi::CString::new(path.as_ref().as_os_str().as_encoded_bytes()).unwrap();
        Model::load_cstr(&cstr)
    }

    /// Load a model from a file (meshes and materials).
    ///
    /// Returns `None` if the given file does not exist, if its not in a valid format or if it is empty.
    ///
    /// Supports the following formats (if they were compiled into raylib): `obj`, `iqm`, `gltf`, `vox` and `m3d`.
    pub fn load_cstr(path: &CStr) -> Option<Model> {
        let model = unsafe { ffi::LoadModel(path.as_ptr()) };
        if model.meshCount == 0 || model.meshes.is_null() {
            return None
        }

        Some(Model(model))
    }
}

impl DrawHandle3D {
    pub fn draw_model(&mut self, model: &Model, pos: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModel(model.0, pos, scale, tint) }
    }
}
