use crate::ffi::{self, Color, Vector3, Vector4};

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
    /// WARNING: argument order is reversed compared to original raylib function
    pub fn color_alpha_blend(self, dst: Color, tint: Color) -> Self { unsafe { ffi::ColorAlphaBlend(dst, self, tint) } }

    // TODO: create safe interface for pixel format
    // /// Get Color from a source pixel pointer of certain format
    // pub fn get_pixel_color(srcPtr: void *, format: i32) -> Self {  }
    // /// Set color formatted into destination pixel pointer
    // pub fn set_pixel_color(dstPtr: void *, format: i32) -> Self {  }
}

// // TODO: create safe for pixel format
// /// Get pixel data size in bytes for certain format
// pub fn get_pixel_data_size(width: i32, height: i32, format: i32) -> i32 {}
