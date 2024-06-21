//! Window functions (module: `rcore`)

use std::{ffi::{CStr, CString}, mem::ManuallyDrop};
use crate::{ffi::{self, Image, MouseCursor}, prelude::{Vector2, vec2}};

use bitflags::bitflags;

use super::Raylib;

bitflags! {
    pub struct ConfigFlags: u32 {
        /// Set to try enabling V-Sync on GPU
        const FLAG_VSYNC_HINT = ffi::ConfigFlags::VsyncHint as u32;
        /// Set to run program in fullscreen
        const FLAG_FULLSCREEN_MODE= ffi::ConfigFlags::FullscreenMode as u32;
        /// Set to allow resizable window
        const FLAG_WINDOW_RESIZABLE = ffi::ConfigFlags::WindowResizable as u32;
        /// Set to disable window decoration (frame and buttons)
        const FLAG_WINDOW_UNDECORATED = ffi::ConfigFlags::WindowUndecorated as u32;
        /// Set to hide window
        const FLAG_WINDOW_HIDDEN= ffi::ConfigFlags::WindowHidden as u32;
        /// Set to minimize window (iconify)
        const FLAG_WINDOW_MINIMIZED = ffi::ConfigFlags::WindowMinimized as u32;
        /// Set to maximize window (expanded to monitor)
        const FLAG_WINDOW_MAXIMIZED = ffi::ConfigFlags::WindowMaximized as u32;
        /// Set to window non focused
        const FLAG_WINDOW_UNFOCUSED = ffi::ConfigFlags::WindowUnfocused as u32;
        /// Set to window always on top
        const FLAG_WINDOW_TOPMOST = ffi::ConfigFlags::WindowTopmost as u32;
        /// Set to allow windows running while minimized
        const FLAG_WINDOW_ALWAYS_RUN= ffi::ConfigFlags::WindowAlwaysRun as u32;
        /// Set to allow transparent framebuffer
        const FLAG_WINDOW_TRANSPARENT = ffi::ConfigFlags::WindowTransparent as u32;
        /// Set to support HighDPI
        const FLAG_WINDOW_HIGHDPI = ffi::ConfigFlags::WindowHighdpi as u32;
        /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
        const FLAG_WINDOW_MOUSE_PASSTHROUGH = ffi::ConfigFlags::WindowMousePassthrough as u32;
        /// Set to run program in borderless windowed mode
        const FLAG_BORDERLESS_WINDOWED_MODE = ffi::ConfigFlags::BorderlessWindowedMode as u32;
        /// Set to try enabling MSAA 4X
        const FLAG_MSAA_4X_HINT = ffi::ConfigFlags::Msaa4XHint as u32;
        /// Set to try enabling interlaced video format (for V3D)
        const FLAG_INTERLACED_HINT= ffi::ConfigFlags::InterlacedHint as u32;
    }
}

/// Initializes raylib.
/// # Example
/// Basic main loop:
/// ```
/// use raylib::prelude::*;
/// let rl = &mut init_window(800, 800, "Raylib bindings!", 60);
/// while !window_should_close(rl) {
///     rl.begin_drawing(|rl| {
///         rl.clear_background(Color::RAYWHITE);
///     });
///     # break
/// }
/// ```
/// # Panics
/// Panics if the given title contains null characters
pub fn init_window(width: i32, height: i32, title: &str, target_fps: i32) -> Raylib {
    let rl = init_window_cstr(width, height, CString::new(title).expect("a title without null characters").as_c_str());
    rl.set_target_fps(target_fps);
    rl
}

pub fn init_window_cstr(width: i32, height: i32, title: &CStr) -> Raylib {
    unsafe { ffi::InitWindow(width, height, title.as_ptr()) }
    let mut this = Raylib { 
        automation_event_set: false,
        automation_event_recording: false,
        quit_requested: false,
        default_font: ManuallyDrop::new(None).into(),
        _private: std::marker::PhantomData
    };
    this.load_default_font();
    this
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn window_should_close(rl: &Raylib) -> bool {
    if rl.quit_requested { return true }
    unsafe { ffi::WindowShouldClose() }
}

/// Asks raylib to close the window (will return false in the next iteration of [`Raylib::window_should_close`]).
pub fn quit(rl: &mut Raylib) {
    rl.quit_requested = true;
}

/// Check if window has been initialized successfully
pub fn is_window_ready(_: &Raylib) -> bool { unsafe { ffi::IsWindowReady() } }
/// Check if window is currently fullscreen
pub fn is_window_fullscreen(_: &Raylib) -> bool { unsafe { ffi::IsWindowFullscreen() } }
/// Check if window is currently hidden (only PLATFORM_DESKTOP)
pub fn is_window_hidden(_: &Raylib) -> bool { unsafe { ffi::IsWindowHidden() } }
/// Check if window is currently minimized (only PLATFORM_DESKTOP)
pub fn is_window_minimized(_: &Raylib) -> bool { unsafe { ffi::IsWindowMinimized() } }
/// Check if window is currently maximized (only PLATFORM_DESKTOP)
pub fn is_window_maximized(_: &Raylib) -> bool { unsafe { ffi::IsWindowMaximized() } }
/// Check if window is currently focused (only PLATFORM_DESKTOP)
pub fn is_window_focused(_: &Raylib) -> bool { unsafe { ffi::IsWindowFocused() } }
/// Check if window has been resized last frame
pub fn is_window_resized(_: &Raylib) -> bool { unsafe { ffi::IsWindowResized() } }
/// Check if one specific window flag is enabled
pub fn is_window_state(_: &Raylib, flag: ffi::ConfigFlags) -> bool { unsafe { ffi::IsWindowState(flag as u32) } }                      
/// Set window configuration state using flags (only PLATFORM_DESKTOP)
pub fn set_window_state(_: &mut Raylib, flags: ConfigFlags) { unsafe { ffi::SetWindowState(flags.bits()) } }
/// Clear the window configuration state flags.
/// That is, set the given flags to false.
pub fn clear_window_state(_: &mut Raylib, flags: ConfigFlags) { unsafe { ffi::ClearWindowState(flags.bits()) } }
/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
pub fn toggle_fullscreen(_: &mut Raylib) { unsafe { ffi::ToggleFullscreen() } }
/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
pub fn toggle_borderless_windowed(_: &mut Raylib) { unsafe { ffi::ToggleBorderlessWindowed() } }
/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
pub fn maximize_window(_: &mut Raylib) { unsafe { ffi::MaximizeWindow() } }
/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
pub fn minimize_window(_: &mut Raylib) { unsafe { ffi::MinimizeWindow() } }
/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
pub fn restore_window(_: &mut Raylib) { unsafe { ffi::RestoreWindow() } }
/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn set_window_icon(_: &mut Raylib, image: Image) { unsafe { ffi::SetWindowIcon(image) } }
/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
pub fn set_window_icons(_: &mut Raylib, images: &[Image]) {
    unsafe { ffi::SetWindowIcons(images.as_ptr() as *mut Image, images.len() as i32) }
}
/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
pub fn set_window_title(_: &mut Raylib, title: &CStr) { unsafe { ffi::SetWindowTitle(title.as_ptr()) } }
/// Set window position on screen (only PLATFORM_DESKTOP)
pub fn set_window_position(_: &mut Raylib, x: i32, y: i32) { unsafe { ffi::SetWindowPosition(x, y) } }
/// TODO: Safer handling for monitor indices
/// Set monitor for the current window
pub fn set_window_monitor(_: &mut Raylib, monitor: i32) { unsafe { ffi::SetWindowMonitor(monitor) } }
/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(_: &mut Raylib, width: i32, height: i32) { unsafe { ffi::SetWindowMinSize(width, height) } }
/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_max_size(_: &mut Raylib, width: i32, height: i32) { unsafe { ffi::SetWindowMaxSize(width, height) } }
/// Set window dimensions
pub fn set_window_size(_: &mut Raylib, width: i32, height: i32) { unsafe { ffi::SetWindowSize(width, height) } }
/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
pub fn set_window_opacity(_: &mut Raylib, opacity: f32) { unsafe { ffi::SetWindowOpacity(opacity) } }


/// Set window focused (only PLATFORM_DESKTOP)
pub fn set_window_focused(_: &Raylib) { unsafe { ffi::SetWindowFocused() } }

// TODO:
// /// Get native window handle
// pub fn GetWindowHandle(void) -> void * ;

/// Gets the current screen (window) width
/// If you need to consider HiDPI, use `Raylib::get_render_width`
pub fn get_screen_width(_: &Raylib) -> f32 { unsafe { ffi::GetScreenWidth() as f32 } }
/// Gets the current screen (window) height
/// If you need to consider HiDPI, use `Raylib::get_render_width`
pub fn get_screen_height(_: &Raylib) -> f32 { unsafe { ffi::GetScreenHeight() as f32 } }
/// Gets the current screen (window) size as a vector
/// If you need to consider HiDPI, use `Raylib::get_render_size`
pub fn get_screen_size(rl: &Raylib) -> Vector2 { vec2(get_screen_width(rl), get_screen_height(rl)) }
/// Gets the current render width (considering HiDPI)
pub fn get_render_width(_: &Raylib) -> f32 { unsafe { ffi::GetRenderWidth() as f32 } }
/// Gets the current render height (considering HiDPI)
pub fn get_render_height(_: &Raylib) -> f32 { unsafe { ffi::GetRenderHeight() as f32 } }
/// Gets the current render size as vector (considering HiDPI)
pub fn get_render_size(rl: &Raylib) -> Vector2 { vec2(get_render_width(rl), get_render_height(rl)) }

/// Get number of connected monitors
pub fn get_monitor_count(_: &Raylib) -> i32 { unsafe { ffi::GetMonitorCount() } }
/// Get current connected monitor
pub fn get_current_monitor(_: &Raylib) -> i32 { unsafe { ffi::GetCurrentMonitor() } }
/// Get specified monitor position
pub fn get_monitor_position(_: &Raylib, monitor: i32) -> Vector2 { unsafe { ffi::GetMonitorPosition(monitor) } }
/// Get specified monitor width (current video mode used by monitor)
pub fn get_monitor_width(_: &Raylib, monitor: i32) -> i32 { unsafe { ffi::GetMonitorWidth(monitor) } }
/// Get specified monitor height (current video mode used by monitor)
pub fn get_monitor_height(_: &Raylib, monitor: i32) -> i32 { unsafe { ffi::GetMonitorHeight(monitor) } }
/// Get specified monitor physical width in millimetres
pub fn get_monitor_physical_width(_: &Raylib, monitor: i32) -> i32 { unsafe { ffi::GetMonitorPhysicalWidth(monitor) } }
/// Get specified monitor physical height in millimetres
pub fn get_monitor_physical_height(_: &Raylib, monitor: i32) -> i32 { unsafe { ffi::GetMonitorPhysicalHeight(monitor) } }
/// Get specified monitor refresh rate
pub fn get_monitor_refresh_rate(_: &Raylib, monitor: i32) -> i32 { unsafe { ffi::GetMonitorRefreshRate(monitor) } }
/// Get window position XY on monitor
pub fn get_window_position(_: &Raylib) -> Vector2 { unsafe { ffi::GetWindowPosition() } }
/// Get window scale DPI factor
pub fn get_window_scale_dpi(_: &Raylib) -> Vector2 { unsafe { ffi::GetWindowScaleDPI() } }
/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn get_monitor_name(_: &Raylib, monitor: i32) -> &CStr {
    unsafe { CStr::from_ptr(ffi::GetMonitorName(monitor)) }
}
/// Set clipboard text content   
pub fn set_clipboard_text(_: &Raylib, text: &CStr) { unsafe { ffi::SetClipboardText(text.as_ptr()) } }
/// Get clipboard text content   
pub fn get_clipboard_text(_: &Raylib) -> &'static CStr { 
    unsafe { CStr::from_ptr(ffi::GetClipboardText()) }
}
/// Enable waiting for events on EndDrawing(), no automatic event polling   
pub fn enable_event_waiting(_: &Raylib) { unsafe { ffi::EnableEventWaiting() } }
/// Disable waiting for events on EndDrawing(), automatic events polling   
pub fn disable_event_waiting(_: &Raylib) { unsafe { ffi::DisableEventWaiting() } }

/// Set the current mouse cursor kind
pub fn set_mouse_cursor(_: &mut Raylib, cursor: MouseCursor) { unsafe { ffi::SetMouseCursor(cursor as i32) } }
