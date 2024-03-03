//! Pure rust reimplementation of raymath.
//! 
//! Large inspiration taken from [https://github.com/raysan5/raylib/blob/master/src/raymath.h]

pub mod color;
pub mod vector2;
pub mod vector3;
pub mod vector4;
pub mod quaternion;
pub mod matrix;
pub mod camera;
pub mod rectangle;

pub use self::{color::*, vector2::*, vector3::*, vector4::*, quaternion::*};

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

