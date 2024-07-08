//! Input related functions (keyboard, mouse, touch, gestures)

use std::ffi::CStr;

use bitflags::bitflags;

use crate::{core::Raylib, ffi::{KeyboardKey, self, MouseButton, GamepadButton, Gesture, Vector2}, prelude::vec2};

/// Check if a key has been pressed in this frame (rising edge)
pub fn is_key_pressed(_: &Raylib, key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyPressed(key as i32) }
}

/// Check if a key has been pressed again (only on desktop platforms)
pub fn is_key_pressed_again(_: &Raylib, key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyPressedRepeat(key as i32) }
}

/// Check if a key has been released in this frame (falling edge)
pub fn is_key_released(_: &Raylib, key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyReleased(key as i32) }
}

/// Check if a key is currently being pressed
pub fn is_key_down(_: &Raylib, key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyDown(key as i32) }
}

/// Check if a key is currently not being pressed
pub fn is_key_up(_: &Raylib, key: KeyboardKey) -> bool {
    unsafe { ffi::IsKeyUp(key as i32) }
}

/// Get the keycode of the key pressed
/// Call multiple times to get queued presses
/// Returns `None` when the queue is empty
/// Will never return `KeyboardKey::Null`
pub fn get_key_pressed(_: &mut Raylib) -> Option<KeyboardKey> {
    let key = unsafe { ffi::GetKeyPressed() };
    if key == 0 { return None }
    KeyboardKey::try_from(key).ok()
}

/// Get the unicode character pressed
/// Call multiple times to get queued presses
/// Returns `None` when the queue is empty
/// Will never return `'\0'`
pub fn get_char_pressed(_: &mut Raylib) -> Option<char> {
    let key = unsafe { ffi::GetCharPressed() };
    if key == 0 { return None }
    char::from_u32(key as u32)
}

/// Set a custom key to exit the program (default is ESC)
pub fn set_exit_key(_: &mut Raylib, key: KeyboardKey) {
    unsafe { ffi::SetExitKey(key as i32) }
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

pub fn gamepad_available(_: &Raylib, gamepad: i32) -> bool {
    unsafe { ffi::IsGamepadAvailable(gamepad) }
}

/// Checks if the given gamepad is available
/// Returns Some(id) if the gamepad is available, and None otherwise
/// There is maximum of 4 gamepads
/// # Examples
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// # begin_drawing(rl, |rl| {
/// if let Some(gamepad) = is_gamepad_available(rl, 0) {
///     if is_gamepad_button_down(rl, gamepad, GamepadButton::MiddleLeft) {
///         draw_circle(rl, 100.0, 100.0, 20.0, Color::RED);
///     }
/// }
/// # });
/// ```
pub fn is_gamepad_available(rl: &Raylib, gamepad: i32) -> Option<Gamepad> {
    if gamepad_available(rl, gamepad) {
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
pub fn get_gamepad_name(rl: &Raylib, gamepad: Gamepad) -> Option<&str> {
    if !gamepad_available(rl, gamepad.0) { return None }

    let cstr = get_gamepad_name_cstr(rl, gamepad)?;
    cstr.to_str().ok()
}

/// Gets the gamepad's internal name id
/// Returns `None` if it isn't available or it doesn't have a name (if the underlying raylib API returns a NULL pointer)
/// # Examples 
/// Get a `Cow<str>` with invalid characters removed:
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// if let Some(gamepad) = is_gamepad_available(rl, 0) {
///     let name = get_gamepad_name_cstr(rl, gamepad).map(|s| s.to_string_lossy());
/// }
/// ```
pub fn get_gamepad_name_cstr(rl: &Raylib, gamepad: Gamepad) -> Option<&CStr> {
    if !gamepad_available(rl, gamepad.0) { return None }

    unsafe { 
        let ptr = ffi::GetGamepadName(gamepad.0);
        if ptr.is_null() { return None }

        Some(CStr::from_ptr(ptr))
    }
}

/// Checks if a gamepad button has been pressed this frame (rising edge)
/// Returns false if the gamepad isn't available
pub fn is_gamepad_button_pressed(rl: &Raylib, gamepad: Gamepad, button: GamepadButton) -> bool {
    if !gamepad_available(rl, gamepad.0) { return false }
    unsafe { ffi::IsGamepadButtonPressed(gamepad.0, button as i32) }
}

/// Checks if a gamepad button has been released this frame (falling edge)
/// Returns false if the gamepad isn't available
pub fn is_gamepad_button_released(rl: &Raylib, gamepad: Gamepad, button: GamepadButton) -> bool {
    if !gamepad_available(rl, gamepad.0) { return false }
    unsafe { ffi::IsGamepadButtonReleased(gamepad.0, button as i32) }
}

/// Checks if a gamepad button is currently being held down
/// Returns false if the gamepad isn't available
pub fn is_gamepad_button_down(rl: &Raylib, gamepad: Gamepad, button: GamepadButton) -> bool {
    if !gamepad_available(rl, gamepad.0) { return false }
    unsafe { ffi::IsGamepadButtonDown(gamepad.0, button as i32) }
}

/// Checks if a gamepad button is currently not being held down
/// Returns false if the gamepad isn't available
pub fn is_gamepad_button_up(rl: &Raylib, gamepad: Gamepad, button: GamepadButton) -> bool {
    if !gamepad_available(rl, gamepad.0) { return false }
    unsafe { ffi::IsGamepadButtonUp(gamepad.0, button as i32) }
}

/// Get the last gamepad button pressed
/// Returns `None` if there wasn't any
/// Will never return `GamepadButton::Unknown`
pub fn get_gamepad_button_pressed(_: &Raylib) -> Option<GamepadButton> {
    let button = unsafe { ffi::GetGamepadButtonPressed() };

    if button == 0 { return None }
    button.try_into().ok()
}

/// Get the number of axis of the gamepad
/// Returns 0 if the gamepad isn't available
pub fn get_gamepad_axis_count(rl: &Raylib, gamepad: Gamepad) -> i32 {
    if !gamepad_available(rl, gamepad.0) { return 0 }

    unsafe { ffi::GetGamepadAxisCount(gamepad.0) }
}

/// Get the axis movement for a given gamepad
/// Returns `Vector2::ZERO` if the gamepad isn't available
pub fn get_gamepad_axis_movement(rl: &Raylib, gamepad: Gamepad, axis: GamepadAxis) -> Vector2 {
    if !gamepad_available(rl, gamepad.0) { return Vector2::ZERO }

    let (x, y) = match axis {
        GamepadAxis::Left => (ffi::GamepadAxis::LeftX, ffi::GamepadAxis::LeftY),
        GamepadAxis::Right => (ffi::GamepadAxis::LeftX, ffi::GamepadAxis::LeftY),
        GamepadAxis::Trigger => (ffi::GamepadAxis::LeftTrigger, ffi::GamepadAxis::RightTrigger),
    };

    vec2(
        unsafe { ffi::GetGamepadAxisMovement(gamepad.0, x as i32) },
        unsafe { ffi::GetGamepadAxisMovement(gamepad.0, y as i32) },
    )
}

// TODO:
// fn set_gamepad_mappings(&mut self, mappings: *const c_char) -> i32

/// Checks if a mouse button has been pressed in this frame (rising edge).
pub fn is_mouse_button_pressed(_: &Raylib, button: MouseButton) -> bool {
    unsafe { ffi::IsMouseButtonPressed(button as i32) }
}

/// Checks if a mouse button is being pressed currently.
pub fn is_mouse_button_down(_: &Raylib, button: MouseButton) -> bool {
    unsafe { ffi::IsMouseButtonDown(button as i32) }
}

/// Checks if a mouse button has been release in this frame (falling edge).
pub fn is_mouse_button_released(_: &Raylib, button: MouseButton) -> bool {
    unsafe { ffi::IsMouseButtonReleased(button as i32) }
}

/// Checks if a mouse button is not being pressed currently.
pub fn is_mouse_button_up(_: &Raylib, button: MouseButton) -> bool {
    unsafe { ffi::IsMouseButtonUp(button as i32) }
}

/// Gets the current X position of the mouse (relative to the window).
pub fn get_mouse_x(_: &Raylib) -> f32 {
    unsafe { ffi::GetMouseX() as f32 }
}

/// Gets the current Y position of the mouse (relative to the window).
pub fn get_mouse_y(_: &Raylib) -> f32 {
    unsafe { ffi::GetMouseY() as f32 }
}

/// Gets the current position of the mouse (relative to the window).
pub fn get_mouse_pos(_: &Raylib) -> Vector2 {
    unsafe { ffi::GetMousePosition() }
}

/// Gets how much the mouse has travelled between the last frame and the current frame.
pub fn get_mouse_delta(_: &Raylib) -> Vector2 {
    unsafe { ffi::GetMouseDelta() }
}

/// Sets the mouse position (relative to the window).
pub fn set_mouse_pos(_: &mut Raylib, position: Vector2) {
    unsafe { ffi::SetMousePosition(position.x as i32, position.y as i32) }
}

/// Sets the mouse offset applied to the mouse position when getting it.
/// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
pub fn set_mouse_offset(_: &mut Raylib, offset: Vector2) {
    unsafe { ffi::SetMouseOffset(offset.x as i32, offset.y as i32) }
}

/// Sets the mouse scale applied to the mouse position when getting it.
/// `mouse_pos = (real_mouse_pos + mouse_offset)*mouse_scale`
pub fn set_mouse_scale(_: &mut Raylib, scale: Vector2) {
    unsafe { ffi::SetMouseScale(scale.x, scale.y) }
}

/// Gets the X or Y mouse wheel movement, whichever is larger.
pub fn get_mouse_wheel_move(_: &Raylib) -> f32 {
    unsafe { ffi::GetMouseWheelMove() }
}

/// Gets the X and Y mouse wheel movement.
pub fn get_mouse_wheel_move_v(_: &Raylib) -> Vector2 {
    unsafe { ffi::GetMouseWheelMoveV() }
}

/// Gets the X position for the first touch point.
/// Returns `None` if there are no touch points.
pub fn get_touchx(rl: &Raylib) -> Option<f32> {
    if get_touch_point_count(rl) == 0 { return None }
    unsafe { Some(ffi::GetTouchX() as f32) }
}

/// Gets the Y position for the first touch point.
/// Returns `None` if there are no touch points.
pub fn get_touch_y(rl: &Raylib) -> Option<f32> {
    if get_touch_point_count(rl) == 0 { return None }
    unsafe { Some(ffi::GetTouchY() as f32) }
}

/// Gets the position for the first touch point.
/// Returns `None` if there are no touch points.
/// If you need a specific touch point index, use `Raylib::get_touch_position` instead.
/// # Examples
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// # begin_drawing(rl, |rl| {
/// if let Some(pos) = get_touch_pos(rl) {
///     draw_circle_v(rl, pos, 30.0, Color::ORANGE);
/// }
/// # });
/// ```
pub fn get_touch_pos(rl: &Raylib) -> Option<Vector2> {
    if get_touch_point_count(rl) == 0 { return None }
    unsafe { Some(ffi::GetTouchPosition(0)) }
}

/// Gets the position for the given touch point.
/// Returns `None` if the given index is over the number of touch points.
/// If you need every touch position, prefer using `Raylib::get_touch_positions`
pub fn get_touch_position(rl: &Raylib, index: usize) -> Option<Vector2> {
    if get_touch_point_count(rl) < index { return None }
    unsafe { Some(ffi::GetTouchPosition(index as i32)) }
}

/// Returns an iterator over every touch position.
/// # Examples
/// Draw a circle at every touch point:
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// # begin_drawing(rl, |rl| {
/// for (idx, pos) in get_touch_positions(rl).enumerate() {
///     draw_circle_v(rl, pos, 30.0, Color::ORANGE);
///     let pos = pos - vec2(10.0, 70.0);
///     rl.text(rl.default_font(), &format!("{idx}"), pos, 40.0, Color::BLACK);
/// }
/// # });
/// ```
/// Get the identifier of every point:
/// ```
/// # use raylib::prelude::*;
/// # let rl = &mut init_window(100, 100, "", 60);
/// for (pos, id) in get_touch_positions(rl).zip(get_touch_point_ids(rl)) {
///     println!("At {pos:?}: {id}");
/// }
/// ```
pub fn get_touch_positions<'a, 'b>(rl: &'a Raylib) -> impl Iterator<Item = Vector2> + 'b {
    (0..get_touch_point_count(rl)).map(|index| {
        unsafe { ffi::GetTouchPosition(index as i32) }
    })
}

/// Get the touch point identifier of a given index.
/// Returns `None` if the given index is over the number of touch points
pub fn get_touch_point_id(rl: &Raylib, index: usize) -> Option<i32> {
    if get_touch_point_count(rl) < index { return None } 
    unsafe { Some(ffi::GetTouchPointId(index as i32)) }
}

/// Returns an iterator over every touch id.
pub fn get_touch_point_ids(rl: &Raylib) -> impl Iterator<Item = i32> {
    (0..get_touch_point_count(rl)).map(|index| {
        unsafe { ffi::GetTouchPointId(index as i32) }
    })
}

/// Returns the number of touch points
pub fn get_touch_point_count(_: &Raylib) -> usize {
    unsafe { ffi::GetTouchPointCount() as usize }
}

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

/// Set which gestures are enabled using flags.
pub fn set_gestures_enabled(_: &mut Raylib, flags: GestureFlags) {
    unsafe { ffi::SetGesturesEnabled(flags.bits()) }
}

/// Checks if a gesture has been detected.
/// Always returns false if the gesture wasn't enabled.
pub fn is_gesture_detected(_: &Raylib, gesture: Gesture) -> bool {
    unsafe { ffi::IsGestureDetected(gesture as u32) }
}

/// Gets the latest gesture detected.
/// Returns `None` if there wasn't any.
/// Never returns `Gesture::None`.
pub fn get_gesture_detected(_: &Raylib) -> Option<Gesture> {
    let gesture = unsafe { ffi::GetGestureDetected() };
    if gesture == 0 { return None }

    gesture.try_into().ok()
}

/// Gets how long the current `Gesture::Hold` gesture is being held (in ms).
/// Always returns `0.0 `if there is no current gesture or if it is not a hold.
pub fn get_gesture_hold_duration(_: &Raylib) -> f32 {
    unsafe { ffi::GetGestureHoldDuration() }
}

/// Gets the vector between the initial touch point and the current one.
/// Works for Holds, Drags and Swipes. 
/// Otherwise, returns `Vector2::ZERO`.
pub fn get_gesture_drag_vector(_: &Raylib) -> Vector2 {
    unsafe { ffi::GetGestureDragVector() }
}

/// Gets the angle between the initial touch point and the current one.
/// Works for Hold, Drag and Swipe gestures. 
/// Otherwise, returns `0.0`.
pub fn get_gesture_drag_angle(_: &Raylib) -> f32 {
    unsafe { ffi::GetGestureDragAngle() }
}

/// Gets the distance between the two pinch points.
/// On gestures other than a pinch, returns `Vector2::ZERO`.
pub fn get_gesture_pinch_vector(_: &Raylib) -> Vector2 {
    unsafe { ffi::GetGesturePinchVector() }
}

/// Gets the angle between the two pinch points.
/// On gestures other than a pinch, returns `0.0`.
pub fn get_gesture_pinch_angle(_: &Raylib) -> f32 {
    unsafe { ffi::GetGesturePinchAngle() }
}
