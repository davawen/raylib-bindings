use std::{ffi::c_void, marker::PhantomData};

use ffi::{CubemapLayout, Rectangle, TextureFilter, TextureWrap, NPatchInfo, PixelFormat};

use crate::{ffi, prelude::{Raylib, DrawHandle, get_pixel_data_size, Vector2, Color, vec2}};

use super::image::Image;

/// A raylib texture.
/// Use `Raylib::load_texture` to create one.
/// 
/// Textures are stored on the GPU in VRAM.
/// If you need to interact with graphical data from the CPU, prefer using an `Image`.
#[repr(transparent)]
#[derive(Debug)]
pub struct Texture(ffi::Texture, PhantomData<*const c_void>);

/// A weak reference to a raylib texture.
/// Used when we need long-time read-only access to a texture, while allowing modifications to it, such as in [`crate::model::Material`].
/// This reference offers no garantee that the texture will live for its whole lifetime.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct WeakTexture(ffi::Texture);

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
    pub fn weak(&self) -> WeakTexture {
        WeakTexture(self.0)
    }

    /// Consumes self and returns the internal `ffi::Texture` without freeing it
    #[inline]
    pub unsafe fn unwrap(self) -> ffi::Texture {
        let this = std::mem::ManuallyDrop::new(self);
        this.0
    }

    /// Get a reference to the underlying ffi texture type
    #[inline]
    pub fn get_ffi(&self) -> &ffi::Texture {
        &self.0
    }
}

impl WeakTexture {
    /// Consumes self and returns the internal `ffi::Texture`
    #[inline]
    pub unsafe fn unwrap(self) -> ffi::Texture {
        self.0
    }

    /// Get a reference to the underlying ffi texture type
    #[inline]
    pub fn get_ffi(&self) -> &ffi::Texture {
        &self.0
    }
}

impl Into<WeakTexture> for &Texture {
    fn into(self) -> WeakTexture {
        self.weak()
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
    pub fn update(&self, image: &Image) -> Result<(), ()> {
        if self.0.format != image.format() as i32 { return Err(()) }
        if self.0.width as u32 != image.width() || self.0.height as u32 != image.height() { return Err(()) }

        unsafe { ffi::UpdateTexture(self.0, image.get_ffi_image().data) };
        Ok(())
    }

    /// Updates the texture with the given raw image data.
    /// The data must be in the same format as the texture.
    /// Returns `Err(())` if the buffer's size does not correspond to the texture.
    pub fn update_raw(&self, buffer: &[u8]) -> Result<(), ()> {
        if get_pixel_data_size(self.0.width, self.0.height, self.0.format.try_into().unwrap()) as usize != buffer.len() { return Err(()) }

        unsafe { ffi::UpdateTexture(self.0, buffer.as_ptr() as *const c_void) };
        Ok(())
    }

    /// Updates part of a texture with the given image.
    /// Returns `Err(())` if:
    /// - The image's format does not correspond to the texture
    /// - The image's size does not correspond to `rec`.
    /// - `rec` is out of texture bounds.
    pub fn update_rec(&self, rec: Rectangle, image: &Image) -> Result<(), ()> {
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
    pub fn update_rec_raw(&self, rec: Rectangle, buffer: &[u8]) -> Result<(), ()> {
        if get_pixel_data_size(rec.width as i32, rec.height as i32, self.0.format.try_into().unwrap()) as usize != buffer.len() { return Err(()) }
        if rec.x < 0.0 || rec.y < 0.0 || (rec.x + rec.width) as u32 > self.0.width as u32 || (rec.y + rec.height) as u32 >= self.0.height as u32 {
            return Err(())
        }

        unsafe { ffi::UpdateTextureRec(self.0, rec, buffer.as_ptr() as *const c_void) };
        Ok(())
    }

    /// Resizes this texture to new dimensions, losing image data in the process. 
    /// # Safety
    /// Since [`Texture`]s are neither `Send` nor `Sync`, if the current texture was well created,
    /// it should be living on the main thread.
    /// This means it does not have to take a [`Raylib`] parameter.
    /// Note that this function is not safe to be called from any context other than the main thread.
    pub fn resize_lossy(&mut self, new_width: u32, new_height: u32) {
        unsafe { ffi::UnloadTexture(self.0) };

        let empty_image = ffi::Image {
            data: std::ptr::null_mut(),
            width: new_width as i32, height: new_height as i32,
            format: self.0.format as i32,
            mipmaps: 1
        };

        let texture = unsafe { ffi::LoadTextureFromImage(empty_image) };
        *self = Texture::from_ffi(texture).unwrap();
    }

    /// Resizes this texture using the bicubic scaling algorithm. 
    /// See [`Image::resize_bicubic`] for limitations.
    /// # Safety
    /// Since [`Texture`]s are neither `Send` nor `Sync`, if the current texture was well created,
    /// it should be living on the main thread.
    /// This means it does not have to take a [`Raylib`] parameter.
    /// Note that this function is not safe to be called from any context other than the main thread.
    pub fn resize_bicubic(&mut self, new_width: u32, new_height: u32) {
        let mut image = unsafe { ffi::LoadImageFromTexture(self.0) };
        unsafe { ffi::ImageResize(&mut image as *mut _, new_width as i32, new_height as i32) };

        let texture = unsafe { ffi::LoadTextureFromImage(image) };
        *self = Texture::from_ffi(texture).unwrap();
    }

    /// Resizes this texture using the nearest neighbour algorithm. 
    /// See [`Image::resize_nn`] for limitations.
    /// # Safety
    /// Since [`Texture`]s are neither `Send` nor `Sync`, if the current texture was well created,
    /// it should be living on the main thread.
    /// This means it does not have to take a [`Raylib`] parameter.
    /// Note that this function is not safe to be called from any context other than the main thread.
    pub fn resize_nn(&mut self, new_width: u32, new_height: u32) {
        let mut image = unsafe { ffi::LoadImageFromTexture(self.0) };
        unsafe { ffi::ImageResizeNN(&mut image as *mut _, new_width as i32, new_height as i32) };

        let texture = unsafe { ffi::LoadTextureFromImage(image) };
        *self = Texture::from_ffi(texture).unwrap();
    }

    /// Crops part of the texture and fills out of bounds part with the given color.
    /// See [`Image::resize_canvas`] for limitations.
    /// # Safety
    /// Since [`Texture`]s are neither `Send` nor `Sync`, if the current texture was well created,
    /// it should be living on the main thread.
    /// This means it does not have to take a [`Raylib`] parameter.
    /// Note that this function is not safe to be called from any context other than the main thread.
    pub fn resize_canvas(&mut self, new_width: u32, new_height: u32, offset_x: i32, offset_y: i32, fill: Color) {
        let mut image = unsafe { ffi::LoadImageFromTexture(self.0) };
        unsafe { ffi::ImageResizeCanvas(&mut image as *mut _, new_width as i32, new_height as i32, offset_x, offset_y, fill) };

        let texture = unsafe { ffi::LoadTextureFromImage(image) };
        *self = Texture::from_ffi(texture).unwrap();
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

/// Draw a texture.
#[inline]
pub fn draw_texture(rl: &DrawHandle, texture: impl Into<WeakTexture>, x: f32, y: f32, tint: Color) {
    draw_texture_ex(rl, texture, vec2(x, y), 0.0, 1.0, tint);
}
/// Draw a texture.
#[inline]
pub fn draw_texture_v(rl: &DrawHandle, texture: impl Into<WeakTexture>, pos: Vector2, tint: Color) {
    draw_texture_ex(rl, texture, pos, 0.0, 1.0, tint);
}
/// Draw a rotated and scaled texture.
/// The rotation is in radians.
#[inline]
pub fn draw_texture_ex(rl: &DrawHandle, texture: impl Into<WeakTexture>, pos: Vector2, rotation: f32, scale: f32, tint: Color) {
    let texture = texture.into();
    let source = Rectangle::new(0.0, 0.0, texture.0.width as f32, texture.0.height as f32);
    let dest = Rectangle::new(pos.x, pos.y, texture.0.width as f32 * scale, texture.0.height as f32 * scale);
    draw_texture_pro(rl, texture, source, dest, Vector2::ZERO, rotation, tint)
}
/// Draw part of a texture.
#[inline]
pub fn draw_texture_rec(rl: &DrawHandle, texture: impl Into<WeakTexture>, source: Rectangle, pos: Vector2, tint: Color) {
    let dest = Rectangle::new(pos.x, pos.y, source.width, source.height);
    draw_texture_pro(rl, texture, source, dest, Vector2::ZERO, 0.0, tint)
}
/// Draw part of a texture to a part of the screen, rotated around the given origin point.
/// Origin is **relative** to the destination rectangle.
/// The rotation is in radians.
#[inline]
pub fn draw_texture_pro(_rl: &DrawHandle, texture: impl Into<WeakTexture>, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
    unsafe { ffi::DrawTexturePro(texture.into().0, source, dest, origin, rotation.to_degrees(), tint) }
}
/// Draws a texture that stretches and shrinks using n-patch info.
/// The rotation is in radians.
#[inline]
pub fn draw_texture_npatch(_rl: &DrawHandle, texture: impl Into<WeakTexture>, info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
    unsafe { ffi::DrawTextureNPatch(texture.into().0, info, dest, origin, rotation.to_degrees(), tint) }
}
