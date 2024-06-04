use super::DrawHandle3D;

use crate::ffi;
use crate::prelude::{Vector2, Vector3, Ray, Color};

impl DrawHandle3D<'_> {
    /// Draw a line in 3D world space.
    #[inline]
    pub fn line(&self, start: Vector3, end: Vector3, color: Color) {
        unsafe { ffi::DrawLine3D(start, end, color) }
    }
    /// Draw a point in 3D space.
    #[inline]
    pub fn point(&self, p: Vector3, color: Color) {
        unsafe { ffi::DrawPoint3D(p, color) }
    }
    /// Draw a circle in 3D space.
    #[inline]
    pub fn circle(&self, center: Vector3, radius: f32, rotation_axis: Vector3, rotation_angle: f32, color: Color) {
        unsafe { ffi::DrawCircle3D(center, radius, rotation_axis, rotation_angle, color) }
    }
    /// Draw a color filled triangle in 3D space.
    #[inline]
    pub fn triangle(&self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { ffi::DrawTriangle3D(v1, v2, v3, color) }
    }
    /// Draw a color filled triangle strip in 3D space.
    #[inline]
    pub fn triangle_strip(&self, points: &[Vector3], color: Color) {
        // NOTE: Always the same note about the safety of cast_mut: the points aren't modified but the API forgot to put a `const`.
        unsafe { ffi::DrawTriangleStrip3D(points.as_ptr().cast_mut(), points.len() as i32, color) }
    }
    /// Draw a cube in 3D space.
    #[inline]
    pub fn cube(&self, pos: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCube(pos, width, height, length, color) }
    }
    /// Draw a cube in 3D space.
    #[inline]
    pub fn cube_v(&self, pos: Vector3, size: Vector3, color: Color) {
        self.cube(pos, size.x, size.y, size.z, color)
    }
    /// Draw a wireframe cube in 3D space.
    #[inline]
    pub fn cube_wires(&self, pos: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe { ffi::DrawCubeWires(pos, width, height, length, color) }
    }
    /// Draw a wireframe cube in 3D space.
    #[inline]
    pub fn cube_wires_v(&self, pos: Vector3, size: Vector3, color: Color) {
        self.cube_wires(pos, size.x, size.y, size.z, color)
    }
    /// Draw a sphere in 3D space.
    #[inline]
    pub fn sphere(&self, center: Vector3, radius: f32, color: Color) {
        self.sphere_ex(center, radius, 16, 16, color);
    }
    /// Draw a sphere in 3D space.
    #[inline]
    pub fn sphere_ex(&self, center: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
        unsafe { ffi::DrawSphereEx(center, radius, rings, slices, color) }
    }
    /// Draw a wireframe sphere in 3D space.
    #[inline]
    pub fn sphere_wires(&self, center: Vector3, radius: f32, color: Color) {
        self.sphere_wires_ex(center, radius, 16, 16, color)
    }
    /// Draw a wireframe sphere in 3D space.
    #[inline]
    pub fn sphere_wires_ex(&self, center: Vector3, radius: f32, rings: i32, slices: i32, color: Color) {
        unsafe { ffi::DrawSphereWires(center, radius, rings, slices, color) }
    }
    /// Draw a cylinder/cone in 3D space.
    /// It is centered at the given position.
    #[inline]
    pub fn cylinder(&self, pos: Vector3, radius_top: f32, radius_bot: f32, height: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinder(pos, radius_top, radius_bot, height, slices, color) }
    }
    /// Draw a cylinder/cone in 3D space.
    #[inline]
    pub fn cylinder_ex(&self, start: Vector3, end: Vector3, radius_start: f32, radius_end: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderEx(start, end, radius_start, radius_end, slices, color) }
    }
    /// Draw a wireframe cylinder/cone in 3D space.
    /// It is centered at the given position.
    #[inline]
    pub fn cylinder_wires(&self, pos: Vector3, radius_top: f32, radius_bot: f32, height: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderWires(pos, radius_top, radius_bot, height, slices, color) }
    }
    /// Draw a wireframe cylinder/cone in 3D space.
    #[inline]
    pub fn cylinder_wires_ex(&self, start: Vector3, end: Vector3, radius_start: f32, radius_end: f32, slices: i32, color: Color) {
        unsafe { ffi::DrawCylinderWiresEx(start, end, radius_start, radius_end, slices, color) }
    }
    /// Draw a capsule with the center of its sphere caps at `start` and `end`.
    #[inline]
    pub fn capsule(&self, start: Vector3, end: Vector3, radius: f32, slices: i32, rings: i32, color: Color) {
        unsafe { ffi::DrawCapsule(start, end, radius, slices, rings, color) }
    }
    /// Draw a wireframe capsule with the center of its sphere caps at `start` and `end`.
    #[inline]
    pub fn capsule_wires(&self, start: Vector3, end: Vector3, radius: f32, slices: i32, rings: i32, color: Color) {
        unsafe { ffi::DrawCapsuleWires(start, end, radius, slices, rings, color) }
    }
    /// Draws an XZ plane (with its normal pointing in the Y direction).
    #[inline]
    pub fn plane(&self, pos: Vector3, size: Vector2, color: Color) {
        unsafe { ffi::DrawPlane(pos, size, color) }
    }
    /// Draws a ray as a line.
    #[inline]
    pub fn ray(&self, ray: Ray, color: Color) {
        unsafe { ffi::DrawRay(ray, color) }
    }
    /// Draws a grid centered at (0, 0, 0)
    #[inline]
    pub fn grid(&self, slices: i32, spacing: f32) {
        unsafe { ffi::DrawGrid(slices, spacing) }
    }
}


