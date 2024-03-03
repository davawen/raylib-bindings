pub use crate::ffi::{Camera, Camera2D, Camera3D, CameraMode, CameraProjection};

use crate::prelude::{Vector2, Ray, Matrix, Vector3};
use crate::ffi;

impl Camera {
    /// Get a ray trace from mouse position
    pub fn get_mouse_ray(&self, mouse_pos: Vector2) -> Ray {
        unsafe { ffi::GetMouseRay(mouse_pos, *self) }
    }

    /// Get this camera's view matrix
    pub fn get_matrix(&self) -> Matrix {
        unsafe { ffi::GetCameraMatrix(*self) }
    }

    /// Gets the screen space coordinate of a position in the current window
    /// Equivalent to calling `Camera::get_world_to_screen_ex(position, rl.get_screen_size())`
    pub fn get_world_to_screen(&self, position: Vector3) -> Vector2 {
        unsafe { ffi::GetWorldToScreen(position, *self) }
    }

    /// Gets the screen space coordinate of a position in the window of the given size
    pub fn get_world_to_screen_ex(&self, position: Vector3, size: Vector2) -> Vector2 {
        unsafe { ffi::GetWorldToScreenEx(position, *self, size.x as i32, size.y as i32) }
    }

    /// Update the camera position for the selected mode
    pub fn update_camera(&mut self, mode: CameraMode) {
        // SAFETY: rcamera is a separate module and shares no state with the rest of raylib
        unsafe { ffi::UpdateCamera(self as *mut Camera, mode as i32) }
    }

    /// Update the camera's values
    /// - `movement` is { x: forwards/backwards, y: right/left, z: up/down }
    /// - `rotation` is { x: yaw, y: pitch, z: roll }
    pub fn udpate_camera_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        // SAFETY: rcamera is a separate module and shares no state with the rest of raylib
        unsafe { ffi::UpdateCameraPro(self as *mut Camera, movement, rotation, zoom) }
    }
}

impl Camera2D {
    /// Gets this camera's 2d view matrix
    pub fn get_matrix2d(&self) -> Matrix {
        unsafe { ffi::GetCameraMatrix2D(*self) }
    }

    /// Gets the world space position for a given screen position in a 2d world
    pub fn get_screen_to_world(&self, position: Vector2) -> Vector2 {
        unsafe { ffi::GetScreenToWorld2D(position, *self) }
    }

    /// Gets the screen space position for a given world space position in a 2d world
    pub fn get_world_to_screen(&self, position: Vector2) -> Vector2 {
        unsafe { ffi::GetWorldToScreen2D(position, *self) }
    }
}
