///! Large inspiration taken from https://github.com/raysan5/raylib/blob/master/src/raymath.h

use crate::ffi::Vector2;

use std::ops::{Add, Sub, Mul, Neg, Div};

use super::MathUtils;

impl Vector2 {
    pub const ZERO: Self = Vector2::splat(0.0);
    pub const ONE: Self = Vector2::splat(1.0);
    pub const X: Self = Vector2::new(1.0, 0.0);
    pub const Y: Self = Vector2::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self { Vector2 { x, y } }
    pub const fn splat(v: f32) -> Self { Vector2::new(v, v) }

    pub const fn tuple(self) -> (f32, f32) { (self.x, self.y) }
    pub const fn array(self) -> [f32; 2] { [self.x, self.y] }

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
        self.cross(rhs).atan2(self.dot(rhs))
    }
    /// Calculate the angle defined by the line passing through the two vectors (as points)
    /// NOTE: Angles move clockwise as in the original raylib implementation
    pub fn line_angle(self, end: Self) -> f32 {
        (end.y - self.y).atan2(end.x - self.x)
    }
    
    /// Calculate the dot product between the two vectors
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y 
    }
    /// Calculate the 2d cross product of the two vectors
    /// x1*y2 - y1*x2
    pub fn cross(self, rhs: Self) -> f32 {
        self.x*rhs.y - self.y*rhs.x
    }
    /// Component-wise multiplication of two vectors
    pub fn multiply(self, rhs: Self) -> Self {
        Vector2::new(self.x*rhs.x, self.y*rhs.y)
    }
    /// Component-wise division of two vectors
    pub fn divide(self, rhs: Self) -> Self {
        Vector2::new(self.x/rhs.x, self.y/rhs.y)
    }

    /// Normalizes this vector (make its length 1)
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    /// Component-wise linear interpolation of two vectors
    pub fn lerp(self, rhs: Self, amount: f32) -> Self {
        Vector2::new(self.x.lerp(rhs.x, amount), self.y.lerp(rhs.y, amount))
    }
    /// Component-wise clamp of this vector between the values specified by min and max
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Vector2::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
    /// Clamp the length of this vector between the specified values by stretching or squeezing it if needed
    /// Doesn't do anything if `self` is the null vector
    /// ```
    /// use raylib::Vector2;
    /// let v = Vector2::splat(2.0); // length is sqrt(8)
    /// assert_eq!(v.clamp_magnitude(5.0, 10.0).length(), 5.0);
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
    /// Calculate vector reflected by normal
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0*normal*normal.dot(self)
    }
    /// Rotates a vector by an angle
    pub fn rotate(self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vector2::new(self.x*cos - self.y*sin, self.x*sin + self.y*cos)
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

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output { Vector2::new(self.x + rhs.x, self.y + rhs.y) }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Self::Output { Vector2::new(self.x + rhs, self.y + rhs) }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output { Vector2::new(self.x - rhs.x, self.y - rhs.y) }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: f32) -> Self::Output { Vector2::new(self.x - rhs, self.y - rhs) }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output { Vector2::new(self.x * rhs, self.y * rhs) }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output { rhs * self }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Self::Output { Vector2::new(self.x / rhs, self.y / rhs) }
}

impl Div<Vector2> for f32 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Self::Output { Vector2::new(self / rhs.x, self / rhs.y) }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output { Vector2::new(-self.x, -self.y) }
}

// TODO: 
// impl Mul<Mat> for Vector2
