use super::Raylib;
use crate::ffi;

pub trait RaylibCursorFunctions {
    /// Shows cursor
    fn show_cursor(&self) { unsafe { ffi::ShowCursor() } }
    /// Hides cursor
    fn hide_cursor(&self) { unsafe { ffi::HideCursor() } }
    /// Checks if cursor is visible or not
    fn is_cursor_hidden(&self) -> bool { unsafe { ffi::IsCursorHidden() } }
    /// Unlocks cursor
    fn enable_cursor(&self) { unsafe { ffi::EnableCursor() } }
    /// Locks cursor
    fn disable_cursor(&self) { unsafe { ffi::DisableCursor() } }
    /// Checks if cursor is on the screen
    fn is_cursor_on_screen(&self) -> bool { unsafe { ffi::IsCursorOnScreen() } }
}

impl RaylibCursorFunctions for Raylib {}
