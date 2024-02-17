use super::Raylib;
use crate::ffi;

impl Raylib {
    /// Shows cursor
    pub fn show_cursor(&self) { unsafe { ffi::ShowCursor() } }
    /// Hides cursor
    pub fn hide_cursor(&self) { unsafe { ffi::HideCursor() } }
    /// Checks if cursor is visible or not
    pub fn is_cursor_hidden(&self) -> bool { unsafe { ffi::IsCursorHidden() } }
    /// Unlocks cursor
    pub fn enable_cursor(&self) { unsafe { ffi::EnableCursor() } }
    /// Locks cursor
    pub fn disable_cursor(&self) { unsafe { ffi::DisableCursor() } }
    /// Checks if cursor is on the screen
    pub fn is_cursor_on_screen(&self) -> bool { unsafe { ffi::IsCursorOnScreen() } }
}

