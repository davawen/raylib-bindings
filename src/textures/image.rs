use std::ffi::CStr;

use ffi::{PixelFormat, Color, Rectangle};

use crate::{ffi, prelude::Raylib, cstr, math::color::get_pixel_data_size};

use super::texture::Texture;

/// A raylib image.
/// Use `Raylib::load_image` to create one.
/// 
/// Images are stored on the CPU in RAM.
/// If you need to draw images many times, or if you need to use them in shaders, prefer using a `Texture`.
///
/// # Safety
/// An `ffi::Image` may have any data inside of it.
/// A `raylib::Image` is garanteed to have valid data and well allocated memory.
pub struct Image {
    /// The underlying ffi image
    image: ffi::Image,
    /// The image format validated to be a `PixelFormat` variant.
    format: PixelFormat,
    /// The size in bytes of the underlying `image.data` buffer.
    size: usize
}

impl Image {
    /// Creates a safe image struct from an ffi image by calculating its size.
    /// Checks for image validity and returns `None` if the image is not valid.
    pub fn from_ffi(image: ffi::Image) -> Option<Self> {
        if !image.is_valid() { return None }
        let format = image.format.try_into().unwrap();

        Some(Image {
            image,
            format,
            size: get_pixel_data_size(image.width, image.height, format) as usize 
        })
    }

    /// Get access to the underlying image data buffer
    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.image.data as *const u8, self.size) }
    }

    /// Get mutable access to the underlying image data buffer
    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.image.data as *mut u8, self.size) }
    }

    /// Size in bytes of the image data buffer
    pub fn size(&self) -> usize {
        self.size
    }

    /// The underlying image width
    pub fn width(&self) -> u32 {
        self.image.width as u32
    }

    /// The underlying image height
    pub fn height(&self) -> u32 {
        self.image.height as u32
    }

    pub fn format(&self) -> PixelFormat {
        self.format
    }

    pub unsafe fn get_ffi_image(&self) -> ffi::Image {
        self.image
    }
}

impl ffi::Image {
    /// Checks if the underlying image metadata is valid.
    /// Equivalent to the not so well named `IsImageReady` raylib function.
    /// Prefer using this instead of checking for a null image data pointer.
    #[inline]
    pub fn is_valid(&self) -> bool {
        let ready = unsafe { ffi::IsImageReady(*self) };
        ready && PixelFormat::try_from(self.format).is_ok()
    }
}


impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.image) }
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

        Some(Image { image, format, size: size as usize })
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

/// # Image generation functions
/// 
/// ---
impl Raylib {
    /// Create an image of the given size composed of plain color.
    /// The resulting image format is `PixelFormat::UncompressedR8G8B8A8`.
    #[inline]
    pub fn gen_image_color(&mut self, width: u32, height: u32, color: Color) -> Image {
        let image = unsafe { ffi::GenImageColor(width as i32, height as i32, color) };
        Image::from_ffi(image).unwrap()
    }

    /// Create an image of the given size composed of a linear gradient.
    /// The angle is in radians, `0.0` is a vertical gradient from left to right.
    /// The resulting image format is `PixelFormat::UncompressedR8G8B8A8`.
    /// WARN: Due to raylib shenigans, the angle cannot go below integer degree precision.
    #[inline]
    pub fn gen_image_gradient_linear(&mut self, width: u32, height: u32, angle: f32, start: Color, end: Color) -> Image {
        let image = unsafe { ffi::GenImageGradientLinear(width as i32, height as i32, angle.to_degrees().round() as i32, start, end) };
        Image::from_ffi(image).unwrap()
    }

    /// Create an image of the given size composed of a radial gradient.
    /// - The generated circle has a diameter of width or height, whichever is smaller.
    /// - Its outside is `outer`, and its inside is a linear blend of `inner` and `outer` depending on distance.
    /// - As density increases, it takes less distance for `outer` to blend into `inner`.
    /// - If density is `1`, the result is a perfectly crisp circle.
    /// - For densities above `1`, the gradient is reversed.
    ///
    /// The resulting image format is `PixelFormat::UncompressedR8G8B8A8`.
    #[inline]
    pub fn gen_image_gradient_radial(&mut self, width: u32, height: u32, density: f32, inner: Color, outer: Color) -> Image {
        let image = unsafe { ffi::GenImageGradientRadial(width as i32, height as i32, density, inner, outer) };
        Image::from_ffi(image).unwrap()
    }

    /// Create an image of the given size composed of a square gradient.
    /// - As density increases, it takes less distance for `outer` to blend into `inner`.
    /// - For densities above or equal to `1`, the image is plain `outer` color.
    ///
    /// The resulting image format is `PixelFormat::UncompressedR8G8B8A8`.
    #[inline]
    pub fn gen_image_gradient_square(&mut self, width: u32, height: u32, density: f32, inner: Color, outer: Color) -> Image {
        let image = unsafe { ffi::GenImageGradientSquare(width as i32, height as i32, density, inner, outer) };
        Image::from_ffi(image).unwrap()
    }

    /// Creates an image of the given size in a checkerboard pattern.
    /// If you need a set number of square instead of a set size, use `Raylib::gen_image_checked_num`.
    #[inline]
    pub fn gen_image_checked(&mut self, width: u32, height: u32, check_width: u32, check_height: u32, top_left: Color, other: Color) -> Image {
        let image = unsafe { ffi::GenImageChecked(width as i32, height as i32, check_width as i32, check_height as i32, top_left, other) };
        Image::from_ffi(image).unwrap()
    }

    /// Creates an image of the given size in a checkerboard pattern.
    /// If you need a set size instead of a set number of square, use `Raylib::gen_image_checked`.
    #[inline]
    pub fn gen_image_checked_num(&mut self, width: u32, height: u32, num_check_x: u32, num_check_y: u32, top_left: Color, other: Color) -> Image {
        let w = width / num_check_x;
        let h = height / num_check_y;
        self.gen_image_checked(width, height, w, h, top_left, other)
    }

    /// Creates an image with white noise (TV snow).
    /// - `factor` defines the ratio between white and black pixels.
    /// - `0.0` means all black, `1.0` means all white.
    #[inline]
    pub fn gen_image_white_noise(&mut self, width: u32, height: u32, factor: f32) -> Image {
        let image = unsafe { ffi::GenImageWhiteNoise(width as i32, height as i32, factor) };
        Image::from_ffi(image).unwrap()
    }

    /// Generate an image with 2D layered perlin noise.
    /// Raylib uses these default parameters:
    /// - lacunarity = 2.0
    /// - gain = 0.5
    /// - octaves = 6
    #[inline]
    pub fn gen_image_perlin_noise(&mut self, width: u32, height: u32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
        let image = unsafe { ffi::GenImagePerlinNoise(width as i32, height as i32, offset_x, offset_y, scale) };
        Image::from_ffi(image).unwrap()
    }

    /// Generate an image with cellular noise (whorley noise).
    /// Use a bigger tile size to get less points.
    /// Pixels close to points are black, pixels far away are white.
    #[inline]
    pub fn gen_image_cellular(&mut self, width: u32, height: u32, tile_size: u32) -> Image {
        let image = unsafe { ffi::GenImageCellular(width as i32, height as i32, tile_size as i32) };
        Image::from_ffi(image).unwrap()
    }

    /// Create an image from the byte data in text.
    // NOTE: I honestly have no idea what would be the use for this.
    #[inline]
    pub fn gen_image_text(&mut self, width: u32, height: u32, text: &CStr) -> Image {
        let image = unsafe { ffi::GenImageText(width as i32, height as i32, text.as_ptr()) };
        Image::from_ffi(image).unwrap()
    }
}

/// # Image manipulation functions
///
/// ---
impl Raylib {
    /// Duplicates the given image to a new memory buffer.
    /// Needs a reference to the raylib object since it allocates memory using raylib's allocator.
    pub fn image_copy(&mut self, image: &Image) -> Image {
        let other = unsafe { ffi::ImageCopy(image.image) };
        Image::from_ffi(other).unwrap()
    }

    /// Duplicates part of the given image to a new memory buffer.
    /// Needs a reference to the raylib object since it allocates memory using raylib's allocator.
    pub fn image_from_image(&mut self, rec: Rectangle, image: &Image) -> Image {
        let other = unsafe { ffi::ImageFromImage(image.image, rec) };
        Image::from_ffi(other).unwrap()
    }

    // TODO: Font support
    // pub fn image_text_ex(&mut self, font: Font, color: Color) -> Image;
}
