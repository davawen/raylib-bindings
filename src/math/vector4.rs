use float_cmp::{ApproxEq, F32Margin};

pub use crate::ffi::Vector4;
use crate::prelude::{Vector2, vec2, Vector3, vec3, Quaternion};

use std::ops::{Add, Sub, Mul, Neg, Div};

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
    Vector4 { x, y, z, w }
}

impl Vector4 {
    pub const ZERO: Self = Vector4::splat(0.0);
    pub const ONE: Self = Vector4::splat(1.0);
    pub const X: Self = vec4(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = vec4(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = vec4(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = vec4(0.0, 0.0, 0.0, 1.0);

    /// NOTE: You can also use the [`vec4`] function.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Vector4 { x, y, z, w } }
    /// Creates a new vector with all values set to `v`.
    pub const fn splat(v: f32) -> Self { vec4(v, v, v, v) }

    pub const fn tuple(self) -> (f32, f32, f32, f32) { (self.x, self.y, self.z, self.w) }
    pub const fn array(self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }

    pub const fn vec2(self) -> Vector2 { vec2(self.x, self.y) }
    pub const fn vec3(self) -> Vector3 { vec3(self.x, self.y, self.z) }
    pub const fn quat(self) -> Quaternion { Quaternion::new(self.x, self.y, self.z, self.w) }

    /// Calculate the length of the vector
    /// NOTE: If you need the length squared, consider using `Vector4::length_sqr`
    pub fn length(self) -> f32 { self.length_sqr().sqrt() }
    /// Calculate the squared length of the vector
    pub fn length_sqr(self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w }

    /// Calculate the distance between two vectors (as points)
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length() 
    }
    /// Calculate the distance squared between two vectors (as points)
    pub fn distance_sqr(self, rhs: Self) -> f32 {
        (self - rhs).length_sqr() 
    }

    /// Calculate the dot product between the two vectors
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z + self.w*rhs.w
    }
    /// Component-wise multiplication of two vectors
    pub fn multiply(self, rhs: Self) -> Self {
        vec4(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z, self.w*self.w)
    }
    /// Component-wise division of two vectors
    pub fn divide(self, rhs: Self) -> Self {
        vec4(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z, self.w/rhs.w)
    }

    /// Normalizes this vector (make its length 1)
    /// Undefined for the null vector
    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl ApproxEq for Vector4 {
    type Margin = F32Margin;
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin) &&
        self.z.approx_eq(other.z, margin) && self.w.approx_eq(other.w, margin)
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Self::Output { vec4(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w) }
}

impl Add<f32> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: f32) -> Self::Output { vec4(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs) }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Self::Output { vec4(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w) }
}

impl Sub<f32> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: f32) -> Self::Output { vec4(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs) }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Self::Output { vec4(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs) }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4; fn mul(self, rhs: Vector4) -> Self::Output { rhs * self }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Self::Output { vec4(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs) }
}

impl Div<Vector4> for f32 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Self::Output { vec4(self / rhs.x, self / rhs.y, self / rhs.z, self / rhs.w) }
}

impl Neg for Vector4 {
    type Output = Vector4;
    fn neg(self) -> Self::Output { vec4(-self.x, -self.y, -self.z, -self.w) }
}
