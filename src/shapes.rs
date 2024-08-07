//! Basic shapes drawing functions (module: `rshapes`)

use std::f32::consts::PI;

use ffi::Rectangle;

use crate::{core::draw::DrawHandle, ffi, prelude::{vec2, Color, Vector2, WeakTexture}};

/// Set texture and rectangle to be used on shapes drawing.
#[inline]
pub fn set_shapes_texture(_rl: &DrawHandle, texture: impl Into<WeakTexture>, source: Rectangle) {
    unsafe { ffi::SetShapesTexture(*texture.into().get_ffi(), source) }
}

/// Draw a single pixel.
#[inline]
pub fn draw_pixel(rl: &DrawHandle, pos_x: f32, pos_y: f32, color: Color) {
    draw_pixel_v(rl, vec2(pos_x, pos_y), color)
}
/// Draw a single pixel.
#[inline]
pub fn draw_pixel_v(_rl: &DrawHandle, pos: Vector2, color: Color) {
    unsafe { ffi::DrawPixelV(pos, color) }
}
/// Draw a line.
#[inline]
pub fn draw_line(rl: &DrawHandle, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
    draw_line_v(rl, vec2(start_x, start_y), vec2(end_x, end_y), color)
}
/// Draw a line.
#[inline]
pub fn draw_line_v(_rl: &DrawHandle, start: Vector2, end: Vector2, color: Color) {
    unsafe { ffi::DrawLineV(start, end, color) }
}
/// Draw a line with thickness.
#[inline]
pub fn draw_line_ex(_rl: &DrawHandle, start: Vector2, end: Vector2, thick: f32, color: Color) {
    unsafe { ffi::DrawLineEx(start, end, thick, color) }
}
/// Draw a sequence of lines with every point in `points` connected from start to last.
#[inline]
pub fn draw_line_strip(_rl: &DrawHandle, points: &[Vector2], color: Color) {
    // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
    // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
    // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
    unsafe { ffi::DrawLineStrip(points.as_ptr().cast_mut(), points.len() as i32, color) }
}
/// Draw a line with cubic-bezier in-out interpolation.
#[inline]
pub fn draw_line_bezier(_rl: &DrawHandle, start: Vector2, end: Vector2, thick: f32, color: Color) {
    unsafe { ffi::DrawLineBezier(start, end, thick, color) }
}
/// Draw a filled circle.
#[inline]
pub fn draw_circle(rl: &DrawHandle, center_x: f32, center_y: f32, radius: f32, color: Color) {
    draw_circle_v(rl, vec2(center_x, center_y), radius, color)
}
/// Draw part of a filled circle.
/// Angles are in radians.
#[inline]
pub fn draw_circle_sector(_rl: &DrawHandle, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawCircleSector(center, radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
}
/// Draw part of the outline of a circle.
/// Angles are in radians.
#[inline]
pub fn draw_circle_sector_lines(_rl: &DrawHandle, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawCircleSectorLines(center, radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
}
/// Draw a gradient filled circle.
#[inline]
pub fn draw_circle_gradient(_rl: &DrawHandle, center: Vector2, radius: f32, color1: Color, color2: Color) {
    unsafe { ffi::DrawCircleGradient(center.x as i32, center.y as i32, radius, color1, color2) }
}
/// Draw a filled circle.
#[inline]
pub fn draw_circle_v(_rl: &DrawHandle, center: Vector2, radius: f32, color: Color) {
    unsafe { ffi::DrawCircleV(center, radius, color) }
}
/// Draw the outline of a circle.
#[inline]
pub fn draw_circle_lines(rl: &DrawHandle, center_x: f32, center_y: f32, radius: f32, color: Color) {
    draw_circle_lines_v(rl, vec2(center_x, center_y), radius, color)
}
/// Draw the outline of a circle.
#[inline]
pub fn draw_circle_lines_v(_rl: &DrawHandle, center: Vector2, radius: f32, color: Color) {
    unsafe { ffi::DrawCircleLinesV(center, radius, color) }
}
/// Draw a filled ellipse.
#[inline]
pub fn draw_ellipse(_rl: &DrawHandle, center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { ffi::DrawEllipse(center.x as i32, center.y as i32, radius_h, radius_v, color) }
}
/// Draw the outline of an ellipse.
#[inline]
pub fn draw_ellipse_lines(_rl: &DrawHandle, center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { ffi::DrawEllipse(center.x as i32, center.y as i32, radius_h, radius_v, color) }
}
/// Draw a filled ring.
#[inline]
pub fn draw_ring(rl: &DrawHandle, center: Vector2, inner_radius: f32, outer_radius: f32, color: Color) {
    draw_ring_ex(rl, center, inner_radius, outer_radius, 0.0, 2.0 * PI, 36, color)
}
/// Draw part of a filled ring.
/// Angles are in radians.
#[inline]
pub fn draw_ring_ex(_rl: &DrawHandle, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawRing(center, inner_radius, outer_radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
}
/// Draw the outlines of a ring.
#[inline]
pub fn draw_ring_lines(rl: &DrawHandle, center: Vector2, inner_radius: f32, outer_radius: f32, color: Color) {
    draw_ring_lines_ex(rl, center, inner_radius, outer_radius, 0.0, 2.0 * PI, 36, color)
}
/// Draw part of the outlines of a ring.
/// Angles are in radians.
#[inline]
pub fn draw_ring_lines_ex(_rl: &DrawHandle, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawRingLines(center, inner_radius, outer_radius, start_angle.to_degrees(), end_angle.to_degrees(), segments, color) }
}
/// Draw a filled rectangle.
#[inline]
pub fn draw_rectangle(rl: &DrawHandle, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
    draw_rectangle_v(rl, vec2(pos_x, pos_y), vec2(width, height), color)
}
/// Draw a filled rectangle.
#[inline]
pub fn draw_rectangle_v(_rl: &DrawHandle, pos: Vector2, size: Vector2, color: Color) {
    unsafe { ffi::DrawRectangleV(pos, size, color) }
}
/// Draw a filled rectangle.
#[inline]
pub fn draw_rectangle_rec(_rl: &DrawHandle, rec: Rectangle, color: Color) {
    unsafe { ffi::DrawRectangleRec(rec, color) }
}
/// Draw a rotated filled rectangle.
#[inline]
pub fn draw_rectangle_pro(_rl: &DrawHandle, rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe { ffi::DrawRectanglePro(rec, origin, rotation, color) }
}
/// Draw a horizontal-gradient-filled rectangle.
/// Gradient goes from left to right.
#[inline]
pub fn draw_rectangle_gradient_h(_rl: &DrawHandle, rec: Rectangle, left: Color, right: Color) {
    unsafe { ffi::DrawRectangleGradientH(rec.x as i32, rec.y as i32, rec.width as i32, rec.height as i32, left, right) }
}
/// Draw a vertical-gradient-filled rectangle.
/// Gradient goes from bottom to top.
/// NOTE: While this behaviour is pretty weird, it is how raylib does it originally.
#[inline]
pub fn draw_rectangle_gradient_v(_rl: &DrawHandle, rec: Rectangle, bottom: Color, top: Color) {
    unsafe { ffi::DrawRectangleGradientV(rec.x as i32, rec.y as i32, rec.width as i32, rec.height as i32, bottom, top) }
}
/// Draw a gradient-filled rectangle with custom vertex colors.
/// Each color refers to a corner, starting in the top-left and going counter clockwise.
#[inline]
pub fn draw_rectangle_gradient_ex(_rl: &DrawHandle, rec: Rectangle, top_left: Color, bot_left: Color, bot_right: Color, top_right: Color) {
    unsafe { ffi::DrawRectangleGradientEx(rec, top_left, bot_left, bot_right, top_right) }
}
/// Draw the outline of a rectangle.
#[inline]
pub fn draw_rectangle_lines(_rl: &DrawHandle, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
    unsafe { ffi::DrawRectangleLines(pos_x as i32, pos_y as i32, width as i32, height as i32, color) }
}
/// Draw the outline of a rectangle.
#[inline]
pub fn draw_rectangle_lines_v(rl: &DrawHandle, pos: Vector2, size: Vector2, color: Color) {
    draw_rectangle_lines(rl, pos.x, pos.y, size.x, size.y, color)
}
/// Draw the outline of a rectangle.
#[inline]
pub fn draw_rectangle_lines_rec(rl: &DrawHandle, rec: Rectangle, color: Color) {
    draw_rectangle_lines(rl, rec.x, rec.y, rec.width, rec.height, color)
}
/// Draw the outline of a rectangle with line thickness.
#[inline]
pub fn draw_rectangle_lines_ex(_rl: &DrawHandle, rec: Rectangle, thick: f32, color: Color) {
    unsafe { ffi::DrawRectangleLinesEx(rec, thick, color) }
}
/// Draw a filled rectangle with rounded corners.
/// `roundness` goes from 0.0 to 1.0.
#[inline]
pub fn draw_rectangle_rounded(_rl: &DrawHandle, rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawRectangleRounded(rec, roundness, segments, color) }
}
/// Draw the outline of a rectangle with rounded corners and thickness.
/// `roundness` goes from 0.0 to 1.0.
#[inline]
pub fn draw_rectangle_rounded_lines(_rl: &DrawHandle, rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { ffi::DrawRectangleRoundedLines(rec, roundness, segments, color) }
}
/// Draw the outline of a rectangle with rounded corners and thickness.
/// `roundness` goes from 0.0 to 1.0.
#[inline]
pub fn draw_rectangle_rounded_lines_ex(_rl: &DrawHandle, rec: Rectangle, roundness: f32, segments: i32, thick: f32, color: Color) {
    unsafe { ffi::DrawRectangleRoundedLinesEx(rec, roundness, segments, thick, color) }
}
/// Draw a filled triangle.
/// WARN: The vertices MUST be given in counter-clockwise order! Otherwise, the triangle won't render!
/// If you cannot be certain of the order of the vertices, use `triangle_unordered`.
#[inline]
pub fn draw_triangle(_rl: &DrawHandle, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { ffi::DrawTriangle(v1, v2, v3, color) }
}
/// Draw a filled triangle with vertices in any order.
/// If you can be certain that your vertices are counter-clockwise, prefer using `triangle`.
#[inline]
pub fn draw_triangle_unordered(rl: &DrawHandle, mut v1: Vector2, v2: Vector2, mut v3: Vector2, color: Color) {
    // calculate the determinant of matrix [[x1, y1, 1], [x2, y2, 1], [x3, y3, 1]]
    let det = v3.x*(v1.y - v2.y) + v1.x*(v2.y - v3.y) + v2.x*(-v1.y + v3.y);
    // positive determinant -> counter-clockwise, negative determinant -> clockwise
    if det < 0.0 {
        std::mem::swap(&mut v1, &mut v3);
    }

    draw_triangle(rl, v1, v2, v3, color)
}
/// Draw the outline of a triangle.
/// WARN: The vertices MUST be given in counter-clockwise order! Otherwise, the triangle won't render!
/// If you cannot be certain of the order of the vertices, use `triangle_lines_unordered`.
#[inline]
pub fn draw_triangle_lines(_rl: &DrawHandle, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { ffi::DrawTriangleLines(v1, v2, v3, color) }
}
/// Draw the outline of a triangle with vertices in any order.
/// If you can be certain that your vertices are counter-clockwise, prefer using `triangle_lines`.
#[inline]
pub fn draw_triangle_lines_unordered(rl: &DrawHandle, mut v1: Vector2, v2: Vector2, mut v3: Vector2, color: Color) {
    // calculate the determinant of matrix [[x1, y1, 1], [x2, y2, 1], [x3, y3, 1]]
    let det = v3.x*(v1.y - v2.y) + v1.x*(v2.y - v3.y) + v2.x*(-v1.y + v3.y);
    // positive determinant -> counter-clockwise, negative determinant -> clockwise
    if det < 0.0 {
        std::mem::swap(&mut v1, &mut v3);
    }
    draw_triangle_lines(rl, v1, v2, v3, color)
}
/// Draw a filled triangle fan.
/// The first vertex is the center and is shared by all triangles.
/// Draws nothing if less than 3 points are given.
/// WARN: The vertices must be given in counter-clockwise order!
#[inline]
pub fn draw_triangle_fan(_rl: &DrawHandle, points: &[Vector2], color: Color) {
    // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
    // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
    // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
    unsafe { ffi::DrawTriangleFan(points.as_ptr().cast_mut(), points.len() as i32, color) }
}
/// Draw a filled triangle strip.
/// Every new vertex connects with the previous two to create a triangle.
/// Draws nothing if less than 3 points are given.
#[inline]
pub fn draw_triangle_strip(_rl: &DrawHandle, points: &[Vector2], color: Color) {
    // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
    // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
    // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
    unsafe { ffi::DrawTriangleStrip(points.as_ptr().cast_mut(), points.len() as i32, color) }
}
/// Draw a filled regular polygon (pentagon, hexagon, etc..).
#[inline]
pub fn draw_poly(_rl: &DrawHandle, center: Vector2, sides: usize, radius: f32, rotation: f32, color: Color) {
    unsafe { ffi::DrawPoly(center, sides as i32, radius, rotation, color) }
}
/// Draw the outline of a regular polygon (pentagon, hexagon, etc..).
#[inline]
pub fn draw_poly_lines(_rl: &DrawHandle, center: Vector2, sides: usize, radius: f32, rotation: f32, color: Color) {
    unsafe { ffi::DrawPolyLines(center, sides as i32, radius, rotation, color) }
}
/// Draw the outline of a regular polygon (pentagon, hexagon, etc..) with thickness.
#[inline]
pub fn draw_poly_lines_ex(_rl: &DrawHandle, center: Vector2, sides: usize, radius: f32, rotation: f32, thickness: f32, color: Color) {
    unsafe { ffi::DrawPolyLinesEx(center, sides as i32, radius, rotation, thickness, color) }
}

/// Draw a linear spline. Draws nothing if less than 2 points are given.
#[inline]
pub fn draw_spline_linear(_rl: &DrawHandle, points: &[Vector2], thickness: f32, color: Color) {
    // SAFETY: points aren't modified
    unsafe { ffi::DrawSplineLinear(points.as_ptr().cast_mut(), points.len() as i32, thickness, color) }
}
/// Draw a B-Spline. Draws nothing if less than 4 points are given.
#[inline]
pub fn draw_spline_basis(_rl: &DrawHandle, points: &[Vector2], thickness: f32, color: Color) {
    // SAFETY: points aren't modified
    unsafe { ffi::DrawSplineBasis(points.as_ptr().cast_mut(), points.len() as i32, thickness, color) }
}
/// Draw a Catmull-Rom spline. Draws nothing if less than 4 points are given.
/// A Catmull-Rom spline goes through every point given.
#[inline]
pub fn draw_spline_catmull_rom(_rl: &DrawHandle, points: &[Vector2], thickness: f32, color: Color) {
    // SAFETY: points aren't modified
    unsafe { ffi::DrawSplineCatmullRom(points.as_ptr().cast_mut(), points.len() as i32, thickness, color) }
}
/// Draw a quadratic bezier spline. Draws nothing if less than 3 points are given.
/// Points should be given in order: [position, control, position, control, ...].
#[inline]
pub fn draw_spline_bezier_quadratic(_rl: &DrawHandle, points: &[Vector2], thickness: f32, color: Color) {
    // SAFETY: points aren't modified
    unsafe { ffi::DrawSplineBezierQuadratic(points.as_ptr().cast_mut(), points.len() as i32, thickness, color) }
}
/// Draw a cubic bezier spline. Draws nothing if less than 4 points are given.
/// Points should be given in order: [position, control, control, position, control, control, position, ...].
#[inline]
pub fn draw_spline_bezier_cubic(_rl: &DrawHandle, points: &[Vector2], thickness: f32, color: Color) {
    // SAFETY: points aren't modified
    unsafe { ffi::DrawSplineBezierCubic(points.as_ptr().cast_mut(), points.len() as i32, thickness, color) }
}
/// Draw a single linear spline.
#[inline]
pub fn draw_spline_seg_linear(_rl: &DrawHandle, p1: Vector2, p2: Vector2, thickness: f32, color: Color) {
    unsafe { ffi::DrawSplineSegmentLinear(p1, p2, thickness, color) }
}
/// Draw a single B-Spline.
#[inline]
pub fn draw_spline_seg_basis(_rl: &DrawHandle, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
    unsafe { ffi::DrawSplineSegmentBasis(p1, p2, p3, p4, thickness, color) }
}
/// Draw a single Catmull-Rom spline.
#[inline]
pub fn draw_spline_seg_catmull_rom(_rl: &DrawHandle, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
    unsafe { ffi::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thickness, color) }
}
/// Draw a single quadratic bezier spline.
#[inline]
pub fn draw_spline_seg_bezier_quad(_rl: &DrawHandle, p1: Vector2, p2: Vector2, p3: Vector2, thickness: f32, color: Color) {
    unsafe { ffi::DrawSplineSegmentBezierQuadratic(p1, p2, p3, thickness, color) }
}
/// Draw a single cubic bezier spline.
#[inline]
pub fn draw_spline_seg_bezier_cubic(_rl: &DrawHandle, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
    unsafe { ffi::DrawSplineSegmentBezierCubic(p1, p2, p3, p4, thickness, color) }
}

/// Evaluate a linear spline for `t` in `[0.0, 1.0]`
#[inline]
pub fn get_spline_point_linear(start: Vector2, end: Vector2, t: f32) -> Vector2 {
    unsafe { ffi::GetSplinePointLinear(start, end, t) }
}
/// Evaluate a linear spline for `t` in `[0.0, 1.0]`
#[inline]
pub fn get_spline_point_basis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2 {
    unsafe { ffi::GetSplinePointBasis(p1, p2, p3, p4, t) }
}
/// Evaluate a Catmull-Rom spline for `t` in `[0.0, 1.0]`
#[inline]
pub fn get_spline_point_catmull_rom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2 {
    unsafe { ffi::GetSplinePointCatmullRom(p1, p2, p3, p4, t) }
}
/// Evaluate a quadratic bezier spline for `t` in `[0.0, 1.0]`
#[inline]
pub fn get_spline_point_bezier_quad(p1: Vector2, p2: Vector2, p3: Vector2, t: f32) -> Vector2 {
    unsafe { ffi::GetSplinePointBezierQuad(p1, p2, p3, t) }
}
/// Evaluate a cubic bezier spline for `t` in `[0.0, 1.0]`
#[inline]
pub fn get_spline_point_bezier_cubic(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2 {
    unsafe { ffi::GetSplinePointBezierCubic(p1, p2, p3, p4, t) }
}
