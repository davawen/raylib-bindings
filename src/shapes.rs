use std::f32::consts::PI;

use ffi::{Vector2, Rectangle};

use crate::{core::draw::DrawHandle, prelude::Color, ffi};

impl<P> DrawHandle<'_, P> {
    #[inline]
    pub fn pixel(&mut self, pos_x: f32, pos_y: f32, color: Color) {
        self.pixel_v(Vector2::new(pos_x, pos_y), color)
    }
    #[inline]
    pub fn pixel_v(&mut self, pos: Vector2, color: Color) {
        unsafe { ffi::DrawPixelV(pos, color) }
    }
    /// Draw a line.
    #[inline]
    pub fn line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.line_v(Vector2::new(start_x, start_y), Vector2::new(end_x, end_y), color)
    }
    /// Draw a line.
    #[inline]
    pub fn line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe { ffi::DrawLineV(start, end, color) }
    }
    /// Draw a line with thickness.
    #[inline]
    pub fn line_ex(&mut self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        unsafe { ffi::DrawLineEx(start, end, thick, color) }
    }
    /// Draw a sequence of lines with every point in `points` connected from start to last.
    #[inline]
    pub fn line_strip(&mut self, points: &[Vector2], color: Color) {
        // NOTE: The cast_mut is safe, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
        // However, that does mean the raylib API can internally change to modify the input (though that would be very very naughty).
        // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
        unsafe { ffi::DrawLineStrip(points.as_ptr().cast_mut(), points.len() as i32, color) }
    }
    /// Draw a line with cubic-bezier in-out interpolation.
    #[inline]
    pub fn line_bezier(&mut self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        unsafe { ffi::DrawLineBezier(start, end, thick, color) }
    }
    /// Draw a filled circle.
    #[inline]
    pub fn circle(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.circle_v(Vector2::new(center_x, center_y), radius, color)
    }
    /// Draw part of a filled circle.
    /// Angles are in radians.
    #[inline]
    pub fn circle_sector(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawCircleSector(center, radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
    }
    /// Draw part of the outline of a circle.
    /// Angles are in radians.
    #[inline]
    pub fn circle_sector_lines(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawCircleSectorLines(center, radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
    }
    /// Draw a gradient filled circle.
    #[inline]
    pub fn circle_gradient(&mut self, center: Vector2, radius: f32, color1: Color, color2: Color) {
        unsafe { ffi::DrawCircleGradient(center.x as i32, center.y as i32, radius, color1, color2) }
    }
    /// Draw a filled circle.
    #[inline]
    pub fn circle_v(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleV(center, radius, color) }
    }
    /// Draw the outline of a circle.
    #[inline]
    pub fn circle_lines(&mut self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        self.circle_lines_v(Vector2::new(center_x, center_y), radius, color)
    }
    /// Draw the outline of a circle.
    #[inline]
    pub fn circle_lines_v(&mut self, center: Vector2, radius: f32, color: Color) {
        unsafe { ffi::DrawCircleLinesV(center, radius, color) }
    }
    /// Draw a filled ellipse.
    #[inline]
    pub fn ellipse(&mut self, center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
        unsafe { ffi::DrawEllipse(center.x as i32, center.y as i32, radius_h, radius_v, color) }
    }
    /// Draw the outline of an ellipse.
    #[inline]
    pub fn ellipse_lines(&mut self, center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
        unsafe { ffi::DrawEllipse(center.x as i32, center.y as i32, radius_h, radius_v, color) }
    }
    /// Draw a filled ring.
    #[inline]
    pub fn ring(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, color: Color) {
        self.ring_ex(center, inner_radius, outer_radius, 0.0, 2.0 * PI, 36, color)
    }
    /// Draw part of a filled ring.
    /// Angles are in radians.
    #[inline]
    pub fn ring_ex(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawRing(center, inner_radius, outer_radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
    }
    /// Draw the outlines of a ring.
    #[inline]
    pub fn ring_lines(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, color: Color) {
        self.ring_lines_ex(center, inner_radius, outer_radius, 0.0, 2.0 * PI, 36, color)
    }
    /// Draw part of the outlines of a ring.
    /// Angles are in radians.
    #[inline]
    pub fn ring_lines_ex(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawRingLines(center, inner_radius, outer_radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
    }
    /// Draw a filled rectangle
    #[inline]
    pub fn rectangle(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
        self.rectangle_v(Vector2::new(pos_x, pos_y), Vector2::new(width, height), color)
    }
    /// Draw a filled rectangle
    #[inline]
    pub fn rectangle_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        unsafe { ffi::DrawRectangleV(pos, size, color) }
    }
    /// Draw a filled rectangle
    #[inline]
    pub fn rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleRec(rec, color) }
    }
}
