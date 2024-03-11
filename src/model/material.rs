use crate::{ffi, prelude::{Raylib, Shader, MaterialMapIndex, Color, Texture}};

use std::ffi::CStr;

#[derive(Debug)]
pub struct Material<'a>(ffi::Material, std::marker::PhantomData<&'a Shader>);

impl ffi::Material {
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsMaterialReady(*self) }
    }
}

impl<'a> Material<'a> {
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
        mat.is_valid().then(|| Material(mat, std::marker::PhantomData))
    }

    /// Get an immutable reference to a [`ffi::MaterialMap`].
    pub fn get_map(&self, index: MaterialMapIndex) -> &ffi::MaterialMap {
        // SAFETY: index is always in range of the `maps` array
        unsafe { self.0.maps.offset(index as isize).as_ref().unwrap_unchecked() }
    }

    /// Get a mutable reference to a [`ffi::MaterialMap`].
    pub fn get_map_mut(&mut self, index: MaterialMapIndex) -> &mut ffi::MaterialMap {
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

    /// Sets the color of a specified map.
    pub fn set_color(&mut self, index: MaterialMapIndex, color: Color) {
        let map = self.get_map_mut(index);
        map.color = color;
    }

    /// Gets the color of a specified map.
    pub fn get_color(&self, index: MaterialMapIndex) -> Color {
        let map = self.get_map(index);
        map.color
    }

    /// Sets the shader on this material.
    /// 
    /// The shader must live longer than the material.
    pub fn set_shader(&mut self, shader: &'a Shader) {
        self.0.shader = unsafe { *shader.get_ffi() };
    }

    /// Gets a reference to the underlying raylib material struct.
    /// # Safety
    /// The returned object must not be copied or used outside if this object's lifetime, as it still manages its destruction.
    pub unsafe fn get_ffi(&self) -> &ffi::Material {
        &self.0
    }
}

impl Drop for Material<'_> {
    /// manually unload material, knowing its shaders aren't owned
    fn drop(&mut self) {
        const MAX_MATERIAL_MAPS: usize = 12;

        for idx in 0..MAX_MATERIAL_MAPS {
            let map = unsafe { self.0.maps.add(idx).as_mut().unwrap() };
            // don't unload default texture
            if map.texture.id != 1 {
                unsafe { ffi::UnloadTexture(map.texture) }
            }
        }
        unsafe { ffi::MemFree(self.0.maps as *mut _) }
    }
}
