//! Module handling 2d and 3d collision code

use crate::{ffi, prelude::{Vector2, Vector3, BoundingBox, Ray, Rectangle}};

/// Ray hit information.
/// 
/// Not to be confused with [`ffi::RayCollision`], which encodes wether a hit happened in the struct itself.
/// An optional ray collision should prefer using `Option<RayHit>`.
#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    /// Coordinates of the hit point
    pub point: Vector3,
    /// Normal vector of the surface at the hit point
    pub normal: Vector3,
    /// Distance to the hit point
    pub distance: f32
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

/// Check collision between two spheres
#[inline]
pub fn check_collision_spheres(center1: Vector3, radius1: f32, center2: Vector3, radius2: f32) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center1, radius1, center2, radius2) }
}
/// Check collision between two bounding boxes
#[inline]
pub fn check_collision_boxes(a: BoundingBox, b: BoundingBox) -> bool {
    unsafe { ffi::CheckCollisionBoxes(a, b) }
}
/// Check collision between a box and a sphere
#[inline]
pub fn check_collision_box_sphere(bounds: BoundingBox, center: Vector3, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionBoxSphere(bounds, center, radius) }
}
/// Get collision info between a ray and a sphere
#[inline]
pub fn get_ray_collision_sphere(ray: Ray, center: Vector3, radius: f32) -> Option<RayHit> {
    let col = unsafe { ffi::GetRayCollisionSphere(ray, center, radius) };
    col.hit.then_some(RayHit {
        point: col.point,
        normal: col.normal,
        distance: col.distance
    })
}
/// Get collision info between a ray and a box
#[inline]
pub fn get_ray_collision_box(ray: Ray, bounds: BoundingBox) -> Option<RayHit> {
    let col = unsafe { ffi::GetRayCollisionBox(ray, bounds) };
    col.hit.then_some(RayHit {
        point: col.point,
        normal: col.normal,
        distance: col.distance
    })
}
// TODO: Mesh implementation
// /// Get collision info between a ray and a box
// #[inline]
// pub fn get_ray_collision_mesh(ray: Ray, mesh: Mesh, transform: Transform) -> Option<RayHit> {
//     let col = unsafe { ffi::GetRayCollisionBox(ray, bounds) };
//     col.hit.then_some(RayHit {
//         point: col.point,
//         normal: col.normal,
//         distance: col.distance
//     })
// }

/// Get collision info between a ray and a triangle
#[inline]
pub fn get_ray_collision_triangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> Option<RayHit> {
    let col = unsafe { ffi::GetRayCollisionTriangle(ray, p1, p2, p3) };
    col.hit.then_some(RayHit {
        point: col.point,
        normal: col.normal,
        distance: col.distance
    })
}
/// Get collision info between a ray and a quad
#[inline]
pub fn get_ray_collision_quad(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3) -> Option<RayHit> {
    let col = unsafe { ffi::GetRayCollisionQuad(ray, p1, p2, p3, p4) };
    col.hit.then_some(RayHit {
        point: col.point,
        normal: col.normal,
        distance: col.distance
    })
}
