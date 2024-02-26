use std::{ffi::CStr, collections::HashSet};

use ffi::{PixelFormat, Color, Rectangle, Vector2};
use half::f16;

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
        let size = image.compute_size();

        Some(Image {
            image,
            format,
            size
        })
    }

    /// Recomputes size and format after modifying the underlying image handle.
    fn recompute(&mut self) {
        self.format = self.image.format.try_into().unwrap();
        self.size = self.image.compute_size();
    }

    /// Get access to the underlying image data buffer.
    /// Note that this contains the data for all mipmap levels.
    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.image.data as *const u8, self.size) }
    }

    /// Get mutable access to the underlying image data buffer.
    /// Note that this contains the data for all mipmap levels.
    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.image.data as *mut u8, self.size) }
    }

    /// Size in bytes of the image data buffer.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The underlying image width.
    pub fn width(&self) -> u32 {
        self.image.width as u32
    }

    /// The underlying image height.
    pub fn height(&self) -> u32 {
        self.image.height as u32
    }

    /// The pixel format of the image.
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Get the number of mipmap levels in the image.
    /// 1 by default.
    pub fn mipmaps(&self) -> u32 {
        // image validity checks for mipmaps above 0.
        self.image.mipmaps as u32
    }

    /// Get a copy of the raw raylib image handle.
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

    /// Computes the size of the `data` buffer.
    /// # Panics
    /// Panics if `format` is not a valid PixelFormat.
    #[inline]
    pub fn compute_size(&self) -> usize {
        let format = self.format.try_into().unwrap();
        let mut size = 0;

        let mut width = self.width;
        let mut height = self.height;
        for _ in 0..self.mipmaps {
            size += get_pixel_data_size(width, height, format) as usize;
            width /= 2;
            height /= 2;
        }

        size
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::UnloadImage(self.image) }
    }
}

impl PixelFormat {
    pub fn is_compressed(&self) -> bool {
        use PixelFormat as P;
        matches!(
            self,
            P::CompressedDxt1Rgb | P::CompressedDxt1Rgba |
            P::CompressedDxt3Rgba | P::CompressedDxt5Rgba |
            P::CompressedEtc1Rgb | P::CompressedEtc2Rgb |
            P::CompressedEtc2EacRgba | P::CompressedPvrtRgb |
            P::CompressedPvrtRgba | P::CompressedAstc4X4Rgba |
            P::CompressedAstc8X8Rgba
        )
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
    #[inline]
    pub fn image_copy(&mut self, image: &Image) -> Image {
        let other = unsafe { ffi::ImageCopy(image.image) };
        Image::from_ffi(other).unwrap()
    }

    /// Duplicates part of the given image to a new memory buffer.
    /// Needs a reference to the raylib object since it allocates memory using raylib's allocator.
    #[inline]
    pub fn image_from_image(&mut self, rec: Rectangle, image: &Image) -> Image {
        let other = unsafe { ffi::ImageFromImage(image.image, rec) };
        Image::from_ffi(other).unwrap()
    }

    // TODO: Font support
    // pub fn image_text_ex(&mut self, font: Font, color: Color) -> Image;

    /// Convert image data to the desired format.
    /// Clears mipmap levels.
    /// Doesn't do anything for compressed data formats.
    pub fn image_format(&mut self, image: &mut Image, new_format: PixelFormat) {
        unsafe { ffi::ImageFormat(&mut image.image as *mut _, new_format as i32) }
        image.recompute();
    }

    /// Makes the image's size a power of two.
    /// Fills the created pixels with the given color.
    /// Clears mipmap levels.
    pub fn image_to_pot(&mut self, image: &mut Image, fill: Color) {
        unsafe { ffi::ImageToPOT(&mut image.image as *mut _, fill) }
        image.recompute();
    }

    /// Crops the image to the given rectangle.
    /// Clamps the rectangle to the image's bounds if it is too big.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_crop(&mut self, image: &mut Image, crop: Rectangle) {
        unsafe { ffi::ImageCrop(&mut image.image as *mut _, crop) }
        image.recompute();
    }

    /// Removes the edges of the image whose alpha value go lower than the threshold.
    /// `threshold` should be between 0 and 1.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_alpha_crop(&mut self, image: &mut Image, threshold: f32) {
        unsafe { ffi::ImageAlphaCrop(&mut image.image as *mut _, threshold) }
        image.recompute();
    }

    /// Replace pixels with alpha values lower than `threshold` by the given color.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_alpha_clear(&mut self, image: &mut Image, color: Color, threshold: f32) {
        unsafe { ffi::ImageAlphaClear(&mut image.image as *mut _, color, threshold) }
        image.recompute();
    }

    /// Applies an alpha mask to `image`.
    /// - `mask` should be in `PixelFormat::UncompressedGrayscale`.
    /// - Clears mipmap levels.
    /// - Does nothing for compressed images.
    /// - Does nothing if `image` and `mask` do not have the same size.
    /// 
    /// The resulting image is either `PixelFormat::UncompressedGrayAlpha` or `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_alpha_mask(&mut self, mut image: Image, mask: &Image) -> Image {
        unsafe { ffi::ImageAlphaMask(&mut image.image as *mut _, mask.image) };
        image.recompute();
        image
    }

    /// Multiplies the given image's color components by their respective alpha value.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_alpha_premultiply(&mut self, mut image: Image) -> Image {
        unsafe { ffi::ImageAlphaPremultiply(&mut image.image as *mut _) };
        image.recompute();
        image
    }

    /// Approximates a gaussian blur using a box blur.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_blur_gaussian(&mut self, mut image: Image, blur_size: i32) -> Image {
        unsafe { ffi::ImageBlurGaussian(&mut image.image as *mut _, blur_size) };
        image.recompute();
        image
    }

    /// Resize an image using the bicubic scaling algorithm.
    /// - Clears mipmap levels.
    /// - Does nothing for compressed images.
    /// - The image format stays unchanged if it is one of:
    ///   `UncompressedGrayscale`, `UncompressedGrayAlpha`, `UncompressedR8G8B8`, or `UncompressedR8G8B8A8`.
    /// - Otherwise, the image is converted to `PixelFormat::UncompressedR8G8B8A8`.
    ///   Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_resize_bicubic(&mut self, mut image: Image, new_width: u32, new_height: u32) -> Image {
        unsafe { ffi::ImageResize(&mut image.image as *mut _, new_width as i32, new_height as i32) };
        image.recompute();
        image
    }

    /// Resize an image using the nearest neighbour algorithm.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_resize_nn(&mut self, mut image: Image, new_width: u32, new_height: u32) -> Image {
        unsafe { ffi::ImageResizeNN(&mut image.image as *mut _, new_width as i32, new_height as i32) };
        image.recompute();
        image
    }

    /// Crops part of the image and fills out of bounds part with the given color.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_resize_canvas(&mut self, image: &mut Image, new_width: u32, new_height: u32, offset_x: i32, offset_y: i32, fill: Color) {
        unsafe { ffi::ImageResizeCanvas(&mut image.image as *mut _, new_width as i32, new_height as i32, offset_x, offset_y, fill) }
        image.recompute();
    }

    /// Computes all mipmap levels of an image (until the resulting mipmap size is 1).
    /// Supports POT and non POT images.
    /// The resulting image format is unchanged.
    pub fn image_mipmaps(&mut self, image: &mut Image) {
        unsafe { ffi::ImageMipmaps(&mut image.image as *mut _) };
        image.recompute();
    }

    /// Dither image data to a format with 16 bits per pixels (Floyd-Steinberg dithering).
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// `target` must be `UncompressedR5G6B5`, `UncompressedR5G5B5A1` or `UncompressedR4G4B4A4`, otherwise the image is returned unmodified.
    /// The resulting dithered image is of the target format.
    pub fn image_dither(&mut self, mut image: Image, target: PixelFormat) -> Image {
        let (r, g, b, a) = match target {
            PixelFormat::UncompressedR5G6B5 => (5, 6, 5, 0),
            PixelFormat::UncompressedR5G5B5A1 => (5, 5, 5, 1),
            PixelFormat::UncompressedR4G4B4A4 => (4, 4, 4, 4),
            _ => return image
        };

        unsafe { ffi::ImageDither(&mut image.image as *mut _, r, g, b, a) };
        image.recompute();
        image
    }

    /// Flips image vertically.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_flip_vertical(&mut self, image: &mut Image) {
        unsafe { ffi::ImageFlipVertical(&mut image.image as *mut _) };
        image.recompute();
    }

    /// Flips image horizontally.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_flip_horizontal(&mut self, image: &mut Image) {
        unsafe { ffi::ImageFlipHorizontal(&mut image.image as *mut _) };
        image.recompute();
    }

    /// Rotates an image clockwise in radians.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_rotate(&mut self, image: &mut Image, degrees: f32) {
        unsafe { ffi::ImageRotate(&mut image.image as *mut _, degrees.to_radians() as i32)  };
        image.recompute();
    }
    
    /// Rotates an image clockwise 90 degrees.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_rotate_cw(&mut self, image: &mut Image) {
        unsafe { ffi::ImageRotateCW(&mut image.image as *mut _) }
        image.recompute()
    }

    /// Rotates an image counter-clockwise 90 degrees.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    pub fn image_rotate_ccw(&mut self, image: &mut Image) {
        unsafe { ffi::ImageRotateCCW(&mut image.image as *mut _) }
        image.recompute()
    }

    /// Tints the image in the given color (multiplies every pixel's colors by the tint).
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_color_tint(&mut self, mut image: Image, tint: Color) -> Image {
        unsafe { ffi::ImageColorTint(&mut image.image as *mut _, tint) }
        image.recompute();
        image
    }

    /// Inverts the color of the image (1-color)
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_color_invert(&mut self, mut image: Image) -> Image {
        unsafe { ffi::ImageColorInvert(&mut image.image as *mut _) }
        image.recompute();
        image
    }

    /// Makes the image grayscale.
    /// Equivalent to `rl.image_format(&mut image, PixelFormat::UncompressedGrayscale)`.
    /// The resulting image is thus `PixelFormat::UncompressedGrayscale`.
    /// Clears mipmap levels.
    pub fn image_color_grayscale(&mut self, image: &mut Image) {
        unsafe { ffi::ImageColorGrayscale(&mut image.image as *mut _) };
        image.recompute();
    }

    /// Modifies the image's contrast.
    /// The contrast value should be between `0.0` (no contrast) and `1.0` (full contrast).
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_color_contrast(&mut self, mut image: Image, contrast: f32) -> Image {
        unsafe { ffi::ImageColorContrast(&mut image.image as *mut _, contrast*200.0 - 100.0) };
        image.recompute();
        image
    }

    /// Modifies the image's brightness.
    /// The brightness value should be between `0.0` (completely dark) and `1.0` (completely white).
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_color_brightness(&mut self, mut image: Image, brightness: f32) -> Image {
        unsafe { ffi::ImageColorBrightness(&mut image.image as *mut _, (brightness*510.0 - 255.0) as i32) }
        image.recompute();
        image
    }

    /// Replaces a color in the image by another one.
    /// Only replaces exact matches.
    /// Clears mipmap levels.
    /// Does nothing for compressed images.
    /// The resulting image is always `PixelFormat::UncompressedR8G8B8A8`.
    /// Since the pixel format might get changed, unlike the original raylib function, it returns a new image.
    pub fn image_color_replace(&mut self, mut image: Image, color: Color, replacement: Color) -> Image {
        unsafe { ffi::ImageColorReplace(&mut image.image as *mut _, color, replacement) }
        image.recompute();
        image
    }
}

impl Image {
    /// Get data from the image as a color iterator.
    /// Equivalent to `LoadImageColors`, but does not allocate any memory.
    /// Returns an empty iterator if the image is in a compressed format.
    /// Colors will be compressed into 8 bits for formats with more than 8 bits per pixel.
    pub fn colors<'a>(&'a self) -> impl Iterator<Item = Color> + 'a {
        let num = if self.format.is_compressed() {
            0..0
        } else {
            0..self.height()
        };

        let width = 0..self.width();
        num.flat_map(move |x|
            width.clone().map(move |y|
                // SAFETY:
                // - x and y are garanteed to be in bounds
                // - the format is checked to not be compressed
                unsafe { self.get_color(x, y).unwrap_unchecked() }
            )
        )
    }

    /// Creates a color palette from the given image
    /// Returns a hash set of all the different colors present in the image, up to the specified maximum.
    /// Does not count transparent pixels.
    /// Returns the empty set for compressed images.
    pub fn palette(&self, max_size: usize) -> HashSet<Color> {
        let mut set = HashSet::new();
        for color in self.colors() {
            if color.a == 0 { continue }

            if set.insert(color) {
                if set.len() >= max_size {
                    break
                }
            }
        }
        set
    }

    /// Get image pixel color at the given position.
    /// - Returns `None` if the coordinate is out of bounds.
    /// - Returns `None` if the image is in a compressed format.
    #[inline]
    pub fn get_color(&self, x: u32, y: u32) -> Option<Color> {
        if self.format.is_compressed() { return None }
        if x >= self.width() || y >= self.height() { return None }

        use PixelFormat as P;
        let data = self.data();
        let i = y as usize*self.height() as usize + x as usize;
        let color = match self.format {
            P::UncompressedGrayscale => Color::gray(data[i]),
            P::UncompressedGrayAlpha => Color::graya(data[i], data[i+1]),
            P::UncompressedR5G5B5A1 => {
                let pixel = u16::from_ne_bytes([data[i*2], data[i*2+1]]);
                Color::rgba(
                    (((pixel & 0b11111000000000) >> 11)*(255/31)) as u8,
                    (((pixel & 0b0000011111000000) >>  6)*(255/31)) as u8,
                    (((pixel & 0b0000000000111110) >>  1)*(255/31)) as u8,
                    ((pixel & 0b1)*255) as u8
                )
            }
            P::UncompressedR5G6B5 => {
                let pixel = u16::from_ne_bytes([data[i*2], data[i*2+1]]);
                Color::rgb(
                    (((pixel & 0b1111100000000000) >> 11)*(255/31)) as u8,
                    (((pixel & 0b0000011111100000) >>  5)*(255/63)) as u8,
                    (((pixel & 0b0000000000011111))*(255/31)) as u8,
                )
            }
            P::UncompressedR4G4B4A4 => {
                let pixel = u16::from_ne_bytes([data[i*2], data[i*2+1]]);
                Color::rgba(
                    (((pixel & 0b1111000000000000) >> 12)*(255/15)) as u8,
                    (((pixel & 0b0000111100000000) >>  8)*(255/15)) as u8,
                    (((pixel & 0b0000000011110000) >>  4)*(255/15)) as u8,
                    (((pixel & 0b0000000000001111))*(255/15)) as u8
                )
            }
            P::UncompressedR8G8B8A8 => Color::rgba(data[i*4], data[i*4+1], data[i*4+2], data[i*4+3]),
            P::UncompressedR8G8B8 => Color::rgb(data[i*3], data[i*3+1], data[i*3+2]),
            P::UncompressedR32 => Color::rgb(
                (f32::from_ne_bytes([data[i*4], data[i*4+1], data[i*4+2], data[i*4+3]])*255.0) as u8,
                0,
                0
            ),
            P::UncompressedR32G32B32 => Color::rgb(
                (f32::from_ne_bytes([data[i*12], data[i*12+1], data[i*12+2], data[i*12+3]])*255.0) as u8,
                (f32::from_ne_bytes([data[i*12+4], data[i*12+5], data[i*12+6], data[i*12+7]])*255.0) as u8,
                (f32::from_ne_bytes([data[i*12+8], data[i*12+9], data[i*12+10], data[i*12+11]])*255.0) as u8,
            ),
            P::UncompressedR32G32B32A32 => Color::rgba(
                (f32::from_ne_bytes([data[i*16], data[i*16+1], data[i*16+2], data[i*16+3]])*255.0) as u8,
                (f32::from_ne_bytes([data[i*16+4], data[i*16+5], data[i*16+6], data[i*16+7]])*255.0) as u8,
                (f32::from_ne_bytes([data[i*16+8], data[i*16+9], data[i*16+10], data[i*16+11]])*255.0) as u8,
                (f32::from_ne_bytes([data[i*16+12], data[i*16+13], data[i*16+14], data[i*16+15]])*255.0) as u8,
            ),
            P::UncompressedR16 => Color::rgb(
                (f16::from_ne_bytes([data[i*2], data[i*2+1]]).to_f32()*255.0) as u8,
                0,
                0
            ),
            P::UncompressedR16G16B16 => Color::rgb(
                (f16::from_ne_bytes([data[i*6], data[i*6+1]]).to_f32()*255.0) as u8,
                (f16::from_ne_bytes([data[i*6+2], data[i*6+3]]).to_f32()*255.0) as u8,
                (f16::from_ne_bytes([data[i*6+4], data[i*6+5]]).to_f32()*255.0) as u8,
            ),
            P::UncompressedR16G16B16A16 => Color::rgba(
                (f16::from_ne_bytes([data[i*8], data[i*8+1]]).to_f32()*255.0) as u8,
                (f16::from_ne_bytes([data[i*8+2], data[i*8+3]]).to_f32()*255.0) as u8,
                (f16::from_ne_bytes([data[i*8+4], data[i*8+5]]).to_f32()*255.0) as u8,
                (f16::from_ne_bytes([data[i*8+6], data[i*8+7]]).to_f32()*255.0) as u8,
            ),
            _ => unreachable!()
        };

        Some(color)
    }
}

/// # Image drawing functions (software rendering)
///
/// ---
impl Image {
    /// Clear image with the given color.
    /// Only clears base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ImageClearBackground(&mut self.image as *mut _, color) } 
    }

    /// Draw a single pixel.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_pixel(&mut self, pos_x: f32, pos_y: f32, color: Color) {
        self.draw_pixel_v(Vector2::new(pos_x, pos_y), color)
    }
    /// Draw a single pixel.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_pixel_v(&mut self, pos: Vector2, color: Color) {
        unsafe { ffi::ImageDrawPixelV(&mut self.image as *mut _, pos, color) }
    }
    /// Draw a line.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        unsafe { ffi::ImageDrawLine(&mut self.image as *mut _, start_x as i32, start_y as i32, end_x as i32, end_y as i32, color) }
    }
    /// Draw a line.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        self.draw_line(start.x, start.y, end.x, end.y, color)
    }
    /// Draw a filled circle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_circle_v(Vector2::new(center_x, center_y), radius, color)
    }
    /// Draw a filled circle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_circle_v(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::ImageDrawCircleV(&mut self.image as *mut _, center, radius as i32, color) }
    }
    /// Draw the outline of a circle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_circle_lines(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.draw_circle_lines_v(Vector2::new(center_x, center_y), radius, color)
    }
    /// Draw the outline of a circle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::ImageDrawCircleLinesV(&mut self.image as *mut _, center, radius as i32, color) }
    }
    /// Draw a filled rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
        self.draw_rectangle_v(Vector2::new(pos_x, pos_y), Vector2::new(width, height), color)
    }
    /// Draw a filled rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        unsafe { ffi::ImageDrawRectangleV(&mut self.image as *mut _, pos, size, color) }
    }
    /// Draw a filled rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        unsafe { ffi::ImageDrawRectangleRec(&mut self.image as *mut _, rec, color) }
    }
    /// Draw the outline of a rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle_lines(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, thickness: f32, color: Color) {
        self.draw_rectangle_lines_rec(Rectangle { x: pos_x, y: pos_y, width, height }, thickness, color)
    }
    /// Draw the outline of a rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle_lines_v(&mut self, pos: Vector2, size: Vector2, thickness: f32, color: Color) {
        self.draw_rectangle_lines(pos.x, pos.y, size.x, size.y, thickness, color)
    }
    /// Draw the outline of a rectangle.
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_rectangle_lines_rec(&mut self, rec: Rectangle, thickness: f32, color: Color) {
        unsafe { ffi::ImageDrawRectangleLines(&mut self.image as *mut _, rec, thickness as i32, color) }
    }
    /// Draw an other image within this image
    /// Only changes base image, does not update mipmap levels.
    /// Does not support compressed images.
    #[inline]
    pub fn draw_image(&mut self, src: &Image, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
        unsafe { ffi::ImageDraw(&mut self.image as *mut _, src.image, src_rec, dst_rec, tint) };
    }

    // TODO: Implement text support
    // pub fn image_draw_text(&mut self) {}
}
