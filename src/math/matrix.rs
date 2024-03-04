///! Large inspiration taken from https://github.com/raysan5/raylib/blob/master/src/raymath.h

use std::array;
pub use crate::ffi::Matrix;
use crate::prelude::{Vector2, Vector3, Vector4, vec4};

use std::ops::{Add, Sub, Mul};

impl Matrix {
    /// WARN: Be careful not to confuse this with `Matrix::IDENTITY`
    pub const ZERO: Self = Matrix::from_array([0.0; 16]);
    /// The do-nothing matrix
    pub const IDENTITY: Self = Matrix::from_coefs(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    /// Create a new matrix with coefficients given in row-major order
    /// | m0,  m1,  m2,  m3  |
    /// | m4,  m5,  m6,  m7  |
    /// | m8,  m9,  m10, m11 |
    /// | m12, m13, m14, m15 |
    pub const fn from_coefs(m0: f32, m1: f32, m2: f32, m3: f32, m4: f32, m5: f32, m6: f32, m7: f32, m8: f32, m9: f32, m10: f32, m11: f32, m12: f32, m13: f32, m14: f32, m15: f32) -> Self {
        Self { m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15 }
    }

    /// Create a new matrix with coefficients given in row-major order
    /// | m0,  m1,  m2,  m3  |
    /// | m4,  m5,  m6,  m7  |
    /// | m8,  m9,  m10, m11 |
    /// | m12, m13, m14, m15 |
    pub const fn from_array(coefs: [f32; 16]) -> Self {
        let c = coefs;
        Matrix::from_coefs(c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15])
    }

    /// Create a new matrix with the given rows
    pub const fn from_rows(rows: [Vector4; 4]) -> Self {
        Self::from_array([
            rows[0].x, rows[0].y, rows[0].z, rows[0].w,
            rows[1].x, rows[1].y, rows[1].z, rows[1].w,
            rows[2].x, rows[2].y, rows[2].z, rows[2].w,
            rows[3].x, rows[3].y, rows[3].z, rows[3].w,
        ])
    }

    /// Create a new matrix with the given columns
    pub const fn from_cols(cols: [Vector4; 4]) -> Self {
        Self::from_array([
            cols[0].x, cols[1].x, cols[2].x, cols[3].x,
            cols[0].y, cols[1].y, cols[2].y, cols[3].y,
            cols[0].z, cols[1].z, cols[2].z, cols[3].z,
            cols[0].w, cols[1].w, cols[2].w, cols[3].w,
        ])
    }

    pub const fn array(self) -> [f32; 16] {
        [ self.m0, self.m1, self.m2, self.m3, self.m4, self.m5, self.m6, self.m7, self.m8, self.m9, self.m10, self.m11, self.m12, self.m13, self.m14, self.m15 ]
    }

    /// Get the current matrix as a list of columns
    pub const fn cols(self) -> [Vector4; 4] {
        [
            vec4(self.m0, self.m4, self.m8, self.m12),
            vec4(self.m1, self.m5, self.m9, self.m13),
            vec4(self.m2, self.m6, self.m10, self.m14),
            vec4(self.m3, self.m7, self.m11, self.m15),
        ]
    }

    /// Get the current matrix as a list of rows
    pub const fn rows(self) -> [Vector4; 4] {
        [
            vec4(self.m0, self.m1, self.m2, self.m3),
            vec4(self.m4, self.m5, self.m6, self.m7),
            vec4(self.m8, self.m9, self.m10, self.m11),
            vec4(self.m12, self.m13, self.m14, self.m15),
        ]
    }

    /// Get a given translation matrix
    pub const fn translation(p: Vector3) -> Self {
        Matrix::from_coefs(
            1.0, 0.0, 0.0, p.x,
            0.0, 1.0, 0.0, p.y,
            0.0, 0.0, 1.0, p.z,
            0.0, 0.0, 0.0, 1.0
        )
    }

    /// Get a rotation matrix from an axis angle rotation
    /// WARN: `axis` needs to be normalized
    pub fn rotation(axis: Vector3, angle: f32) -> Self {
        let (x, y, z) = axis.tuple();

        let sin = angle.sin();
        let cos = angle.cos();
        let t = 1.0 - cos;
        Matrix::from_coefs(
            x*x*t +   cos, y*x*t + z*sin, z*x*t - y*sin, 0.0,
            x*y*t - z*sin, y*y*t +   cos, z*y*t + x*sin, 0.0,
            x*z*t + y*sin, y*z*t - x*sin, z*z*t +   cos, 0.0,
                      0.0,           0.0,           0.0, 1.0
        )
    }

    /// Get a rotation matrix from a rotation along the X-axis
    /// `angle` is in radians
    pub fn rotation_x(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        m.m5 = cos;
        m.m6 = sin;
        m.m9 = -sin;
        m.m10 = cos;
        m
    }

    /// Get a rotation matrix from a rotation along the Y-axis
    /// `angle` is in radians
    pub fn rotation_y(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        m.m0 = cos;
        m.m8 = sin;
        m.m2 = -sin;
        m.m10 = cos;
        m
    }

    /// Get a rotation matrix from a rotation along the Z-axis
    /// `angle` is in radians
    pub fn rotation_z(angle: f32) -> Self {
        let mut m = Self::IDENTITY;
        let cos = angle.cos();
        let sin = angle.sin();
        m.m0 = cos;
        m.m1 = sin;
        m.m4 = -sin;
        m.m5 = cos;
        m
    }

    /// Computes a rotation matrix applied in order X -> Y -> Z
    /// Angles are in radians
    pub fn rotation_xyz(angles: Vector3) -> Self {
        Matrix::rotation_z(angles.z) * Matrix::rotation_y(angles.y) * Matrix::rotation_x(angles.x)
    }

    /// Computes a rotation matrix applied in order Z -> Y -> X
    /// Angles are in radians
    pub fn rotation_zyx(angles: Vector3) -> Self {
        Matrix::rotation_x(angles.x) * Matrix::rotation_y(angles.y) * Matrix::rotation_z(angles.z)
    }

    /// Computes a rotation matrix applied in order Y -> X -> Z
    /// Angles are in radians
    pub fn rotation_yxz(angles: Vector3) -> Self {
        Matrix::rotation_z(angles.z) * Matrix::rotation_x(angles.x) * Matrix::rotation_y(angles.y)
    }

    /// Computes the scale matrix for the given scales
    pub fn scale(scale: Vector3) -> Self {
        Matrix::from_coefs(
            scale.x,     0.0,     0.0, 0.0,
                0.0, scale.y,     0.0, 0.0,
                0.0,     0.0, scale.z, 0.0,
                0.0,     0.0,     0.0, 0.0
        )
    }

    /// Get the perspective projection matrix corresponding to a given frustum
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = right - left;
        let tb = top - bottom;
        let an = far - near;

        let mut m = Matrix::ZERO;
        m.m0 = near * 2.0 / rl;
        m.m5 = near * 2.0 / tb;
        m.m8 = (right + left)/rl;
        m.m9 = (top + bottom)/tb;
        m.m10 = -(far + near)/an;
        m.m11 = -1.0; // -Z origin
        m.m14 = -(far * near * 2.0)/an;
        m
    }

    /// Get the perspective projection matrix for a given camera
    /// `fovy` needs to be in radians
    /// `aspect` is the aspect ratio of the camera (width / height)
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let top = near * (fovy*0.5).tan();
        let right = top * aspect;

        Matrix::frustum(-right, right, -top, top, near, far)
    }

    /// The the orthographic projection matrix corresponding to a given frustum
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let rl = right - left;
        let tb = top - bottom;
        let an = far - near;
        let mut m = Matrix::ZERO;
        m.m0 = 2.0 / rl;
        m.m5 = 2.0 / tb;
        m.m10 = -2.0 / an;
        m.m12 = -(left + right)/rl;
        m.m13 = -(top + bottom)/tb;
        m.m14 = -(far + near)/an;
        m.m15 = 1.0;
        m
    }

    /// Create a view matrix at position `eye` looking at `target`
    /// `up` should by `Vector3::Y` most of the time
    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let vz = (eye - target).normalize();
        let vx = up.cross(vz).normalize();
        let vy = vz.cross(vx);
        Matrix::from_coefs(
            vx.x, vy.x, vz.x, 0.0,
            vx.y, vy.y, vz.y, 0.0,
            vx.z, vy.z, vz.z, 0.0,
            -vx.dot(eye), -vy.dot(eye), -vz.dot(eye), 1.0
        )
    }

    /// Computes the matrix determinant
    pub fn determinant(self) -> f32 {
        let a00 = self.m0; let a01 = self.m1; let a02 = self.m2; let a03 = self.m3;
        let a10 = self.m4; let a11 = self.m5; let a12 = self.m6; let a13 = self.m7;
        let a20 = self.m8; let a21 = self.m9; let a22 = self.m10; let a23 = self.m11;
        let a30 = self.m12; let a31 = self.m13; let a32 = self.m14; let a33 = self.m15;

        a30*a21*a12*a03 - a20*a31*a12*a03 - a30*a11*a22*a03 + a10*a31*a22*a03 +
        a20*a11*a32*a03 - a10*a21*a32*a03 - a30*a21*a02*a13 + a20*a31*a02*a13 +
        a30*a01*a22*a13 - a00*a31*a22*a13 - a20*a01*a32*a13 + a00*a21*a32*a13 +
        a30*a11*a02*a23 - a10*a31*a02*a23 - a30*a01*a12*a23 + a00*a31*a12*a23 +
        a10*a01*a32*a23 - a00*a11*a32*a23 - a20*a11*a02*a33 + a10*a21*a02*a33 +
        a20*a01*a12*a33 - a00*a21*a12*a33 - a10*a01*a22*a33 + a00*a11*a22*a33
    }

    /// Get the trace of the matrix (sum of the values along the diagonal)
    pub fn trace(self) -> f32 {
        self.m0 + self.m5 + self.m10 + self.m15
    }

    /// Computes the matrix transpose
    pub fn transpose(self) -> Self {
        Matrix::from_cols(self.rows())
    }

    /// Computes the inverse of the given matrix
    pub fn invert(self) -> Self {
        let a00 = self.m0; let a01 = self.m1; let a02 = self.m2; let a03 = self.m3;
        let a10 = self.m4; let a11 = self.m5; let a12 = self.m6; let a13 = self.m7;
        let a20 = self.m8; let a21 = self.m9; let a22 = self.m10; let a23 = self.m11;
        let a30 = self.m12; let a31 = self.m13; let a32 = self.m14; let a33 = self.m15;

        let b00 = a00*a11 - a01*a10;
        let b01 = a00*a12 - a02*a10;
        let b02 = a00*a13 - a03*a10;
        let b03 = a01*a12 - a02*a11;
        let b04 = a01*a13 - a03*a11;
        let b05 = a02*a13 - a03*a12;
        let b06 = a20*a31 - a21*a30;
        let b07 = a20*a32 - a22*a30;
        let b08 = a20*a33 - a23*a30;
        let b09 = a21*a32 - a22*a31;
        let b10 = a21*a33 - a23*a31;
        let b11 = a22*a33 - a23*a32;

        // Calculate the invert determinant (inlined to avoid double-caching)
        let inv_det = 1.0/(b00*b11 - b01*b10 + b02*b09 + b03*b08 - b04*b07 + b05*b06);

        Matrix::from_coefs(
            (a11*b11 - a12*b10 + a13*b09)*inv_det,
            (-a01*b11 + a02*b10 - a03*b09)*inv_det,
            (a31*b05 - a32*b04 + a33*b03)*inv_det,
            (-a21*b05 + a22*b04 - a23*b03)*inv_det,
            (-a10*b11 + a12*b08 - a13*b07)*inv_det,
            (a00*b11 - a02*b08 + a03*b07)*inv_det,
            (-a30*b05 + a32*b02 - a33*b01)*inv_det,
            (a20*b05 - a22*b02 + a23*b01)*inv_det,
            (a10*b10 - a11*b08 + a13*b06)*inv_det,
            (-a00*b10 + a01*b08 - a03*b06)*inv_det,
            (a30*b04 - a31*b02 + a33*b00)*inv_det,
            (-a20*b04 + a21*b02 - a23*b00)*inv_det,
            (-a10*b09 + a11*b07 - a12*b06)*inv_det,
            (a00*b09 - a01*b07 + a02*b06)*inv_det,
            (-a30*b03 + a31*b01 - a32*b00)*inv_det,
            (a20*b03 - a21*b01 + a22*b00)*inv_det
        )
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output {
        let a = self.array();
        let b = rhs.array();
        Matrix::from_array(array::from_fn(|i| a[i] + b[i]))
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output {
        let a = self.array();
        let b = rhs.array();
        Matrix::from_array(array::from_fn(|i| a[i] - b[i]))
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let a = self.rows();
        let b = rhs.cols();
        Matrix::from_coefs(
            a[0].dot(b[0]), a[0].dot(b[1]), a[0].dot(b[2]), a[0].dot(b[3]), 
            a[1].dot(b[0]), a[1].dot(b[1]), a[1].dot(b[2]), a[1].dot(b[3]), 
            a[2].dot(b[0]), a[2].dot(b[1]), a[2].dot(b[2]), a[2].dot(b[3]), 
            a[3].dot(b[0]), a[3].dot(b[1]), a[3].dot(b[2]), a[3].dot(b[3]), 
        )
    }
}

impl Mul<Vector2> for Matrix {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Self::Output {
        (self * rhs.vec4(0.0, 1.0)).vec2()
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        (self * rhs.vec4(1.0)).vec3()
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Self::Output {
        let cols = self.rows();
        vec4(rhs.dot(cols[0]), rhs.dot(cols[1]), rhs.dot(cols[2]), rhs.dot(cols[3]))
    }
}
