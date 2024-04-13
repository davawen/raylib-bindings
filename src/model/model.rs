use std::ffi::CStr;

use crate::{ffi, prelude::{Vector2, Vector3, Color, Texture, Camera, BoundingBox, Rectangle}};
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
    /// Draw a model (with texture if set)
    pub fn model(&mut self, model: &Model, pos: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModel(model.0, pos, scale, tint) }
    }

    /// Draw a model (with texture if set) with extended parameters
    pub fn model_ex(&mut self, model: &Model, pos: Vector3, axis: Vector3, angle: f32, scale: Vector3, tint: Color) {
        unsafe { ffi::DrawModelEx(model.0, pos, axis, angle, scale, tint) }
    }

    /// Draw a wireframe model (with texture if set)
    pub fn model_wires(&mut self, model: &Model, pos: Vector3, scale: f32, tint: Color) {
        unsafe { ffi::DrawModelWires(model.0, pos, scale, tint) }
    }

    /// Draw a wireframe model (with texture if set) with extended parameters
    pub fn model_wires_ex(&mut self, model: &Model, pos: Vector3, axis: Vector3, angle: f32, scale: Vector3, tint: Color) {
        unsafe { ffi::DrawModelWiresEx(model.0, pos, axis, angle, scale, tint) }
    }

    /// Draw a wireframe bounding box
    pub fn bounding_box(bounds: BoundingBox, color: Color) {
        unsafe { ffi::DrawBoundingBox(bounds, color) }
    }

    /// Draw a billboard texture
    pub fn billboard(camera: Camera, texture: Texture, pos: Vector3, size: f32, tint: Color) {
        unsafe { ffi::DrawBillboard(camera, *texture.get_ffi(), pos, size, tint) }
    }

    /// Draw part of a billboard texture
    pub fn billboard_rec(camera: Camera, texture: Texture, source: Rectangle, pos: Vector3, size: Vector2, tint: Color) {
        unsafe { ffi::DrawBillboardRec(camera, *texture.get_ffi(), source, pos, size, tint) }
    }

    /// Draw part of a billboard texture with source and rotation
    /// Angles are in a radians
    pub fn billboard_pro(
        camera: Camera, texture: Texture, source: Rectangle,
        pos: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: f32, tint: Color)
    {
        unsafe { ffi::DrawBillboardPro(camera, *texture.get_ffi(), source, pos, up, size, origin, rotation.to_degrees(), tint) }
    }
}
