//! Cursor functions (module: `rcore`)

use crate::prelude::{Raylib, MouseCursor};
use crate::ffi;

/// Shows cursor
pub fn show_cursor(_: &Raylib) { unsafe { ffi::ShowCursor() } }
/// Hides cursor
pub fn hide_cursor(_: &Raylib) { unsafe { ffi::HideCursor() } }
/// Checks if cursor is visible or not
pub fn is_cursor_hidden(_: &Raylib) -> bool { unsafe { ffi::IsCursorHidden() } }
/// Unlocks cursor
pub fn enable_cursor(_: &Raylib) { unsafe { ffi::EnableCursor() } }
/// Locks cursor
pub fn disable_cursor(_: &Raylib) { unsafe { ffi::DisableCursor() } }
/// Checks if cursor is on the screen
pub fn is_cursor_on_screen(_: &Raylib) -> bool { unsafe { ffi::IsCursorOnScreen() } }

/// Set the current mouse cursor kind
pub fn set_mouse_cursor(_: &mut Raylib, cursor: MouseCursor) { unsafe { ffi::SetMouseCursor(cursor as i32) } }
