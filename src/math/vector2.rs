use crate::ffi::Vector2;

use std::ops::{Add, Sub, Mul, Neg, Div};

use super::MathUtils;

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
