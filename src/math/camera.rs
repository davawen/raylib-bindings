use crate::ffi::{self, Vector2, Camera, Ray, Matrix, Vector3, Camera2D};

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
