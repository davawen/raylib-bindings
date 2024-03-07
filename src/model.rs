use std::ffi::CStr;

use ffi::{Camera3D, MaterialMapIndex};

use crate::{prelude::{Ray, Vector2, Vector3, Color, DrawHandle, Raylib, Texture}, ffi};

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

impl DrawHandle3D {
    /// Draw a line in 3D world space.
    #[inline]
    pub fn line(&mut self, start: Vector3, end: Vector3, color: Color) {
        unsafe { ffi::DrawLine3D(start, end, color) }
    }
    /// Draw a point in 3D space.
    #[inline]
    pub fn point(&mut self, p: Vector3, color: Color) {
        unsafe { ffi::DrawPoint3D(p, color) }
    }
    /// Draw a circle in 3D space.
    #[inline]
    pub fn circle(&mut self, center: Vector3, radius: f32, rotation_axis: Vector3, rotation_angle: f32, color: Color) {
        unsafe { ffi::DrawCircle3D(center, radius, rotation_axis, rotation_angle, color) }
    }
    /// Draw a color filled triangle in 3D space.
    #[inline]
    pub fn triangle(&mut self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { ffi::DrawTriangle3D(v1, v2, v3, color) }
    }
    /// Draw a color filled triangle strip in 3D space.
    #[inline]
    pub fn triangle_strip(&mut self, points: &[Vector3], color: Color) {
        // NOTE: Always the same note about the safety of cast_mut: the points aren't modified but the API forgot to put a `const`.
        unsafe { ffi::DrawTriangleStrip3D(points.as_ptr().cast_mut(), points.len() as i32, color) }
    }
    /// Draw a cube in 3D space.
    #[inline]
    pub fn cube(&mut self, pos: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCube(pos, width, height, length, color) }
    }
    /// Draw a cube in 3D space.
    #[inline]
    pub fn cube_v(&mut self, pos: Vector3, size: Vector3, color: Color) {
        self.cube(pos, size.x, size.y, size.z, color)
    }
    /// Draw a wireframe cube in 3D space.
    #[inline]
    pub fn cube_wires(&mut self, pos: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCubeWires(pos, width, height, length, color) }
    }
    /// Draw a wireframe cube in 3D space.
    #[inline]
    pub fn cube_wires_v(&mut self, pos: Vector3, size: Vector3, color: Color) {
        self.cube_wires(pos, size.x, size.y, size.z, color)
    }
    /// Draw a sphere in 3D space.
    #[inline]
    pub fn sphere(&mut self, center: Vector3, radius: f32, color: Color) {
        self.sphere_ex(center, radius, 16, 16, color);
    }
    /// Draw a sphere in 3D space.
    #[inline]
    pub fn sphere_ex(&mut self, center: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
        unsafe { ffi::DrawSphereEx(center, radius, rings, slices, color) }
    }
    /// Draw a wireframe sphere in 3D space.
    #[inline]
    pub fn sphere_wires(&mut self, center: Vector3, radius: f32, color: Color) {
        self.sphere_wires_ex(center, radius, 16, 16, color)
    }
    /// Draw a wireframe sphere in 3D space.
    #[inline]
    pub fn sphere_wires_ex(&mut self, center: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
        unsafe { ffi::DrawSphereWires(center, radius, rings, slices, color) }
    }
    /// Draw a cylinder/cone in 3D space.
    /// It is centered at the given position.
    #[inline]
    pub fn cylinder(&mut self, pos: Vector3, radius_top: f32, radius_bot: f32, height: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinder(pos, radius_top, radius_bot, height, slices, color) }
    }
    /// Draw a cylinder/cone in 3D space.
    #[inline]
    pub fn cylinder_ex(&mut self, start: Vector3, end: Vector3, radius_start: f32, radius_end: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderEx(start, end, radius_start, radius_end, slices, color) }
    }
    /// Draw a wireframe cylinder/cone in 3D space.
    /// It is centered at the given position.
    #[inline]
    pub fn cylinder_wires(&mut self, pos: Vector3, radius_top: f32, radius_bot: f32, height: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderWires(pos, radius_top, radius_bot, height, slices, color) }
    }
    /// Draw a wireframe cylinder/cone in 3D space.
    #[inline]
    pub fn cylinder_wires_ex(&mut self, start: Vector3, end: Vector3, radius_start: f32, radius_end: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderWiresEx(start, end, radius_start, radius_end, slices, color) }
    }
    /// Draw a capsule with the center of its sphere caps at `start` and `end`.
    #[inline]
    pub fn capsule(&mut self, start: Vector3, end: Vector3, radius: f32, slices: i32, rings: i32, color: Color) {
        unsafe { ffi::DrawCapsule(start, end, radius, slices, rings, color) }
    }
    /// Draw a wireframe capsule with the center of its sphere caps at `start` and `end`.
    #[inline]
    pub fn capsule_wires(&mut self, start: Vector3, end: Vector3, radius: f32, slices: i32, rings: i32, color: Color) {
        unsafe { ffi::DrawCapsuleWires(start, end, radius, slices, rings, color) }
    }
    /// Draws an XZ plane (with its normal pointing in the Y direction).
    #[inline]
    pub fn plane(&mut self, pos: Vector3, size: Vector2, color: Color) {
        unsafe { ffi::DrawPlane(pos, size, color) }
    }
    /// Draws a ray as a line.
    #[inline]
    pub fn ray(&mut self, ray: Ray, color: Color) {
        unsafe { ffi::DrawRay(ray, color) }
    }
    /// Draws a grid centered at (0, 0, 0)
    #[inline]
    pub fn grid(&mut self, slices: i32, spacing: f32) {
        unsafe { ffi::DrawGrid(slices, spacing) }
    }
}

pub struct Model(ffi::Model);

pub struct Mesh(ffi::Mesh);

#[derive(Debug)]
pub struct Material(ffi::Material);

impl ffi::Material {
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsMaterialReady(*self) }
    }
}

impl Material {
    /// Loads all materials from a model (`.mtl`) file.
    /// Returns `None` if raylib was not compiled with `mtl` support, if the given file is not an `mtl` file,
    /// or if the given file is invalid.
    pub fn load_multiple(path: impl AsRef<std::path::Path>) -> Option<Vec<Self>> {
        let cstr = std::ffi::CString::new(path.as_ref().as_os_str().as_encoded_bytes()).unwrap();
        Material::load_multiple_cstr(&cstr)
    }

    /// Loads all materials from a model (`.mtl`) file.
    /// Returns `None` if raylib was not compiled with `mtl` support, if the given file is not an `mtl` file,
    /// or if the given file is invalid.
    pub fn load_multiple_cstr(filename: &CStr) -> Option<Vec<Self>> {
        // load ffi materials
        let mut count = 0;
        let ptr = unsafe { ffi::LoadMaterials(filename.as_ptr(), &mut count as *mut _) };

        if ptr.is_null() || count == 0 { return None }

        // copy slice contents to rust vector
        let slice = unsafe { std::slice::from_raw_parts(ptr, count as usize) };
        let materials = slice.iter().map(|m| Material::from_ffi(*m)).collect();

        // free raylib memory
        unsafe { ffi::MemFree(ptr as *mut _) }

        materials
    }

    /// Load default material (supports: `MaterialMapIndex::{Albedo, Metalness, Normal}`).
    /// 
    /// # Examples
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// let mut mat = Material::load_default();
    /// ```
    /// # Panics
    /// Panics if the struct generated by raylib is invalid
    pub fn load_default(_: &mut Raylib) -> Self {
        let mat = unsafe { ffi::LoadMaterialDefault() };
        Material::from_ffi(mat).expect("expected a valid material")
    }

    /// Creates a safe material wrapper from an ffi struct.
    /// 
    /// Returns `None` if the material is not valid.
    pub fn from_ffi(mat: ffi::Material) -> Option<Self> {
        mat.is_valid().then(|| Material(mat))
    }

    fn get_map(&self, index: MaterialMapIndex) -> &ffi::MaterialMap {
        // SAFETY: index is always in range of the `maps` array
        unsafe { self.0.maps.offset(index as isize).as_ref().unwrap_unchecked() }
    }

    fn get_map_mut(&mut self, index: MaterialMapIndex) -> &mut ffi::MaterialMap {
        // SAFETY: index is always in range of the `maps` array
        unsafe { self.0.maps.offset(index as isize).as_mut().unwrap_unchecked() }
    }

    /// Sets the texture for a material map kind.
    /// 
    /// The textures are moved into the material and unloaded by it.
    /// If you wish to modify them, use [`Material::get_texture_mut`].
    /// If you wish to get them back, use [`Material::take_texture`].
    pub fn set_texture(&mut self, index: MaterialMapIndex, texture: Texture, color: Color) {
        unsafe { ffi::SetMaterialTexture(&mut self.0 as *mut _, index as i32, texture.unwrap()) };
        self.get_map_mut(index).color = color;
    }

    /// Gets a reference to a texture used in the material.
    ///
    /// Returns `None` if no texture were set for the specified map index.
    /// Unlike [`Material::get_texture_mut`], this function returns `Some` if the default texture is set at the specified index.
    pub fn get_texture(&self, index: MaterialMapIndex) -> Option<&Texture> {
        let map = self.get_map(index);
        if !map.texture.is_valid() { return None }

        // SAFETY: `Texture` has the same in-memory representation as `ffi::Texture`, and its lifetime is tied to self.
        // SAFETY: The texture has been checked to be valid
        Some(unsafe { std::mem::transmute(&map.texture) })
    }

    /// Gets a mutable reference to a texture used in the material.
    /// 
    /// Returns `None` if no textures were set for the specified map index, or if the set texture is the default one (we can't modify it).
    pub fn get_texture_mut(&mut self, index: MaterialMapIndex) -> Option<&mut Texture> {
        let map = self.get_map_mut(index);
        if !map.texture.is_valid() { return None }
        // Check that it isn't the default texture
        if map.texture.id == 1 { return None }

        // SAFETY: `Texture` has the same in-memory representation as `ffi::Texture`, and its lifetime is tied to self.
        // SAFETY: The texture has been checked to be valid
        // SAFETY: We aren't returning the default texture
        Some(unsafe { std::mem::transmute(&mut map.texture) })
    }

    /// Get a texture back from a material, replacing it with `None`.
    /// 
    /// Returns `None` if no textures were set for the specified map index, or if the set texture is the default one.
    pub fn take_texture(&mut self, index: MaterialMapIndex) -> Option<Texture> {
        let map = self.get_map_mut(index);
        if !map.texture.is_valid() { return None }
        // Check that it isn't the default texture
        if map.texture.id == 1 { return None }

        let texture = Texture::from_ffi(map.texture);
        map.texture = ffi::Texture { id: 0, width: 0, height: 0, format: 0, mipmaps: 0 };
        texture
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe { ffi::UnloadMaterial(self.0) }
    }
}
