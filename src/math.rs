use crate::ffi::{Color, Vector2, Vector3, Vector4, self};

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


/// Reimplementation of raylib math utils
pub trait MathUtils: Copy {
    /// Calculate linear interpolation between two floats
    fn lerp(self, end: Self, amount: Self) -> Self;
    /// Normalize input value within input range
    /// Remap values from [start; end] to [0; 1]
    fn normalize(self, start: Self, end: Self) -> Self;
    /// Remap input value within input range to output range
    fn remap(self, input_start: Self, input_end: Self, output_start: Self, output_end: Self) -> Self;
    /// Wrap input value from min to max
    fn wrap(self, min: Self, max: Self) -> Self;
}

impl MathUtils for f32 {
    fn lerp(self, end: Self, amount: Self) -> Self { self + amount*(end - self) }
    fn normalize(self, start: Self, end: Self) -> Self { (self - start)/(end - start) }
    fn remap(self, input_start: Self, input_end: Self, output_start: Self, output_end: Self) -> Self { (self - input_start)/(input_end - input_start) * (output_end - output_start) + output_start }
    fn wrap(self, min: Self, max: Self) -> Self { self - (max - min)*((self - min)/(max-min)).floor() }
}

impl MathUtils for f64 {
    fn lerp(self, end: Self, amount: Self) -> Self { self + amount*(end - self) }
    fn normalize(self, start: Self, end: Self) -> Self { (self - start)/(end - start) }
    fn remap(self, input_start: Self, input_end: Self, output_start: Self, output_end: Self) -> Self { (self - input_start)/(input_end - input_start) * (output_end - output_start) + output_start }
    fn wrap(self, min: Self, max: Self) -> Self { self - (max - min)*((self - min)/(max-min)).floor() }
}

use std::ops::{Add, Sub, Mul, Neg, Div};

impl Vector2 {
    pub const ZERO: Self = Vector2 { x: 0.0, y: 0.0 };
    pub const ONE: Self = Vector2 { x: 1.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self { Vector2 { x, y } }
    pub const fn splat(v: f32) -> Self { Vector2::new(v, v) }

    /// Calculate the length of the vector
    /// NOTE: If you need the length squared, consider using `Vector2::length_sqr`
    pub fn length(self) -> f32 { self.length_sqr().sqrt() }
    /// Calculate the squared length of the vector
    pub fn length_sqr(self) -> f32 { self.x*self.x + self.y*self.y }

    /// Calculate the distance between two vectors (as points)
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length() 
    }
    /// Calculate the distance squared between two vectors (as points)
    pub fn distance_sqr(self, rhs: Self) -> f32 {
        (self - rhs).length_sqr() 
    }
    /// Calculate the angle between two vectors
    /// NOTE: Angle is calculated from origin point (0, 0)
    pub fn angle(self, rhs: Self) -> f32 {
        let dot = self.x*rhs.x + self.y*rhs.y;
        let det = self.x*rhs.y - self.y*rhs.x;
        det.atan2(dot)
    }
    /// Calculate the angle defined by the line passing through the two vectors (as points)
    /// NOTE: Angles move clockwise (?) as in the original raylib implementation
    pub fn line_angle(self, end: Self) -> f32 {
        (end.y - self.y).atan2(end.x - self.x)
    }
    
    /// Calculate the dot product between the two vectors
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y 
    }
    /// Component-wise multiplication of two vectors
    pub fn multiply(self, rhs: Self) -> Self {
        Vector2 { x: self.x*rhs.x, y: self.x*rhs.y }
    }
    /// Component-wise division of two vectors
    pub fn divide(self, rhs: Self) -> Self {
        Vector2 { x: self.x/rhs.x, y: self.x/rhs.y }
    }

    /// Normalizes this vector (make its length 1)
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    /// Component-wise linear interpolation of two vectors
    pub fn lerp(self, rhs: Self, amount: f32) -> Self {
        Vector2 { x: self.x.lerp(rhs.x, amount), y: self.y.lerp(rhs.y, amount) }
    }
    /// Component-wise clamp of this vector between the values specified by min and max
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Vector2 { x: self.x.clamp(min.x, max.x), y: self.y.clamp(min.y, max.y) }
    }
    /// Clamp the length of this vector between the specified values
    pub fn clamp_magnitude(self, min: f32, max: f32) -> Self {
        let length = self.length_sqr();
        if length == 0.0 { return self }

        let length = length.sqrt();
        let scale = if length < min {
            min / length
        } else if length > max {
            max / length
        } else { 1.0 };

        self * scale
    }
    /// Calculate vector reflected by normal
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0*normal*normal.dot(self)
    }
    /// Rotates a vector by an angle
    pub fn rotate(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vector2 { x: self.x*cos - self.y*sin, y: self.x*sin + self.y*cos }
    }
    /// Move vector towards the targets, moving by a maximum step of `max_distance`
    pub fn move_towards(self, target: Self, max_distance: f32) -> Self {
        let dir = target - self; 
        if dir.length_sqr() > max_distance*max_distance {
            self + dir.normalize()*max_distance
        } else {
            self + dir
        }
    }
    /// Returns 1/self
    pub fn invert(self) -> Self { 1.0 / self }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output { Vector2 { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output { Vector2 { x: self.x + rhs, y: self.y + rhs } }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output { Vector2 { x: self.x - rhs.x, y: self.y - rhs.y } }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: f32) -> Self::Output { Vector2 { x: self.x - rhs, y: self.y - rhs } }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output { Vector2 { x: self.x * rhs, y: self.y * rhs } }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output { rhs * self }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output { Vector2 { x: self.x / rhs, y: self.y / rhs} }
}

impl Div<Vector2> for f32 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Self::Output { Vector2 { x: self / rhs.x, y: self / rhs.y} }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output { Vector2 { x: -self.x, y: -self.y } }
}

// TODO: 
// impl Mul<Mat> for Vector2
