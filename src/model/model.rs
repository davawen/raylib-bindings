use std::ffi::CStr;

use crate::{ffi, prelude::{BoundingBox, Camera, Color, Matrix, Mesh, Rectangle, Transform, Vector2, Vector3, WeakTexture}};
use super::{DrawHandle3D, Material};

/// A raylib model.
///
/// Models contain the following information:
/// - Mesh(es) data
/// - Material(s) data
/// - What material each mesh uses
/// - Transform data
/// - Animation data (bones and bind poses)
/// 
/// To draw a model, you need to create a [`DrawHandle3D`].
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

impl Model {
    /// Returns a reference to this model's transform matrix
    pub fn transform(&self) -> &Matrix {
        &self.0.transform
    }
    /// Returns a mutable reference to this model's transform matrix
    pub fn transform_mut(&mut self) -> &mut Matrix {
        &mut self.0.transform
    }
    /// Returns a slice to this model's meshes 
    pub fn meshes(&self) -> &[Mesh] {
        unsafe { std::slice::from_raw_parts(self.0.meshes as *const Mesh, self.0.meshCount as usize) }
    }
    /// Returns a mutable slice to the model's meshes
    pub fn meshes_mut(&mut self) -> &mut [Mesh] {
        unsafe { std::slice::from_raw_parts_mut(self.0.meshes as *mut Mesh, self.0.meshCount as usize) }
    }
    /// Returns a slice to the model's materials
    pub fn materials(&self) -> &[Material] {
        unsafe { std::slice::from_raw_parts(self.0.materials as *const Material, self.0.materialCount as usize) }
    }
    /// Returns a mutable slice to the model's materials
    pub fn materials_mut(&mut self) -> &mut [Material] {
        unsafe { std::slice::from_raw_parts_mut(self.0.materials as *mut Material, self.0.materialCount as usize) }
    }
    /// Returns a slice to the model's mesh to material map.
    pub fn mesh_materials(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.0.meshMaterial, self.0.meshCount as usize) }
    }
    /// Returns a mutable slice to the model's mesh to material map.
    ///
    /// # Safety
    /// Modified values must be valid indices into the model's material slice.
    pub unsafe fn mesh_materials_mut(&mut self) -> &mut [i32] {
        std::slice::from_raw_parts_mut(self.0.meshMaterial, self.0.meshCount as usize)
    }

    /// Sets the material index of the given mesh
    /// 
    /// # Panics
    /// Panics if `mesh_index` or `material_index` are out of bounds.
    pub fn set_mesh_material(&mut self, mesh_index: usize, material_index: i32) {
        assert!(mesh_index < self.0.meshCount as usize);
        assert!(material_index >= 0 && material_index < self.0.materialCount);

        unsafe {
            self.mesh_materials_mut()[mesh_index] = material_index;
        }
    }

    // TODO: function to get safe bone info

    /// Returns a slice to the model's animation bind poses.
    pub fn bind_poses(&self) -> &[Transform] {
        unsafe { std::slice::from_raw_parts(self.0.bindPose, self.0.boneCount as usize) }
    }
    /// Returns a mutable slice to the model's animation bind poses.
    pub fn bind_poses_mut(&mut self) -> &mut [Transform] {
        unsafe { std::slice::from_raw_parts_mut(self.0.bindPose, self.0.boneCount as usize) }
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
    pub fn billboard(camera: Camera, texture: impl Into<WeakTexture>, pos: Vector3, size: f32, tint: Color) {
        unsafe { ffi::DrawBillboard(camera, *texture.into().get_ffi(), pos, size, tint) }
    }

    /// Draw part of a billboard texture
    pub fn billboard_rec(camera: Camera, texture: impl Into<WeakTexture>, source: Rectangle, pos: Vector3, size: Vector2, tint: Color) {
        unsafe { ffi::DrawBillboardRec(camera, *texture.into().get_ffi(), source, pos, size, tint) }
    }

    /// Draw part of a billboard texture with source and rotation
    /// Angles are in a radians
    pub fn billboard_pro(
        camera: Camera, texture: impl Into<WeakTexture>, source: Rectangle,
        pos: Vector3, up: Vector3, size: Vector2, origin: Vector2, rotation: f32, tint: Color)
    {
        unsafe { ffi::DrawBillboardPro(camera, *texture.into().get_ffi(), source, pos, up, size, origin, rotation.to_degrees(), tint) }
    }
}
