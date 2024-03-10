use float_cmp::{ApproxEq, F32Margin};

pub use crate::ffi::Vector3;
use crate::prelude::{Matrix, Vector2, vec2, Vector4, vec4, Quaternion};

use std::ops::{Add, Sub, Mul, Neg, Div};

use super::MathUtils;

pub const fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
}

impl Vector3 {
    pub const ZERO: Self = Vector3::splat(0.0);
    pub const ONE: Self = Vector3::splat(1.0);
    pub const X: Self = vec3(1.0, 0.0, 0.0);
    pub const Y: Self = vec3(0.0, 1.0, 0.0);
    pub const Z: Self = vec3(0.0, 0.0, 1.0);

    /// NOTE: You can also use the [`vec3`] function.
    pub const fn new(x: f32, y: f32, z: f32) -> Self { Vector3 { x, y, z } }
    /// Creates a new vector with all values set to `v`.
    pub const fn splat(v: f32) -> Self { vec3(v, v, v) }

    pub const fn tuple(self) -> (f32, f32, f32) { (self.x, self.y, self.z) }
    pub const fn array(self) -> [f32; 3] { [self.x, self.y, self.z] }

    pub const fn vec2(self) -> Vector2 { vec2(self.x, self.y) }
    pub const fn vec4(self, w: f32) -> Vector4 { vec4(self.x, self.y, self.z, w) }

    /// Calculate the length of the vector
    /// NOTE: If you need the length squared, consider using `Vector3::length_sqr`
    pub fn length(self) -> f32 { self.length_sqr().sqrt() }
    /// Calculate the squared length of the vector
    pub fn length_sqr(self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }

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
        let cross = self.cross(rhs);
        let dot = self.dot(rhs);
        cross.length().atan2(dot)
    }
    
    /// Calculate the dot product between the two vectors
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*self.z
    }
    /// Calculate the 3d cross product between the two vectors
    pub fn cross(self, rhs: Self) -> Self {
        vec3(self.y*rhs.z - self.z*rhs.y, self.z*rhs.x - self.x*rhs.z, self.x*rhs.y - self.y*rhs.x)
    }
    /// Component-wise multiplication of two vectors
    pub fn multiply(self, rhs: Self) -> Self {
        vec3(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
    /// Component-wise division of two vectors
    pub fn divide(self, rhs: Self) -> Self {
        vec3(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z)
    }

    /// Normalizes this vector (make its length 1)
    /// Undefined for the null vector
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    /// Component-wise linear interpolation of two vectors
    pub fn lerp(self, rhs: Self, amount: f32) -> Self {
        vec3(self.x.lerp(rhs.x, amount), self.y.lerp(rhs.y, amount), self.z.lerp(rhs.z, amount))
    }
    /// Component-wise clamp of this vector between the values specified by min and max
    pub fn clamp(self, min: Self, max: Self) -> Self {
        vec3(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), self.z.clamp(min.z, max.z))
    }
    /// Clamps the length of this vector between the specified values by stretching or squeezing it if needed
    /// Doesn't do anything if `self` is the null vector
    /// ```
    /// use raylib::prelude::Vector3;
    /// let v = Vector3::splat(2.0); // length is sqrt(12.0)
    /// assert_eq!(v.clamp_magnitude(3.0, 10.0).length(), 12.0_f32.sqrt()); // 3.0 < sqrt(12.0) < 10.0
    /// ```
    pub fn clamp_magnitude(self, min: f32, max: f32) -> Self {
        let length = self.length_sqr();
        if length == 0.0 { return self }

        let length = length.sqrt();
        if length < min {
            self * min / length
        } else if length > max {
            self * max / length
        } else { self }
    }
    /// Calculate one possible perpendicular vector to this one
    pub fn perpendicular(self) -> Self {
        let mut min = self.x.abs();
        let mut cardinal = Vector3::X;
        if self.y.abs() < min {
            min = self.y.abs();
            cardinal = Vector3::Y;
        }
        if self.z.abs() < min {
            cardinal = Vector3::Z;
        }
        self.cross(cardinal)
    }
    /// Orthonomalizes the given vectors
    /// Makes the two vectors normalized and orthogonal to each other
    /// Direct copy of the raylib implementation: `https://github.com/raysan5/raylib/blob/ea31bd47e5135680d5cc0f29800372cf8cfeb49a/src/raymath.h#L766`
    pub fn ortho_normalize(v1: Self, v2: Self) -> (Self, Self) {
        let v1 = v1.normalize();
        let cross = v1.cross(v2).normalize();
        let v2 = cross.cross(v1);

        (v1, v2)
    }
    /// Calculate vector reflected by normal
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0*normal*normal.dot(self)
    }
    /// Calculate the projection of the first vector on to the second one
    pub fn project(self, rhs: Self) -> Self {
        let dot = self.dot(rhs);
        let len = rhs.length_sqr();
        let mag = dot / len;
        rhs * mag
    }
    /// Calculate the rejection of the first vector on to the second one
    /// The rejection is the projection on the plane orthogonal to the vector
    /// `https://math.stackexchange.com/a/2483290`
    pub fn rejection(self, rhs: Self) -> Self {
        self - self.project(rhs)
    }
    /// Compute the direction of a ray refracted through a surface with a given normal
    /// `self` and `normal` need to be normalized
    /// `ratio` is the refractive index of the medium the ray comes from divided by the refractive index of the medium it is going to
    /// Returns the null vector is the refraction is out of range
    pub fn refract(self, normal: Self, ratio: f32) -> Self {
        let dot = self.dot(normal);
        let d = 1.0 - ratio*ratio*(1.0 - dot*dot);

        if d >= 0.0 {
            let d = d.sqrt();
            ratio * self - normal*(ratio*dot + d)
        } else {
            Vector3::ZERO
        }
    }

    /// Transforms the vector by the given matrix (with the translation)
    /// Same as `mat * self`
    pub fn transform(self, mat: Matrix) -> Self {
        mat * self
    }

    /// Rotates the vector by the given quaternion
    /// Same as `self * quat`
    pub fn rotate_by_quaternion(self, quat: Quaternion) -> Self {
        self * quat
    }

    /// Rotates the vector around an axis
    /// WARN: `axis` needs to be normalized
    pub fn rotate_by_axis_angle(self, axis: Self, angle: f32) -> Self {
        let angle = angle / 2.0;

        let w = axis * angle.sin();
        let wv = w.cross(self);
        let wwv = w.cross(wv);

        let wv = wv * angle.cos() * 2.0;
        let wwv = wwv * 2.0;

        self + wv + wwv
    }

    /// Computes barycentric coordinate of `self` given a triangle vertices `a`, `b` and `c`
    /// NOTE: Assume `self` is on the plane of the triangle
    ///
    /// ```
    /// use raylib::prelude::{vec3, Vector3};
    /// use float_cmp::assert_approx_eq;
    /// let a = vec3(0.0, 0.0, 0.0);
    /// let b = vec3(1.0, 0.0, 0.0);
    /// let c = vec3(0.0, 1.0, 0.0);
    ///
    /// let p = a;
    /// assert_eq!(p.barycenter(a, b, c), vec3(1.0, 0.0, 0.0)); // only a
    /// let p = vec3(0.75, 0.0, 0.0);
    /// assert_eq!(p.barycenter(a, b, c), vec3(0.25, 0.75, 0.0)); // between a and b
    /// let p = vec3(1.0/3.0, 1.0/3.0, 0.0);
    /// assert_approx_eq!(Vector3, p.barycenter(a, b, c), Vector3::splat(1.0/3.0), ulps = 2) // center
    /// ```
    pub fn barycenter(self, a: Self, b: Self, c: Self) -> Self {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = self - a;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denom = d00*d11 - d01*d01;

        let y = (d11*d20 - d01*d21)/denom;
        let z = (d00*d21 - d01*d20)/denom;
        let x = 1.0 - z - y;
        vec3(x, y, z)
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

impl ApproxEq for Vector3 {
    type Margin = F32Margin;
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin) && self.z.approx_eq(other.z, margin)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output { vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: f32) -> Self::Output { vec3(self.x + rhs, self.y + rhs, self.z + rhs) }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output { vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: f32) -> Self::Output { vec3(self.x - rhs, self.y - rhs, self.z - rhs) }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output { vec3(self.x * rhs, self.y * rhs, self.z * rhs) }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output { rhs * self }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Self::Output { vec3(self.x / rhs, self.y / rhs, self.z / rhs) }
}

impl Div<Vector3> for f32 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output { vec3(self / rhs.x, self / rhs.y, self / rhs.z) }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output { vec3(-self.x, -self.y, -self.z) }
}

impl Mul<Quaternion> for Vector3 {
    type Output = Vector3;
    fn mul(self, q: Quaternion) -> Self::Output {
        vec3(
            self.x*(q.x*q.x + q.w*q.w - q.y*q.y - q.z*q.z) + self.y*(2.0*q.x*q.y - 2.0*q.w*q.z) + self.z*(2.0*q.x*q.z + 2.0*q.w*q.y),
            self.x*(2.0*q.w*q.z + 2.0*q.x*q.y) + self.y*(q.w*q.w - q.x*q.x + q.y*q.y - q.z*q.z) + self.z*(-2.0*q.w*q.x + 2.0*q.y*q.z),
            self.x*(-2.0*q.w*q.y + 2.0*q.x*q.z) + self.y*(2.0*q.w*q.x + 2.0*q.y*q.z)+ self.z*(q.w*q.w - q.x*q.x - q.y*q.y + q.z*q.z),
        )
    }
}
