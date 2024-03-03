use std::{ffi::c_void, marker::PhantomData};

use ffi::{CubemapLayout, Rectangle, TextureFilter, TextureWrap, NPatchInfo, PixelFormat};

use crate::{ffi, prelude::{Raylib, DrawHandle}, math::{get_pixel_data_size, Vector2, Color}};

use super::image::Image;

/// A raylib texture.
/// Use `Raylib::load_texture` to create one.
/// 
/// Textures are stored on the GPU in VRAM.
/// If you need to interact with graphical data from the CPU, prefer using an `Image`.
#[repr(C)]
pub struct Texture(ffi::Texture, PhantomData<*const c_void>);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.0) }
    }
}

impl Texture {
    /// Creates a safe texture struct from an ffi texture.
    /// Checks for texture validity and returns `None` if it is not valid.
    #[inline]
    pub fn from_ffi(texture: ffi::Texture) -> Option<Self> {
        if !texture.is_valid() { return None }
        Some(Texture(texture, PhantomData))
    }
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width as u32
    }
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height as u32
    }

    #[inline]
    pub unsafe fn get_ffi_texture(&self) -> ffi::Texture {
        self.0
    }
}

impl ffi::Texture {
    /// Checks if the texture metadata is valid.
    /// Equivalent to `IsTextureReady`.
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsTextureReady(*self) }
    }
}

/// A raylib render texture.
/// Use `Raylib::load_render_texture` to create one.
///
/// Render textures work in the same way as textures, except that you can use them as render targets.
pub struct RenderTexture(ffi::RenderTexture);

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRenderTexture(self.0) }
    }
}

impl RenderTexture {
    /// Creates a safe render texture struct from an ffi render texture.
    /// Checks for texture validity and returns `None` if it is not valid.
    #[inline]
    pub fn from_ffi(texture: ffi::RenderTexture) -> Option<RenderTexture> {
        if !texture.is_valid() { return None };
        Some(RenderTexture(texture))
    }
    /// Get a reference to the color texture associated to this render texture.
    #[inline]
    pub fn texture<'a>(&'a self) -> &'a Texture {
        // SAFETY: the returned texture is bound to the lifetime of the RenderTexture, and `Texture` has the same layout as `ffi::Texture`.
        unsafe { std::mem::transmute(&self.0.texture) }
    }
    /// Get a mutable reference to the color texture associated to this render texture.
    #[inline]
    pub fn texture_mut<'a>(&'a mut self) -> &'a mut Texture {
        // SAFETY: the returned texture is bound to the lifetime of the RenderTexture, and `Texture` has the same layout as `ffi::Texture`.
        unsafe { std::mem::transmute(&mut self.0.texture) }
    }
    /// Get a reference to the depth texture associated to this render texture.
    #[inline]
    pub fn depth<'a>(&'a self) -> &'a Texture {
        // SAFETY: the returned texture is bound to the lifetime of the RenderTexture, and `Texture` has the same layout as `ffi::Texture`.
        unsafe { std::mem::transmute(&self.0.depth) }
    }
    /// Get a mutable reference to the depth texture associated to this render texture.
    #[inline]
    pub fn depth_mut<'a>(&'a mut self) -> &'a mut Texture {
        // SAFETY: the returned texture is bound to the lifetime of the RenderTexture, and `Texture` has the same layout as `ffi::Texture`.
        unsafe { std::mem::transmute(&mut self.0.depth) }
    }

    #[inline]
    pub unsafe fn get_ffi_texture(&self) -> ffi::RenderTexture {
        self.0
    }
}

impl ffi::RenderTexture {
    /// Checks if the texture metadata is valid.
    /// Equivalent to `IsRenderTextureReady`.
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsRenderTextureReady(*self) }
    }
}

/// # Texture loading functions
/// 
/// ---
impl Texture {
    /// Loads a texture from a file.
    /// Returns `Err` if there was an error when reading the file.
    /// Returns `Ok(None)` if the file was successfully read,
    /// but support for the given file extension was not compiled into raylib,
    /// or the input file is in an unknown file format.
    /// Otherwise, returns the loaded texture.
    pub fn load(rl: &mut Raylib, filename: impl AsRef<std::path::Path>) -> std::io::Result<Option<Texture>> {
        let image = Image::load(rl, filename)?;
        if let Some(image) = image {
            Ok(Self::load_from_image(rl, &image))
        } else { Ok(None) }
    }

    /// Load texture from image data.
    /// Returns `None` if there was an error loading the texture.
    #[inline]
    pub fn load_from_image(_rl: &mut Raylib, image: &Image) -> Option<Texture> {
        let texture = unsafe { ffi::LoadTextureFromImage(image.get_ffi_image()) };
        Texture::from_ffi(texture)
    }

    /// Loads an empty texture in the given format.
    /// Returns `None` if there was an error creating the texture.
    pub fn load_empty(_rl: &mut Raylib, width: u32, height: u32, format: PixelFormat) -> Option<Texture> {
        let empty_image = ffi::Image {
            data: std::ptr::null_mut(),
            width: width as i32, height: height as i32,
            format: format as i32,
            mipmaps: 1
        };

        let texture = unsafe { ffi::LoadTextureFromImage(empty_image) };
        Texture::from_ffi(texture)
    }

    /// Load a cubemap texture from an image.
    /// Returns `None` if there was an error loading the texture.
    #[inline]
    pub fn load_cubmap(_rl: &mut Raylib, image: &Image, layout: CubemapLayout) -> Option<Texture> {
        let texture = unsafe { ffi::LoadTextureCubemap(image.get_ffi_image(), layout as i32) };
        Texture::from_ffi(texture)
    }
}

impl RenderTexture {
    /// Create a render texture of the given size.
    /// Returns `None` if there was an error when loading the texture.
    #[inline]
    pub fn load(_rl: &mut Raylib, width: u32, height: u32) -> Option<RenderTexture> {
        let texture = unsafe { ffi::LoadRenderTexture(width as i32, height as i32) };
        RenderTexture::from_ffi(texture)
    }
}

impl Texture {
    /// Updates the texture with the given image.
    /// Returns `Err(())` if the image's format or size does not correspond to the texture.
    pub fn update(&mut self, image: &Image) -> Result<(), ()> {
        if self.0.format != image.format() as i32 { return Err(()) }
        if self.0.width as u32 != image.width() || self.0.height as u32 != image.height() { return Err(()) }

        unsafe { ffi::UpdateTexture(self.0, image.get_ffi_image().data) };
        Ok(())
    }

    /// Updates the texture with the given raw image data.
    /// The data must be in the same format as the texture.
    /// Returns `Err(())` if the buffer's size does not correspond to the texture.
    pub fn update_raw(&mut self, buffer: &[u8]) -> Result<(), ()> {
        if get_pixel_data_size(self.0.width, self.0.height, self.0.format.try_into().unwrap()) as usize != buffer.len() { return Err(()) }

        unsafe { ffi::UpdateTexture(self.0, buffer.as_ptr() as *const c_void) };
        Ok(())
    }

    /// Updates part of a texture with the given image.
    /// Returns `Err(())` if:
    /// - The image's format does not correspond to the texture
    /// - The image's size does not correspond to `rec`.
    /// - `rec` is out of texture bounds.
    pub fn update_rec(&mut self, rec: Rectangle, image: &Image) -> Result<(), ()> {
        if self.0.format != image.format() as i32 { return Err(()) }
        if rec.width as u32 != image.width() || rec.height as u32 != image.height() { return Err(()) }
        if rec.x < 0.0 || rec.y < 0.0 || (rec.x + rec.width) as u32 > self.0.width as u32 || (rec.y + rec.height) as u32 >= self.0.height as u32 {
            return Err(())
        }

        unsafe { ffi::UpdateTextureRec(self.0, rec, image.get_ffi_image().data) };
        Ok(())
    }

    /// Updates part of a texture with the given raw image data.
    /// The data must be in the same format as the texture.
    /// Returns `Err(())` if:
    /// - The buffer's size does not correspond to `rec`.
    /// - `rec` is out of texture bounds.
    pub fn update_rec_raw(&mut self, rec: Rectangle, buffer: &[u8]) -> Result<(), ()> {
        if get_pixel_data_size(rec.width as i32, rec.height as i32, self.0.format.try_into().unwrap()) as usize != buffer.len() { return Err(()) }
        if rec.x < 0.0 || rec.y < 0.0 || (rec.x + rec.width) as u32 > self.0.width as u32 || (rec.y + rec.height) as u32 >= self.0.height as u32 {
            return Err(())
        }

        unsafe { ffi::UpdateTextureRec(self.0, rec, buffer.as_ptr() as *const c_void) };
        Ok(())
    }
}

/// # Texture configuration functions
/// 
/// ---
impl Texture {
    /// Generate GPU mipmaps for a texture.
    /// Most of the time, you should do this instead of generating mipmaps for `Image`s.
    #[inline]
    pub fn gen_texture_mipmaps(&mut self) {
        unsafe { ffi::GenTextureMipmaps(&mut self.0 as *mut _) }
    }
    /// Set texture scaling filter mode
    #[inline]
    pub fn set_texture_filter(&mut self, filter: TextureFilter) {
        unsafe { ffi::SetTextureFilter(self.0, filter as i32) }
    }
    /// Set texture wrapping mode
    #[inline]
    pub fn set_texture_wrap(&mut self, wrap: TextureWrap) {
        unsafe { ffi::SetTextureWrap(self.0, wrap as i32) }
    }
}

/// # Texture drawing functions
/// 
/// ---
impl<P> DrawHandle<'_, P> {
    /// Draw a texture.
    #[inline]
    pub fn texture(&mut self, texture: &Texture, x: f32, y: f32, tint: Color) {
        self.texture_ex(texture, Vector2::new(x, y), 0.0, 1.0, tint);
    }
    /// Draw a texture.
    #[inline]
    pub fn texture_v(&mut self, texture: &Texture, pos: Vector2, tint: Color) {
        self.texture_ex(texture, pos, 0.0, 1.0, tint);
    }
    /// Draw a rotated and scaled texture.
    /// The rotation is in radians.
    #[inline]
    pub fn texture_ex(&mut self, texture: &Texture, pos: Vector2, rotation: f32, scale: f32, tint: Color) {
        let source = Rectangle::new(0.0, 0.0, texture.0.width as f32, texture.0.height as f32);
        let dest = Rectangle::new(pos.x, pos.y, texture.0.width as f32 * scale, texture.0.height as f32 * scale);
        self.texture_pro(texture, source, dest, Vector2::ZERO, rotation, tint)
    }
    /// Draw part of a texture.
    #[inline]
    pub fn texture_rec(&mut self, texture: &Texture, source: Rectangle, pos: Vector2, tint: Color) {
        let dest = Rectangle::new(pos.x, pos.y, source.width, source.height);
        self.texture_pro(texture, source, dest, Vector2::ZERO, 0.0, tint)
    }
    /// Draw part of a texture to a part of the screen, rotated around the given origin point.
    /// Origin is **relative** to the destination rectangle.
    /// The rotation is in radians.
    #[inline]
    pub fn texture_pro(&mut self, texture: &Texture, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        unsafe { ffi::DrawTexturePro(texture.0, source, dest, origin, rotation.to_degrees(), tint) }
    }
    /// Draws a texture that stretches and shrinks using n-patch info.
    /// The rotation is in radians.
    #[inline]
    pub fn texture_npatch(&mut self, texture: &Texture, info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        unsafe { ffi::DrawTextureNPatch(texture.0, info, dest, origin, rotation.to_degrees(), tint) }
    }
}
