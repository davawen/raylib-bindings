use std::ffi::CStr;

use crate::{Raylib, ffi::{KeyboardKey, self, MouseButton, GamepadButton}, Vector2};

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
    /// Will never return `KeyboardKey::Null`
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

/// A gamepad identifier
/// Use `Raylib::is_gamepad_available` to construct it
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gamepad(i32);

pub enum GamepadAxis {
    /// X and Y values of the left joystick
    Left,
    /// X and Y values of the right joystick
    Right,
    /// Values of the left and right triggers
    Trigger
}

impl Raylib {
    fn gamepad_available(&mut self, gamepad: i32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad) }
    }

    /// Checks if the given gamepad is available
    /// Returns Some(id) if the gamepad is available, and None otherwise
    /// # Examples
    /// ```
    /// # use raylib::{Raylib, GamepadButton, Color};
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// if let Some(gamepad) = rl.is_gamepad_available(0) {
    ///     if rl.is_button_down(gamepad, GamepadButton::ButtonMiddleLeft) {
    ///         rl.draw_circle(100.0, 100.0, 20.0, Color::RED);
    ///     }
    /// }
    /// ```
    pub fn is_gamepad_available(&mut self, gamepad: i32) -> Option<Gamepad> {
        if self.gamepad_available(gamepad) {
            Some(Gamepad(gamepad))
        } else {
            None
        }
    }

    /// Gets the gamepad's internal name id
    /// Returns `None` if:
    /// - it isn't available
    /// - it doesn't have a name (if the underlying raylib API returns a NULL pointer)
    /// - if its name is not valid utf-8. If you need its name regardless, you can use `get_name_cstr` directly.
    pub fn get_gamepad_name(&mut self, gamepad: Gamepad) -> Option<&str> {
        if !self.gamepad_available(gamepad.0) { return None }

        let cstr = self.get_gamepad_name_cstr(gamepad)?;
        cstr.to_str().ok()
    }

    /// Gets the gamepad's internal name id
    /// Returns `None` if it isn't available or it doesn't have a name (if the underlying raylib API returns a NULL pointer)
    /// # Examples 
    /// Get a `Cow<str>` with invalid characters removed:
    /// ```
    /// # use raylib::Raylib;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// if let Some(gamepad) = rl.is_gamepad_available(0) {
    ///     let name = rl.get_name_cstr(gamepad).map(|s| s.to_string_lossy());
    /// }
    /// ```
    pub fn get_gamepad_name_cstr(&mut self, gamepad: Gamepad) -> Option<&CStr> {
        if !self.gamepad_available(gamepad.0) { return None }

        unsafe { 
            let ptr = ffi::GetGamepadName(gamepad.0);
            if ptr.is_null() { return None }

            Some(CStr::from_ptr(ptr))
        }
    }

    /// Checks if a gamepad button has been pressed this frame (rising edge)
    /// Returns false if the gamepad isn't available
    pub fn is_gamepad_button_pressed(&mut self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonPressed(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button has been released this frame (falling edge)
    /// Returns false if the gamepad isn't available
    pub fn is_gamepad_button_released(&mut self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonReleased(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button is currently being held down
    /// Returns false if the gamepad isn't available
    pub fn is_gamepad_button_down(&mut self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonDown(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button is currently not being held down
    /// Returns false if the gamepad isn't available
    pub fn is_gamepad_button_up(&mut self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonUp(gamepad.0, button as i32) }
    }

    /// Get the last gamepad button pressed
    /// Returns `None` if there wasn't any
    /// Will never return `GamepadButton::Unknown`
    pub fn get_gamepad_button_pressed(&mut self) -> Option<GamepadButton> {
        let button = unsafe { ffi::GetGamepadButtonPressed() };

        if button == 0 { return None }
        button.try_into().ok()
    }

    /// Get the number of axis of the gamepad
    /// Returns 0 if the gamepad isn't available
    pub fn get_gamepad_axis_count(&mut self, gamepad: Gamepad) -> i32 {
        if !self.gamepad_available(gamepad.0) { return 0 }

        unsafe { ffi::GetGamepadAxisCount(gamepad.0) }
    }

    /// Get the axis movement for a given gamepad
    /// Returns `Vector2::ZERO` if the gamepad isn't available
    pub fn get_gamepad_axis_movement(&mut self, gamepad: Gamepad, axis: GamepadAxis) -> Vector2 {
        if !self.gamepad_available(gamepad.0) { return Vector2::ZERO }

        let (x, y) = match axis {
            GamepadAxis::Left => (ffi::GamepadAxis::LeftX, ffi::GamepadAxis::LeftY),
            GamepadAxis::Right => (ffi::GamepadAxis::LeftX, ffi::GamepadAxis::LeftY),
            GamepadAxis::Trigger => (ffi::GamepadAxis::LeftTrigger, ffi::GamepadAxis::RightTrigger),
        };

        Vector2::new(
            unsafe { ffi::GetGamepadAxisMovement(gamepad.0, x as i32) },
            unsafe { ffi::GetGamepadAxisMovement(gamepad.0, y as i32) },
        )
    }

    // TODO:
    // pub fn set_gamepad_mappings(&mut self, mappings: *const c_char) -> i32
}

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
