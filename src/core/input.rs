use std::ffi::CStr;

use bitflags::bitflags;

use crate::{core::Raylib, ffi::{KeyboardKey, self, MouseButton, GamepadButton, Gesture, Vector2}};

/// Keyboard input functions (module: `rcore`)
pub trait RaylibKeyboardFunctions {
    /// Check if a key has been pressed in this frame (rising edge)
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed(key as i32) }
    }

    /// Check if a key has been pressed again (only on desktop platforms)
    fn is_key_pressed_again(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressedRepeat(key as i32) }
    }

    /// Check if a key has been released in this frame (falling edge)
    fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased(key as i32) }
    }

    /// Check if a key is currently being pressed
    fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown(key as i32) }
    }

    /// Check if a key is currently not being pressed
    fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp(key as i32) }
    }

    /// Get the keycode of the key pressed
    /// Call multiple times to get queued presses
    /// Returns `None` when the queue is empty
    /// Will never return `KeyboardKey::Null`
    fn get_key_pressed(&mut self) -> Option<KeyboardKey> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key == 0 { return None }
        KeyboardKey::try_from(key).ok()
    }

    /// Get the unicode character pressed
    /// Call multiple times to get queued presses
    /// Returns `None` when the queue is empty
    /// Will never return `'\0'`
    fn get_char_pressed(&mut self) -> Option<char> {
        let key = unsafe { ffi::GetCharPressed() };
        if key == 0 { return None }
        char::from_u32(key as u32)
    }

    /// Set a custom key to exit the program (default is ESC)
    fn set_exit_key(&mut self, key: KeyboardKey) {
        unsafe { ffi::SetExitKey(key as i32) }
    }
}

impl RaylibKeyboardFunctions for Raylib {}

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

/// Gamepad input functions (module: `rcore`)
pub trait RaylibGamepadFunctions {
    fn gamepad_available(&self, gamepad: i32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad) }
    }

    /// Checks if the given gamepad is available
    /// Returns Some(id) if the gamepad is available, and None otherwise
    /// There is maximum of 4 gamepads
    /// # Examples
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// # let mut draw = rl.begin_drawing();
    /// if let Some(gamepad) = rl.is_gamepad_available(0) {
    ///     if rl.is_gamepad_button_down(gamepad, GamepadButton::MiddleLeft) {
    ///         draw.circle(100.0, 100.0, 20.0, Color::RED);
    ///     }
    /// }
    /// ```
    fn is_gamepad_available(&self, gamepad: i32) -> Option<Gamepad> {
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
    fn get_gamepad_name(&self, gamepad: Gamepad) -> Option<&str> {
        if !self.gamepad_available(gamepad.0) { return None }

        let cstr = self.get_gamepad_name_cstr(gamepad)?;
        cstr.to_str().ok()
    }

    /// Gets the gamepad's internal name id
    /// Returns `None` if it isn't available or it doesn't have a name (if the underlying raylib API returns a NULL pointer)
    /// # Examples 
    /// Get a `Cow<str>` with invalid characters removed:
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// if let Some(gamepad) = rl.is_gamepad_available(0) {
    ///     let name = rl.get_gamepad_name_cstr(gamepad).map(|s| s.to_string_lossy());
    /// }
    /// ```
    fn get_gamepad_name_cstr(&self, gamepad: Gamepad) -> Option<&CStr> {
        if !self.gamepad_available(gamepad.0) { return None }

        unsafe { 
            let ptr = ffi::GetGamepadName(gamepad.0);
            if ptr.is_null() { return None }

            Some(CStr::from_ptr(ptr))
        }
    }

    /// Checks if a gamepad button has been pressed this frame (rising edge)
    /// Returns false if the gamepad isn't available
    fn is_gamepad_button_pressed(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonPressed(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button has been released this frame (falling edge)
    /// Returns false if the gamepad isn't available
    fn is_gamepad_button_released(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonReleased(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button is currently being held down
    /// Returns false if the gamepad isn't available
    fn is_gamepad_button_down(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonDown(gamepad.0, button as i32) }
    }

    /// Checks if a gamepad button is currently not being held down
    /// Returns false if the gamepad isn't available
    fn is_gamepad_button_up(&self, gamepad: Gamepad, button: GamepadButton) -> bool {
        if !self.gamepad_available(gamepad.0) { return false }
        unsafe { ffi::IsGamepadButtonUp(gamepad.0, button as i32) }
    }

    /// Get the last gamepad button pressed
    /// Returns `None` if there wasn't any
    /// Will never return `GamepadButton::Unknown`
    fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        let button = unsafe { ffi::GetGamepadButtonPressed() };

        if button == 0 { return None }
        button.try_into().ok()
    }

    /// Get the number of axis of the gamepad
    /// Returns 0 if the gamepad isn't available
    fn get_gamepad_axis_count(&self, gamepad: Gamepad) -> i32 {
        if !self.gamepad_available(gamepad.0) { return 0 }

        unsafe { ffi::GetGamepadAxisCount(gamepad.0) }
    }

    /// Get the axis movement for a given gamepad
    /// Returns `Vector2::ZERO` if the gamepad isn't available
    fn get_gamepad_axis_movement(&self, gamepad: Gamepad, axis: GamepadAxis) -> Vector2 {
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
    // fn set_gamepad_mappings(&mut self, mappings: *const c_char) -> i32
}

impl RaylibGamepadFunctions for Raylib {}

/// # Mouse input functions
///
/// ---
pub trait RaylibMouseFunctions {
    /// Checks if a mouse button has been pressed in this frame (rising edge).
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Checks if a mouse button is being pressed currently.
    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Checks if a mouse button has been release in this frame (falling edge).
    fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Checks if a mouse button is not being pressed currently.
    fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Gets the current X position of the mouse (relative to the window).
    fn get_mouse_x(&self) -> f32 {
        unsafe { ffi::GetMouseX() as f32 }
    }

    /// Gets the current Y position of the mouse (relative to the window).
    fn get_mouse_y(&self) -> f32 {
        unsafe { ffi::GetMouseY() as f32 }
    }

    /// Gets the current position of the mouse (relative to the window).
    fn get_mouse_pos(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition() }
    }

    /// Gets how much the mouse has travelled between the last frame and the current frame.
    fn get_mouse_delta(&self) -> Vector2 {
        unsafe { ffi::GetMouseDelta() }
    }

    /// Sets the mouse position (relative to the window).
    fn set_mouse_pos(&mut self, position: Vector2) {
        unsafe { ffi::SetMousePosition(position.x as i32, position.y as i32) }
    }

    /// Sets the mouse offset applied to the mouse position when getting it.
    /// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
    fn set_mouse_offset(&mut self, offset: Vector2) {
        unsafe { ffi::SetMouseOffset(offset.x as i32, offset.y as i32) }
    }

    /// Sets the mouse scale applied to the mouse position when getting it.
    /// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
    fn set_mouse_scale(&mut self, scale: Vector2) {
        unsafe { ffi::SetMouseScale(scale.x, scale.y) }
    }

    /// Gets the X or Y mouse wheel movement, whichever is larger.
    fn get_mouse_wheel_move(&self) -> f32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Gets the X and Y mouse wheel movement.
    fn get_mouse_wheel_move_v(&self) -> Vector2 {
        unsafe { ffi::GetMouseWheelMoveV() }
    }
}

impl RaylibMouseFunctions for Raylib {}

/// # Touch input functions
///
/// ---
pub trait RaylibTouchFunctions {
    /// Gets the X position for the first touch point.
    /// Returns `None` if there are no touch points.
    fn get_touch_x(&self) -> Option<f32> {
        if self.get_touch_point_count() == 0 { return None }
        unsafe { Some(ffi::GetTouchX() as f32) }
    }

    /// Gets the Y position for the first touch point.
    /// Returns `None` if there are no touch points.
    fn get_touch_y(&self) -> Option<f32> {
        if self.get_touch_point_count() == 0 { return None }
        unsafe { Some(ffi::GetTouchY() as f32) }
    }

    /// Gets the position for the first touch point.
    /// Returns `None` if there are no touch points.
    /// If you need a specific touch point index, use `Raylib::get_touch_position` instead.
    /// # Examples
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// # let mut draw = rl.begin_drawing();
    /// if let Some(pos) = rl.get_touch_pos() {
    ///     draw.circle_v(pos, 30.0, Color::ORANGE);
    /// }
    /// ```
    fn get_touch_pos(&self) -> Option<Vector2> {
        if self.get_touch_point_count() == 0 { return None }
        unsafe { Some(ffi::GetTouchPosition(0)) }
    }

    /// Gets the position for the given touch point.
    /// Returns `None` if the given index is over the number of touch points.
    /// If you need every touch position, prefer using `Raylib::get_touch_positions`
    fn get_touch_position(&self, index: usize) -> Option<Vector2> {
        if self.get_touch_point_count() < index { return None }
        unsafe { Some(ffi::GetTouchPosition(index as i32)) }
    }

    /// Returns an iterator over every touch position.
    /// # Examples
    /// Draw a circle at every touch point:
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// # let mut draw = rl.begin_drawing();
    /// for (idx, pos) in rl.get_touch_positions().enumerate() {
    ///     draw.circle_v(pos, 30.0, Color::ORANGE);
    ///     let pos = pos - Vector2::new(10.0, 70.0);
    ///     draw.text(rl.default_font(), &format!("{idx}"), pos, 40.0, Color::BLACK);
    /// }
    /// ```
    /// Get the identifier of every point:
    /// ```
    /// # use raylib::prelude::*;
    /// # let mut rl = Raylib::init_window(100, 100, "", 60);
    /// for (pos, id) in rl.get_touch_positions().zip(rl.get_touch_point_ids()) {
    ///     println!("At {pos:?}: {id}");
    /// }
    /// ```
    fn get_touch_positions(&self) -> impl Iterator<Item = Vector2> {
        (0..self.get_touch_point_count()).map(|index| {
            unsafe { ffi::GetTouchPosition(index as i32) }
        })
    }

    /// Get the touch point identifier of a given index.
    /// Returns `None` if the given index is over the number of touch points
    fn get_touch_point_id(&self, index: usize) -> Option<i32> {
        if self.get_touch_point_count() < index { return None } 
        unsafe { Some(ffi::GetTouchPointId(index as i32)) }
    }

    /// Returns an iterator over every touch id.
    fn get_touch_point_ids(&self) -> impl Iterator<Item = i32> {
        (0..self.get_touch_point_count()).map(|index| {
            unsafe { ffi::GetTouchPointId(index as i32) }
        })
    }

    /// Returns the number of touch points
    fn get_touch_point_count(&self) -> usize {
        unsafe { ffi::GetTouchPointCount() as usize }
    }
}

impl RaylibTouchFunctions for Raylib {}

bitflags! {
    pub struct GestureFlags: u32 {
        const NONE = Gesture::None as u32;
        const TAP = Gesture::Tap as u32;
        const DOUBLETAP = Gesture::Doubletap as u32;
        const HOLD = Gesture::Hold as u32;
        const DRAG = Gesture::Drag as u32;
        const SWIPE_RIGHT = Gesture::SwipeRight as u32;
        const SWIPE_LEFT = Gesture::SwipeLeft as u32;
        const SWIPE_UP = Gesture::SwipeUp as u32;
        const SWIPE_DOWN = Gesture::SwipeDown as u32;
        const PINCH_IN = Gesture::PinchIn as u32;
        const PINCH_OUT = Gesture::PinchOut as u32;
    }
}

/// # Gestures and touch handling functions (module: `rgesture`)
/// 
/// ---
pub trait RaylibGestureFunctions {
    /// Set which gestures are enabled using flags.
    fn set_gestures_enabled(&mut self, flags: GestureFlags) {
        unsafe { ffi::SetGesturesEnabled(flags.bits()) }
    }

    /// Checks if a gesture has been detected.
    /// Always returns false if the gesture wasn't enabled.
    fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        unsafe { ffi::IsGestureDetected(gesture as u32) }
    }

    /// Gets the latest gesture detected.
    /// Returns `None` if there wasn't any.
    /// Never returns `Gesture::None`.
    fn get_gesture_detected(&self) -> Option<Gesture> {
        let gesture = unsafe { ffi::GetGestureDetected() };
        if gesture == 0 { return None }

        gesture.try_into().ok()
    }

    /// Gets how long the current `Gesture::Hold` gesture is being held (in ms).
    /// Always returns `0.0 `if there is no current gesture or if it is not a hold.
    fn get_gesture_hold_duration(&self) -> f32 {
        unsafe { ffi::GetGestureHoldDuration() }
    }

    /// Gets the vector between the initial touch point and the current one.
    /// Works for Holds, Drags and Swipes. 
    /// Otherwise, returns `Vector2::ZERO`.
    fn get_gesture_drag_vector(&self) -> Vector2 {
        unsafe { ffi::GetGestureDragVector() }
    }

    /// Gets the angle between the initial touch point and the current one.
    /// Works for Hold, Drag and Swipe gestures. 
    /// Otherwise, returns `0.0`.
    fn get_gesture_drag_angle(&self) -> f32 {
        unsafe { ffi::GetGestureDragAngle() }
    }

    /// Gets the distance between the two pinch points.
    /// On gestures other than a pinch, returns `Vector2::ZERO`.
    fn get_gesture_pinch_vector(&self) -> Vector2 {
        unsafe { ffi::GetGesturePinchVector() }
    }

    /// Gets the angle between the two pinch points.
    /// On gestures other than a pinch, returns `0.0`.
    fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe { ffi::GetGesturePinchAngle() }
    }
}

impl RaylibGestureFunctions for Raylib {}
