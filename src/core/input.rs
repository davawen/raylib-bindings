use crate::{Raylib, ffi::{KeyboardKey, self, MouseButton}, Vector2};

// === Keyboard functions ===
impl Raylib {
    /// Check if a key has been pressed in this frame (rising edge)
    pub fn is_key_pressed(&mut self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed(key as i32) }
    }

    /// Check if a key has been pressed again (only on desktop platforms)
    pub fn is_key_pressed_again(&mut self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressedRepeat(key as i32) }
    }

    /// Check if a key has been released in this frame (falling edge)
    pub fn is_key_released(&mut self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased(key as i32) }
    }

    /// Check if a key is currently being pressed
    pub fn is_key_down(&mut self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }

    /// Check if a key is currently not being pressed
    pub fn is_key_up(&mut self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp(key as i32) }
    }

    /// Get the keycode of the key pressed
    /// Call multiple times to get queued presses
    /// Returns `None` when the queue is empty
    /// Will never return `KeyboardKey::KEY_NULL`
    pub fn get_key_pressed(&mut self) -> Option<KeyboardKey> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key == 0 { return None }
        KeyboardKey::try_from(key).ok()
    }

    /// Get the unicode character pressed
    /// Call multiple times to get queued presses
    /// Returns `None` when the queue is empty
    /// Will never return `'\0'`
    pub fn get_char_pressed(&mut self) -> Option<char> {
        let key = unsafe { ffi::GetCharPressed() };
        if key == 0 { return None }
        char::from_u32(key as u32)
    }

    /// Set a custom key to exit the program (default is ESC)
    pub fn set_exit_key(&mut self, key: KeyboardKey) {
        unsafe { ffi::SetExitKey(key as i32) }
    }
}

// TODO: Gamepad functions
// impl Raylib {
// }

// === Mouse functions ===
impl Raylib {
    /// Check if a mouse button has been pressed in this frame (rising edge)
    pub fn is_mouse_button_pressed(&mut self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Check if a mouse button is being pressed currently
    pub fn is_mouse_button_down(&mut self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Check if a mouse button has been release in this frame (falling edge)
    pub fn is_mouse_button_released(&mut self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Check if a mouse button is not being pressed currently
    pub fn is_mouse_button_up(&mut self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Gets the current X position of the mouse (relative to the window)
    pub fn get_mouse_x(&mut self) -> f32 {
        unsafe { ffi::GetMouseX() as f32 }
    }

    /// Gets the current Y position of the mouse (relative to the window)
    pub fn get_mouse_y(&mut self) -> f32 {
        unsafe { ffi::GetMouseY() as f32 }
    }

    /// Gets the current position of the mouse (relative to the window)
    pub fn get_mouse_pos(&mut self) -> Vector2 {
        unsafe { ffi::GetMousePosition() }
    }

    /// Gets how much the mouse has travelled between the last frame and the current frame
    pub fn get_mouse_delta(&mut self) -> Vector2 {
        unsafe { ffi::GetMouseDelta() }
    }

    /// Sets the mouse position (relative to the window)
    pub fn set_mouse_pos(&mut self, position: Vector2) {
        unsafe { ffi::SetMousePosition(position.x as i32, position.y as i32) }
    }

    /// Sets the mouse offset applied to the mouse position when getting it
    /// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
    pub fn set_mouse_offset(&mut self, offset: Vector2) {
        unsafe { ffi::SetMouseOffset(offset.x as i32, offset.y as i32) }
    }

    /// Sets the mouse scale applied to the mouse position when getting it
    /// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
    pub fn set_mouse_scale(&mut self, scale: Vector2) {
        unsafe { ffi::SetMouseScale(scale.x, scale.y) }
    }

    /// Gets the X or Y mouse wheel movement, whichever is larger 
    pub fn get_mouse_wheel_move(&mut self) -> f32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Gets the X and Y mouse wheel movement
    pub fn get_mouse_wheel_move_v(&mut self) -> Vector2 {
        unsafe { ffi::GetMouseWheelMoveV() }
    }
}
