use std::ffi::CStr;

use ffi::PixelFormat;

use crate::{ffi, prelude::Raylib, cstr, math::color::get_pixel_data_size};

/// A raylib texture.
/// Use `Raylib::load_texture` to create one.
/// 
/// Textures are stored on the GPU in VRAM.
/// If you need to interact with graphical data from the CPU, prefer using an `Image`.
pub struct Texture(ffi::Texture);

/// A raylib render texture.
/// Use `Raylib::load_render_texture` to create one.
///
/// Render textures work in the same way as textures, except that you can use them as render targets.
pub struct RenderTexture(ffi::RenderTexture);

/// A raylib image.
/// Use `Raylib::load_image` to create one.
/// 
/// Images are stored on the CPU in RAM.
/// If you need to draw images many times, or if you need to use them in shaders, prefer using a `Texture`.
pub struct Image(ffi::Image, usize);

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadTexture(self.0) }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRenderTexture(self.0) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.0) }
    }
}

/// Defines image filetypes supported by raylib.
pub enum ImageFiletype {
    Png,
    Bmp,
    Tga,
    Jpg,
    Gif,
    Pic,
    Ppm,
    Pgm,
    Psd,
    Hdr,
    Qoi,
    Svg,
    Dds,
    Pkm,
    Ktx,
    Pvr,
    Astc
}

impl ImageFiletype {
    /// Returns the extension corresponding to this filetype as a `&CStr`.
    /// The resulting string contains a leading dot and is lowercase.
    pub fn extension(&self) -> &'static CStr {
        match self {
            ImageFiletype::Png => cstr!(".png"),
            ImageFiletype::Bmp => cstr!(".bmp"),
            ImageFiletype::Tga => cstr!(".tga"),
            ImageFiletype::Jpg => cstr!(".jpg"),
            ImageFiletype::Gif => cstr!(".gif"),
            ImageFiletype::Pic => cstr!(".pic"),
            ImageFiletype::Ppm => cstr!(".ppm"),
            ImageFiletype::Pgm => cstr!(".pgm"),
            ImageFiletype::Psd => cstr!(".psd"),
            ImageFiletype::Hdr => cstr!(".hdr"),
            ImageFiletype::Qoi => cstr!(".qoi"),
            ImageFiletype::Svg => cstr!(".svg"),
            ImageFiletype::Dds => cstr!(".dds"),
            ImageFiletype::Pkm => cstr!(".pkm"),
            ImageFiletype::Ktx => cstr!(".ktx"),
            ImageFiletype::Pvr => cstr!(".pvr"),
            ImageFiletype::Astc => cstr!(".astc"),
        }
    }
}

/// # Image loading functions
/// 
/// ---
impl Raylib {
    /// Loads an image from a file into CPU memory.
    /// Returns `Err` if there was an error when reading the file.
    /// Returns `Some(None)` if the file was successfully read,
    /// but support for the given file extension was not compiled into raylib,
    /// or the input file is in an unknown file format.
    /// Otherwise, returns the loaded image.
    pub fn load_image(&mut self, filename: impl AsRef<std::path::Path>) -> std::io::Result<Option<Image>> {
        if !filename.as_ref().exists() { return Err(std::io::ErrorKind::NotFound.into()) }

        let filetype = match filename.as_ref().extension().map(|s| s.to_str()).flatten() {
            Some(".png") => ImageFiletype::Png,
            Some(".bmp") => ImageFiletype::Bmp,
            Some(".tga") => ImageFiletype::Tga,
            Some(".jpg") => ImageFiletype::Jpg,
            Some(".gif") => ImageFiletype::Gif,
            Some(".pic") => ImageFiletype::Pic,
            Some(".ppm") => ImageFiletype::Ppm,
            Some(".pgm") => ImageFiletype::Pgm,
            Some(".psd") => ImageFiletype::Psd,
            Some(".hdr") => ImageFiletype::Hdr,
            Some(".qoi") => ImageFiletype::Qoi,
            Some(".svg") => ImageFiletype::Svg,
            Some(".dds") => ImageFiletype::Dds,
            Some(".pkm") => ImageFiletype::Pkm,
            Some(".ktx") => ImageFiletype::Ktx,
            Some(".pvr") => ImageFiletype::Pvr,
            Some(".astc") => ImageFiletype::Astc,
            _ => return Ok(None)
        };

        let file = std::fs::read(filename)?;
        Ok(self.load_image_from_memory(filetype, &file))
    }

    /// Loads an image from a memory buffer containing raw data.
    /// Returns `None` if `data`'s size doesn't correspond to the width, height, and format of the image.
    /// This is a reimplementation of the `LoadImageRaw` raylib function, to allow loading a raw image from a memory buffer.
    /// It doesn't take a `headerSize` parameter since you can just index the slice to remove it.
    /// # Panics
    /// Panics if width or height are less than 1.
    /// # Examples
    /// Load a raw image from a file:
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "load image raw", 60);
    /// let data = std::fs::read("assets/raw_image.raw").unwrap();
    /// let rgb = rl.load_image_raw(&data, 3, 3, PixelFormat::UncompressedR8G8B8).unwrap();
    /// let rgb = rl.load_texture_from_image(&rgb);
    /// let mut draw = rl.begin_drawing();
    /// draw.draw_texture(rgb, 0.0, 0.0, Color::WHITE);
    /// ```
    pub fn load_image_raw(&mut self, data: &[u8], width: i32, height: i32, format: PixelFormat) -> Option<Image> {
        assert!(width > 0 && height > 0);

        let size = unsafe { ffi::GetPixelDataSize(width, height, format as i32) };
        if data.len() != size as usize { return None }

        let data_ptr = unsafe { ffi::MemAlloc(size as u32) };
        if data_ptr.is_null() { return None }

        // SAFETY: `data.len()` and `size` are equal
        // data_ptr is not null
        // the range [data_ptr, data_ptr+size] is valid for read/write
        let slice = unsafe { std::slice::from_raw_parts_mut(data_ptr as *mut u8, size as usize) };
        slice.copy_from_slice(data);
        let image = ffi::Image {
            data: data_ptr,
            width, height,
            format: format as i32,
            mipmaps: 1
        };

        Some(Image(image, size as usize))
    }

    /// Loads an image from a memory buffer (in the given filetype's encoding).
    /// Returns `None` if support for the given format was not compiled into raylib.
    /// If you need to create an image from raw data (no filetype), use `load_image_raw`.
    pub fn load_image_from_memory(&mut self, filetype: ImageFiletype, data: &[u8]) -> Option<Image> {
        let filetype = filetype.extension();

        let image = unsafe { ffi::LoadImageFromMemory(filetype.as_ptr(), data.as_ptr(), data.len() as i32) };

        Image::from_ffi(image)
    }

    /// Loads animated image data.
    /// Returns `None` if there was an error when loading the image.
    /// - The resulting `data` buffer includes all frames next to one another.
    /// - The number of frames is returned.
    /// - All frames are in RGBA format.
    /// - Only the GIF filetype is supported (other filetypes will load a single image).
    /// - Frames delay data is discarded.
    pub fn load_image_anim_from_memory(&mut self, filetype: ImageFiletype, data: &[u8]) -> Option<(usize, Image)> {
        let filetype = filetype.extension();
        let mut frames = 0;
        let image = unsafe { ffi::LoadImageAnimFromMemory(filetype.as_ptr(), data.as_ptr(), data.len() as i32, &mut frames as *mut i32) };
        let image = Image::from_ffi(image)?;

        let frames = frames as usize;
        Some((frames, image))
    }

    /// Loads image from GPU texture data.
    /// Compressed format are not supported.
    /// Returns `None` if the texture is in a compressed format or if there was an error when reading the data in the texture.
    pub fn load_image_from_texture(&mut self, texture: &Texture) -> Option<Image> {
        let image = unsafe { ffi::LoadImageFromTexture(texture.get_ffi_texture()) };
        Image::from_ffi(image)
    }

    /// Loads an image from the current screen buffer (= take a screenshot)
    /// # Panics
    /// This function panics if there was an error reading the screen buffer data.
    pub fn load_image_from_screen(&mut self) -> Image {
        let image = unsafe { ffi::LoadImageFromScreen() };

        Image::from_ffi(image).unwrap()
    }
}

impl Texture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::Texture {
        self.0
    }
}

impl RenderTexture {
    pub unsafe fn get_ffi_texture(&self) -> ffi::RenderTexture {
        self.0
    }
}

impl ffi::Image {
    /// Checks if the underlying image metadata is valid.
    /// Equivalent to the not so well named `IsImageReady` raylib function.
    /// Prefer using this instead of checking for a null image data pointer.
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::IsImageReady(*self) }
    }
}

impl Image {
    /// Create a safe image struct from an ffi image by calculating its size.
    /// Checks for image validity and returns `None` if the image is not valid.
    pub fn from_ffi(image: ffi::Image) -> Option<Self> {
        if !image.is_valid() { return None }

        Some(Image(image, get_pixel_data_size(image.width, image.height, image.format.try_into().unwrap()) as usize))
    }

    /// Get access to the underlying image data buffer
    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.data as *const u8, self.1) }
    }

    /// Get mutable access to the underlying image data buffer
    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0.data as *mut u8, self.1) }
    }

    /// Size in bytes of the image data buffer
    pub fn size(&self) -> usize {
        self.1
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }

    pub unsafe fn get_ffi_image(&self) -> ffi::Image {
        self.0
    }
}
