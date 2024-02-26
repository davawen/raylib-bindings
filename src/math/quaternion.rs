use float_cmp::{ApproxEq, F32Margin};

use crate::ffi::{Quaternion, Matrix, Vector3, Vector4};

use std::ops::{Add, Sub, Mul, Div};

use super::MathUtils;

impl Quaternion {
    pub const IDENTITY: Self = Quaternion::new(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Quaternion { x, y, z, w } }
    pub const fn splat(v: f32) -> Self { Quaternion::new(v, v, v, v) }

    pub const fn tuple(self) -> (f32, f32, f32, f32) { (self.x, self.y, self.z, self.w) }
    pub const fn array(self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }

    pub const fn vec4(self) -> Vector4 { Vector4::new(self.x, self.y, self.z, self.w) }

    pub fn from_vec(xyz: Vector3, w: f32) -> Self {
        Quaternion::new(xyz.x, xyz.y, xyz.z, w)
    }

    /// Calculate quaternion needed to rotate one vector to another
    pub fn from_to(from: Vector3, to: Vector3) -> Self {
        let dot = from.dot(to);
        let cross = from.cross(to);
        Quaternion::from_vec(cross, 1.0 + dot).normalize()
    }

    /// Get the quaternion equivalent of the given rotation matrix
    pub fn from_matrix(mat: Matrix) -> Self {
        let four_wsquared_minus1 = mat.m0  + mat.m5 + mat.m10;
        let four_xsquared_minus1 = mat.m0  - mat.m5 - mat.m10;
        let four_ysquared_minus1 = mat.m5  - mat.m0 - mat.m10;
        let four_zsquared_minus1 = mat.m10 - mat.m0 - mat.m5;

        let mut idx = 0;
        let mut four_biggest_squared_minus1 = four_wsquared_minus1;
        if four_xsquared_minus1 > four_biggest_squared_minus1
        {
            four_biggest_squared_minus1 = four_xsquared_minus1;
            idx = 1;
        }
        if four_ysquared_minus1 > four_biggest_squared_minus1 {
            four_biggest_squared_minus1 = four_ysquared_minus1;
            idx = 2;
        }
        if four_zsquared_minus1 > four_biggest_squared_minus1 {
            four_biggest_squared_minus1 = four_zsquared_minus1;
            idx = 3;
        }

        let biggest_val = (four_biggest_squared_minus1 + 1.0).sqrt() * 0.5;
        let mult = 0.25 / biggest_val;
        match idx {
            0 => Quaternion::new((mat.m6 - mat.m9)*mult, (mat.m8 - mat.m2)*mult, (mat.m1 - mat.m4)*mult, biggest_val),
            1 => Quaternion::new(biggest_val, (mat.m1 + mat.m4)*mult, (mat.m8 + mat.m2)*mult, (mat.m6 - mat.m9)*mult),
            2 => Quaternion::new((mat.m1 + mat.m4)*mult, biggest_val, (mat.m6 + mat.m9)*mult, (mat.m8 - mat.m2)*mult),
            3 => Quaternion::new((mat.m8 + mat.m2)*mult, (mat.m6 + mat.m9)*mult, biggest_val, (mat.m1 - mat.m4)*mult),
            _ => unreachable!()
        }
    }

    /// Get the rotation matrix equivalent of this quaternion
    pub fn to_matrix(self) -> Matrix {
        let mut m = Matrix::IDENTITY;

        let a2 = self.x*self.x;
        let b2 = self.y*self.y;
        let c2 = self.z*self.z;
        let ac = self.x*self.z;
        let ab = self.x*self.y;
        let bc = self.y*self.z;
        let ad = self.w*self.x;
        let bd = self.w*self.y;
        let cd = self.w*self.z;

        m.m0 = 1.0 - 2.0*(b2 + c2);
        m.m1 = 2.0*(ab + cd);
        m.m2 = 2.0*(ac - bd);

        m.m4 = 2.0*(ab - cd);
        m.m5 = 1.0 - 2.0*(a2 + c2);
        m.m6 = 2.0*(bc + ad);

        m.m8 = 2.0*(ac + bd);
        m.m9 = 2.0*(bc - ad);
        m.m10 = 1.0 - 2.0*(a2 + b2);
        m
    }

    /// Calculate the length of the quaternion
    /// NOTE: If you need the length squared, consider using `Quaternion::length_sqr`
    pub fn length(self) -> f32 { self.length_sqr().sqrt() }
    /// Calculate the squared length of the quaternion
    pub fn length_sqr(self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w }

    /// Normalizes this quaternion (make its length 1)
    /// Undefined for the null quaternion
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Calculate the dot product between the two quaternions (interpreted as vectors)
    pub fn dot(self, rhs: Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*self.z + self.w*self.w
    }

    /// Computes the inverse of the given quaternion
    /// That is, the quaternion that undoes whatever rotation the given quaternion does
    /// Undefined for the null quaternion
    /// ```
    /// use raylib::prelude::Quaternion;
    /// # let q = Quaternion::new(0.3, 0.6, 0.2, 0.6).normalize();
    /// assert!((q * q.inverse() - Quaternion::IDENTITY).length() <= f32::EPSILON);
    /// ```
    pub fn inverse(self) -> Self {
        let q = self / self.length_sqr();
        Quaternion::new(-q.x, -q.y, -q.z, q.w)
    }

    /// Component wise division between two quaternions
    pub fn divide(self, rhs: Self) -> Self {
        Quaternion::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }

    /// Calculate linear interpolation between two quaternions
    /// WARN: The resulting quaternion is non-normalized

    /// Use `Quaternion::nlerp` if you need a normalized quaternion
    /// Use `Quaternion::slerp` if you need a straight rotation
    pub fn lerp(self, rhs: Self, amount: f32) -> Self {
        Quaternion::new(self.x.lerp(rhs.x, amount), self.y.lerp(rhs.y, amount), self.z.lerp(rhs.z, amount), self.w.lerp(rhs.w, amount))
    }

    /// Calculate spherical linear interpolation between two quaternions
    /// Use `Quaternion::nlerp` if you need a faster implementation at the cost of precision
    pub fn slerp(self, mut rhs: Self, amount: f32) -> Self {
        let mut half_theta_cos = self.dot(rhs);
        if half_theta_cos < 0.0 {
            rhs = rhs * -1.0;
            half_theta_cos = -half_theta_cos;
        }

        if half_theta_cos.abs() >= 1.0 { self }
        else if half_theta_cos > 0.95 { self.nlerp(rhs, amount) }
        else {
            let half_theta = half_theta_cos.acos();
            let sin_half_theta = (1.0 - half_theta_cos*half_theta_cos).sqrt();
            if sin_half_theta < f32::EPSILON {
                (self + rhs)*0.5
            } else {
                let ratio_a = ((1.0 - amount)*half_theta).sin() / sin_half_theta;
                let ratio_b = (amount * half_theta).sin() / sin_half_theta;
                self*ratio_a + rhs*ratio_b
            }
        }
    }

    /// Calculate slerp-optimized interpolation between two quaternion
    pub fn nlerp(self, rhs: Self, amount: f32) -> Self {
        self.lerp(rhs, amount).normalize()
    }
}

impl ApproxEq for Quaternion {
    type Margin = F32Margin;
    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin) &&
        self.z.approx_eq(other.z, margin) && self.w.approx_eq(other.w, margin)
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output { Quaternion::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w) }
}

impl Add<f32> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: f32) -> Self::Output { Quaternion::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs) }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Self::Output { Quaternion::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w) }
}

impl Sub<f32> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: f32) -> Self::Output { Quaternion::new(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs) }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion::new(
            self.x*rhs.w + self.w*rhs.x + self.y*rhs.z - self.z*rhs.y,
            self.y*rhs.w + self.w*rhs.y + self.z*rhs.x - self.x*rhs.z,
            self.z*rhs.w + self.w*rhs.z + self.x*rhs.y - self.y*rhs.x,
            self.w*rhs.w - self.x*rhs.x - self.y*rhs.y - self.z*rhs.z
        )
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f32) -> Self::Output { Quaternion::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs) }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output { rhs * self }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: f32) -> Self::Output { Quaternion::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs) }
}

impl Div<Quaternion> for f32 {
    type Output = Quaternion;
    fn div(self, rhs: Quaternion) -> Self::Output { Quaternion::new(self / rhs.x, self / rhs.y, self / rhs.z, self / rhs.w) }
}
