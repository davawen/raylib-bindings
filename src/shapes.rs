use std::f32::consts::PI;

use ffi::Rectangle;

use crate::{core::draw::DrawHandle, prelude::{Color, Texture, Vector2, vec2}, ffi};

/// # Basic shapes drawing functions (module: `rshapes`)
///
/// ---
impl<P> DrawHandle<'_, P> {
    /// Set texture and rectangle to be used on shapes drawing.
    #[inline]
    pub fn set_shapes_texture(&mut self, texture: Texture, source: Rectangle) {
        unsafe { ffi::SetShapesTexture(texture.get_ffi_texture(), source) }
    }

    /// Draw a single pixel.
    #[inline]
    pub fn pixel(&mut self, pos_x: f32, pos_y: f32, color: Color) {
        self.pixel_v(vec2(pos_x, pos_y), color)
    }
    /// Draw a single pixel.
    #[inline]
    pub fn pixel_v(&mut self, pos: Vector2, color: Color) {
        unsafe { ffi::DrawPixelV(pos, color) }
    }
    /// Draw a line.
    #[inline]
    pub fn line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Color) {
        self.line_v(vec2(start_x, start_y), vec2(end_x, end_y), color)
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
        // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
        // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
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
        self.circle_v(vec2(center_x, center_y), radius, color)
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
        self.circle_lines_v(vec2(center_x, center_y), radius, color)
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
    /// Draw a filled rectangle.
    #[inline]
    pub fn rectangle(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
        self.rectangle_v(vec2(pos_x, pos_y), vec2(width, height), color)
    }
    /// Draw a filled rectangle.
    #[inline]
    pub fn rectangle_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        unsafe { ffi::DrawRectangleV(pos, size, color) }
    }
    /// Draw a filled rectangle.
    #[inline]
    pub fn rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        unsafe { ffi::DrawRectangleRec(rec, color) }
    }
    /// Draw a rotated filled rectangle.
    #[inline]
    pub fn rectangle_pro(&mut self, rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
        unsafe { ffi::DrawRectanglePro(rec, origin, rotation, color) }
    }
    /// Draw a horizontal-gradient-filled rectangle.
    /// Gradient goes from left to right.
    #[inline]
    pub fn rectangle_gradient_h(&mut self, rec: Rectangle, left: Color, right: Color) {
        unsafe { ffi::DrawRectangleGradientH(rec.x as i32, rec.y as i32, rec.width as i32, rec.height as i32, left, right) }
    }
    /// Draw a vertical-gradient-filled rectangle.
    /// Gradient goes from bottom to top.
    /// NOTE: While this behaviour is pretty weird, it is how raylib does it originally.
    #[inline]
    pub fn rectangle_gradient_v(&mut self, rec: Rectangle, bottom: Color, top: Color) {
        unsafe { ffi::DrawRectangleGradientV(rec.x as i32, rec.y as i32, rec.width as i32, rec.height as i32, bottom, top) }
    }
    /// Draw a gradient-filled rectangle with custom vertex colors.
    /// Each color refers to a corner, starting in the top-left and going counter clockwise.
    #[inline]
    pub fn rectangle_gradient_ex(&mut self, rec: Rectangle, top_left: Color, bot_left: Color, bot_right: Color, top_right: Color) {
        unsafe { ffi::DrawRectangleGradientEx(rec, top_left, bot_left, bot_right, top_right) }
    }
    /// Draw the outline of a rectangle.
    #[inline]
    pub fn rectangle_lines(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32, color: Color) {
        unsafe { ffi::DrawRectangleLines(pos_x as i32, pos_y as i32, width as i32, height as i32, color) }
    }
    /// Draw the outline of a rectangle.
    #[inline]
    pub fn rectangle_lines_v(&mut self, pos: Vector2, size: Vector2, color: Color) {
        self.rectangle_lines(pos.x, pos.y, size.x, size.y, color)
    }
    /// Draw the outline of a rectangle.
    #[inline]
    pub fn rectangle_lines_rec(&mut self, rec: Rectangle, color: Color) {
        self.rectangle_lines(rec.x, rec.y, rec.width, rec.height, color)
    }
    /// Draw the outline of a rectangle with line thickness.
    #[inline]
    pub fn rectangle_lines_ex(&mut self, rec: Rectangle, thick: f32, color: Color) {
        unsafe { ffi::DrawRectangleLinesEx(rec, thick, color) }
    }
    /// Draw a filled rectangle with rounded corners.
    /// `roundness` goes from 0.0 to 1.0.
    #[inline]
    pub fn rectangle_rounded(&mut self, rec: Rectangle, roundness: f32, segments: i32, color: Color) {
        unsafe { ffi::DrawRectangleRounded(rec, roundness, segments, color) }
    }
    /// Draw the outline of a rectangle with rounded corners and thickness.
    /// `roundness` goes from 0.0 to 1.0.
    #[inline]
    pub fn rectangle_rounded_lines(&mut self, rec: Rectangle, roundness: f32, segments: i32, thick: f32, color: Color) {
        unsafe { ffi::DrawRectangleRoundedLines(rec, roundness, segments, thick, color) }
    }
    /// Draw a filled triangle.
    /// WARN: The vertices MUST be given in counter-clockwise order! Otherwise, the triangle won't render!
    /// If you cannot be certain of the order of the vertices, use `triangle_unordered`.
    #[inline]
    pub fn triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { ffi::DrawTriangle(v1, v2, v3, color) }
    }
    /// Draw a filled triangle with vertices in any order.
    /// If you can be certain that your vertices are counter-clockwise, prefer using `triangle`.
    #[inline]
    pub fn triangle_unordered(&mut self, mut v1: Vector2, v2: Vector2, mut v3: Vector2, color: Color) {
        // calculate the determinant of matrix [[x1, y1, 1], [x2, y2, 1], [x3, y3, 1]]
        let det = v3.x*(v1.y - v2.y) + v1.x*(v2.y - v3.y) + v2.x*(-v1.y + v3.y);
        // positive determinant -> counter-clockwise, negative determinant -> clockwise
        if det < 0.0 {
            std::mem::swap(&mut v1, &mut v3);
        }

        self.triangle(v1, v2, v3, color)
    }
    /// Draw the outline of a triangle.
    /// WARN: The vertices MUST be given in counter-clockwise order! Otherwise, the triangle won't render!
    /// If you cannot be certain of the order of the vertices, use `triangle_lines_unordered`.
    #[inline]
    pub fn triangle_lines(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { ffi::DrawTriangleLines(v1, v2, v3, color) }
    }
    /// Draw the outline of a triangle with vertices in any order.
    /// If you can be certain that your vertices are counter-clockwise, prefer using `triangle_lines`.
    #[inline]
    pub fn triangle_lines_unordered(&mut self, mut v1: Vector2, v2: Vector2, mut v3: Vector2, color: Color) {
        // calculate the determinant of matrix [[x1, y1, 1], [x2, y2, 1], [x3, y3, 1]]
        let det = v3.x*(v1.y - v2.y) + v1.x*(v2.y - v3.y) + v2.x*(-v1.y + v3.y);
        // positive determinant -> counter-clockwise, negative determinant -> clockwise
        if det < 0.0 {
            std::mem::swap(&mut v1, &mut v3);
        }
        self.triangle_lines(v1, v2, v3, color)
    }
    /// Draw a filled triangle fan.
    /// The first vertex is the center and is shared by all triangles.
    /// Draws nothing if less than 3 points are given.
    /// WARN: The vertices must be given in counter-clockwise order!
    #[inline]
    pub fn triangle_fan(&mut self, points: &[Vector2], color: Color) {
        // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
        // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
        // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
        unsafe { ffi::DrawTriangleFan(points.as_ptr().cast_mut(), points.len() as i32, color) }
    }
    /// Draw a filled triangle strip.
    /// Every new vertex connects with the previous two to create a triangle.
    /// Draws nothing if less than 3 points are given.
    #[inline]
    pub fn triangle_strip(&mut self, points: &[Vector2], color: Color) {
        // NOTE: The cast_mut is sound, as the raylib API doesn't modify the input in any way, it simply forgot the specify the parameter as const.
        // However, that does mean that raylib can later change internally to modify the input (though that would be very very naughty).
        // Thus, this function could become unsound in a future version of raylib, and must be closely monitored.
        unsafe { ffi::DrawTriangleStrip(points.as_ptr().cast_mut(), points.len() as i32, color) }
    }
    /// Draw a filled regular polygon (pentagon, hexagon, etc..).
    #[inline]
    pub fn draw_poly(&mut self, center: Vector2, sides: usize, radius: f32, rotation: f32, color: Color) {
        unsafe { ffi::DrawPoly(center, sides as i32, radius, rotation, color) }
    }
    /// Draw the outline of a regular polygon (pentagon, hexagon, etc..).
    #[inline]
    pub fn draw_poly_lines(&mut self, center: Vector2, sides: usize, radius: f32, rotation: f32, color: Color) {
        unsafe { ffi::DrawPolyLines(center, sides as i32, radius, rotation, color) }
    }
    /// Draw the outline of a regular polygon (pentagon, hexagon, etc..) with thickness.
    #[inline]
    pub fn draw_poly_lines_ex(&mut self, center: Vector2, sides: usize, radius: f32, rotation: f32, thickness: f32, color: Color) {
        unsafe { ffi::DrawPolyLinesEx(center, sides as i32, radius, rotation, thickness, color) }
    }
}

/// # Spline drawing functions (module: `rshapes`)
///
/// ---
impl<P> DrawHandle<'_, P> {
    /// Draw a linear spline. Draws nothing if less than 2 points are given.
    #[inline]
    pub fn draw_spline_linear(&mut self, points: &mut [Vector2], thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineLinear(points.as_mut_ptr(), points.len() as i32, thickness, color) }
    }
    /// Draw a B-Spline. Draws nothing if less than 4 points are given.
    #[inline]
    pub fn draw_spline_basis(&mut self, points: &mut [Vector2], thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineBasis(points.as_mut_ptr(), points.len() as i32, thickness, color) }
    }
    /// Draw a Catmull-Rom spline. Draws nothing if less than 4 points are given.
    /// A Catmull-Rom spline goes through every point given.
    #[inline]
    pub fn draw_spline_catmull_rom(&mut self, points: &mut [Vector2], thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineCatmullRom(points.as_mut_ptr(), points.len() as i32, thickness, color) }
    }
    /// Draw a quadratic bezier spline. Draws nothing if less than 3 points are given.
    /// Points should be given in order: [position, control, position, control, ...].
    #[inline]
    pub fn draw_spline_bezier_quadratic(&mut self, points: &mut [Vector2], thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineBezierQuadratic(points.as_mut_ptr(), points.len() as i32, thickness, color) }
    }
    /// Draw a cubic bezier spline. Draws nothing if less than 4 points are given.
    /// Points should be given in order: [position, control, control, position, control, control, position, ...].
    #[inline]
    pub fn draw_spline_bezier_cubic(&mut self, points: &mut [Vector2], thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineBezierCubic(points.as_mut_ptr(), points.len() as i32, thickness, color) }
    }
    /// Draw a single linear spline.
    #[inline]
    pub fn draw_spline_seg_linear(&mut self, p1: Vector2, p2: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineSegmentLinear(p1, p2, thickness, color) }
    }
    /// Draw a single B-Spline.
    #[inline]
    pub fn draw_spline_seg_basis(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineSegmentBasis(p1, p2, p3, p4, thickness, color) }
    }
    /// Draw a single Catmull-Rom spline.
    #[inline]
    pub fn draw_spline_seg_catmull_rom(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thickness, color) }
    }
    /// Draw a single quadratic bezier spline.
    #[inline]
    pub fn draw_spline_seg_bezier_quad(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineSegmentBezierQuadratic(p1, p2, p3, thickness, color) }
    }
    /// Draw a single cubic bezier spline.
    #[inline]
    pub fn draw_spline_seg_bezier_cubic(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thickness: f32, color: Color) {
        unsafe { ffi::DrawSplineSegmentBezierCubic(p1, p2, p3, p4, thickness, color) }
    }
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

#[inline]
pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionRecs(rec1, rec2) }
}
#[inline]
pub fn check_collision_circles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1, radius1, center2, radius2) }
}
#[inline]
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionCircleRec(center, radius, rec) }
}
#[inline]
pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    unsafe { ffi::CheckCollisionPointRec(point, rec) }
}
#[inline]
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point, center, radius) }
}
#[inline]
pub fn check_collision_point_triangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point, p1, p2, p3) }
}
#[inline]
pub fn check_collision_point_poly(point: Vector2, points: &[Vector2]) -> bool {
    unsafe { ffi::CheckCollisionPointPoly(point, points.as_ptr().cast_mut(), points.len() as i32) }
}
/// Returns `Some` with the coordinate if there is a collision and `None` if there wasn't any.
#[inline]
pub fn check_collision_lines(start1: Vector2, end1: Vector2, start2: Vector2, end2: Vector2) -> Option<Vector2> {
    let mut p = Vector2::ZERO;
    let col = unsafe { ffi::CheckCollisionLines(start1, end1, start2, end2, &mut p as *mut _) };
    if col { Some(p) } else { None }
}
/// Checks if the given point belongs to the given line, within the given threshold in pixels
#[inline]
pub fn check_collision_point_line(point: Vector2, start: Vector2, end: Vector2, threshold: i32) -> bool {
   unsafe { ffi::CheckCollisionPointLine(point, start, end, threshold) }
}
/// Get the intersection between two rectangles
/// Returns None if the resulting intersection is empty
#[inline]
pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Option<Rectangle> {
    let rec = unsafe { ffi::GetCollisionRec(rec1, rec2) };
    if rec.is_empty() {
        return None
    }
    Some(rec)
}
