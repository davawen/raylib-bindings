use std::ffi::c_void;
pub use crate::ffi::Color;
use crate::{ffi::{self, PixelFormat}, prelude::{Vector3, Vector4}};

impl Color {
    pub const LIGHTGRAY: Color = Color::rgb(200, 200, 200);
    pub const GRAY: Color = Color::rgb(130, 130, 130);
    pub const DARKGRAY: Color = Color::rgb(80, 80, 80);
    pub const YELLOW: Color = Color::rgb(253, 249, 0);
    pub const GOLD: Color = Color::rgb(255, 203, 0);
    pub const ORANGE: Color = Color::rgb(255, 161, 0);
    pub const PINK: Color = Color::rgb(255, 109, 194);
    pub const RED: Color = Color::rgb(230, 41, 55);
    pub const MAROON: Color = Color::rgb(190, 33, 55);
    pub const GREEN: Color = Color::rgb(0, 228, 48);
    pub const LIME: Color = Color::rgb(0, 158, 47);
    pub const DARKGREEN: Color = Color::rgb(0, 117, 44);
    pub const SKYBLUE: Color = Color::rgb(102, 191, 255);
    pub const BLUE: Color = Color::rgb(0, 121, 241);
    pub const DARKBLUE: Color = Color::rgb(0, 82, 172);
    pub const PURPLE: Color = Color::rgb(200, 122, 255);
    pub const VIOLET: Color = Color::rgb(135, 60, 190);
    pub const DARKPURPLE: Color = Color::rgb(112, 31, 126);
    pub const BEIGE: Color = Color::rgb(211, 176, 131);
    pub const BROWN: Color = Color::rgb(127, 106, 79);
    pub const DARKBROWN: Color = Color::rgb(76, 63, 47);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const BLANK: Color = Color::rgba(0, 0, 0, 0);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const RAYWHITE: Color = Color::rgb(245, 245, 245);

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
    pub const fn gray(v: u8) -> Self {
        Color::rgb(v, v, v)
    }
    pub const fn graya(v: u8, a: u8) -> Self {
        Color::rgba(v, v, v, a)
    }

    /// Creates a color from floating point values between 0.0 and 1.0.
    pub fn rgba_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::rgba((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, (a*255.0) as u8)
    }
    /// Creates a color from floating point values between 0.0 and 1.0.
    pub fn rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Color::rgba_f32(r, g, b, 1.0)
    }
    /// Multiplies color's components with factor from 0.0 to 1.0.
    /// Does not multiply alpha channel.
    pub fn scale(self, value: f32) -> Self {
        Color::rgba((self.r as f32 * value) as u8, (self.g as f32 * value) as u8, (self.b as f32 * value) as u8, self.a)
    }
    pub fn with_alpha(self, alpha: u8) -> Self {
        Color::rgba(self.r, self.g, self.b, alpha)
    }

    /// Get Color structure from hexadecimal value
    /// Equivalent to raylib `GetColor`
    pub const fn hex(hex: u32) -> Self {
        Color { r: (hex & 0xff) as u8, g: ((hex >> 8) & 0xff) as u8, b: ((hex >> 16) & 0xff) as u8, a: (hex >> 24) as u8 }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    pub fn fade(self, alpha: f32) -> Self { unsafe { ffi::Fade(self, alpha) } }
    /// Get hexadecimal value for a Color
    pub fn color_to_int(self) -> i32 { unsafe { ffi::ColorToInt(self) } }
    /// Get Color normalized as float [0..1]
    pub fn color_normalize(self) -> Vector4 { unsafe { ffi::ColorNormalize(self) } }
    /// Get Color from normalized values [0..1]
    pub fn color_from_normalized(normalized: Vector4) -> Self { unsafe { ffi::ColorFromNormalized(normalized) }}
    /// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
    pub fn color_to_hsv(self) -> Vector3 { unsafe { ffi::ColorToHSV(self) } }
    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    pub fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Self { unsafe { ffi::ColorFromHSV(hue, saturation, value) } }
    /// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
    pub fn color_from_hsv_vec(hsv: Vector3) -> Self { Self::color_from_hsv(hsv.x, hsv.y, hsv.z) }
    /// Get color multiplied with another color
    pub fn color_tint(self, tint: Self) -> Self { unsafe { ffi::ColorTint(self, tint) } }
    /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
    pub fn color_brightness(self, factor: f32) -> Self { unsafe { ffi::ColorBrightness(self, factor) } }
    /// Get color with contrast correction, contrast values between -1.0f and 1.0f
    pub fn color_contrast(self, contrast: f32) -> Self { unsafe { ffi::ColorContrast(self, contrast) } }
    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    pub fn color_alpha(self, alpha: f32) -> Self { unsafe { ffi::ColorAlpha(self, alpha) } }
    /// Get src alpha-blended into dst color with tint
    /// WARNING: argument order is reversed compared to original raylib function to resemble lerp more closely
    pub fn color_alpha_blend(self, dst: Color, tint: Color) -> Self { unsafe { ffi::ColorAlphaBlend(dst, self, tint) } }

    /// Get `Color` from a source pixel pointer of certain format
    /// Compressed formats are not supported
    /// # Panics
    /// Panics if the given slice does not match the expected size of the format
    pub fn get_pixel_color(src: &[u8], format: PixelFormat) -> Self {
        if src.len() != format.get_size() {
            panic!("expected `src` len({}) to match pixel format size({})", src.len(), format.get_size());
        }

        // SAFETY: The cast mut is safe because the function won't modify the given data.
        // The api forgot to specify it as const.
        unsafe { ffi::GetPixelColor(src.as_ptr().cast_mut() as *mut c_void, format as i32) }
    }

    /// Sets this color into the destination pixel pointer in the given pixel format
    /// Compressed formats are not supported
    pub fn set_pixel_color(self, dst: &mut [u8], format: PixelFormat) {
        if dst.len() != format.get_size() {
            panic!("expected `dst` len({}) to match pixel format size({:?} = {})", dst.len(), format, format.get_size());
        }
        unsafe { ffi::SetPixelColor(dst.as_ptr().cast_mut() as *mut c_void, self, format as i32) }
    }
}

impl PixelFormat {
    /// Returns the size in bytes of a single pixel 
    /// Compressed format are not supported and will return 0
    pub fn get_size(&self) -> usize {
        match self {
            PixelFormat::UncompressedGrayscale => 1,
            PixelFormat::UncompressedGrayAlpha => 2,
            PixelFormat::UncompressedR5G6B5 => 2,
            PixelFormat::UncompressedR5G5B5A1 => 2,
            PixelFormat::UncompressedR4G4B4A4 => 2,
            PixelFormat::UncompressedR16 => 2,
            PixelFormat::UncompressedR8G8B8 => 3,
            PixelFormat::UncompressedR8G8B8A8 => 4,
            PixelFormat::UncompressedR32 => 4,
            PixelFormat::UncompressedR16G16B16 => 6,
            PixelFormat::UncompressedR16G16B16A16 => 8,
            PixelFormat::UncompressedR32G32B32 => 12,
            PixelFormat::UncompressedR32G32B32A32 => 16,
            PixelFormat::CompressedDxt1Rgb => 0,
            PixelFormat::CompressedDxt1Rgba => 0,
            PixelFormat::CompressedDxt3Rgba => 0,
            PixelFormat::CompressedDxt5Rgba => 0,
            PixelFormat::CompressedEtc1Rgb => 0,
            PixelFormat::CompressedEtc2Rgb => 0,
            PixelFormat::CompressedEtc2EacRgba => 0,
            PixelFormat::CompressedPvrtRgb => 0,
            PixelFormat::CompressedPvrtRgba => 0,
            PixelFormat::CompressedAstc4X4Rgba => 0,
            PixelFormat::CompressedAstc8X8Rgba => 0,
        }
    }
}

/// Get the size of an image in bytes given a certain pixel format
pub fn get_pixel_data_size(width: i32, height: i32, format: PixelFormat) -> i32 {
    unsafe { ffi::GetPixelDataSize(width, height, format as i32) }
}
