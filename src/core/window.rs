use std::ffi::CStr;
use crate::ffi::{self, Image, Vector2};

use bitflags::bitflags;

use super::Raylib;

bitflags! {
    pub struct ConfigFlags: u32 {
        /// Set to try enabling V-Sync on GPU
        const FLAG_VSYNC_HINT = ffi::ConfigFlags::FLAG_VSYNC_HINT as u32;
        /// Set to run program in fullscreen
        const FLAG_FULLSCREEN_MODE= ffi::ConfigFlags::FLAG_FULLSCREEN_MODE as u32;
        /// Set to allow resizable window
        const FLAG_WINDOW_RESIZABLE = ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32;
        /// Set to disable window decoration (frame and buttons)
        const FLAG_WINDOW_UNDECORATED = ffi::ConfigFlags::FLAG_WINDOW_UNDECORATED as u32;
        /// Set to hide window
        const FLAG_WINDOW_HIDDEN= ffi::ConfigFlags::FLAG_WINDOW_HIDDEN as u32;
        /// Set to minimize window (iconify)
        const FLAG_WINDOW_MINIMIZED = ffi::ConfigFlags::FLAG_WINDOW_MINIMIZED as u32;
        /// Set to maximize window (expanded to monitor)
        const FLAG_WINDOW_MAXIMIZED = ffi::ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32;
        /// Set to window non focused
        const FLAG_WINDOW_UNFOCUSED = ffi::ConfigFlags::FLAG_WINDOW_UNFOCUSED as u32;
        /// Set to window always on top
        const FLAG_WINDOW_TOPMOST = ffi::ConfigFlags::FLAG_WINDOW_TOPMOST as u32;
        /// Set to allow windows running while minimized
        const FLAG_WINDOW_ALWAYS_RUN= ffi::ConfigFlags::FLAG_WINDOW_ALWAYS_RUN as u32;
        /// Set to allow transparent framebuffer
        const FLAG_WINDOW_TRANSPARENT = ffi::ConfigFlags::FLAG_WINDOW_TRANSPARENT as u32;
        /// Set to support HighDPI
        const FLAG_WINDOW_HIGHDPI = ffi::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32;
        /// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
        const FLAG_WINDOW_MOUSE_PASSTHROUGH = ffi::ConfigFlags::FLAG_WINDOW_MOUSE_PASSTHROUGH as u32;
        /// Set to run program in borderless windowed mode
        const FLAG_BORDERLESS_WINDOWED_MODE = ffi::ConfigFlags::FLAG_BORDERLESS_WINDOWED_MODE as u32;
        /// Set to try enabling MSAA 4X
        const FLAG_MSAA_4X_HINT = ffi::ConfigFlags::FLAG_MSAA_4X_HINT as u32;
        /// Set to try enabling interlaced video format (for V3D)
        const FLAG_INTERLACED_HINT= ffi::ConfigFlags::FLAG_INTERLACED_HINT as u32;
    }
}

impl Raylib {
    pub fn init_window(width: i32, height: i32, title: &CStr, target_fps: i32) -> Self {
        unsafe { ffi::InitWindow(width, height, title.as_ptr()) }
        unsafe { ffi::SetTargetFPS(target_fps) }

        Self { 
            automation_event_set: false,
            automation_event_recording: false,
            _private: () 
        }
    }

    /// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
    pub fn window_should_close(&self) -> bool { unsafe { ffi::WindowShouldClose() } }
    /// Check if window has been initialized successfully
    pub fn is_window_ready(&self) -> bool { unsafe { ffi::IsWindowReady() } }
    /// Check if window is currently fullscreen
    pub fn is_window_fullscreen(&self) -> bool { unsafe { ffi::IsWindowFullscreen() } }
    /// Check if window is currently hidden (only PLATFORM_DESKTOP)
    pub fn is_window_hidden(&self) -> bool { unsafe { ffi::IsWindowHidden() } }
    /// Check if window is currently minimized (only PLATFORM_DESKTOP)
    pub fn is_window_minimized(&self) -> bool { unsafe { ffi::IsWindowMinimized() } }
    /// Check if window is currently maximized (only PLATFORM_DESKTOP)
    pub fn is_window_maximized(&self) -> bool { unsafe { ffi::IsWindowMaximized() } }
    /// Check if window is currently focused (only PLATFORM_DESKTOP)
    pub fn is_window_focused(&self) -> bool { unsafe { ffi::IsWindowFocused() } }
    /// Check if window has been resized last frame
    pub fn is_window_resized(&self) -> bool { unsafe { ffi::IsWindowResized() } }
    /// Check if one specific window flag is enabled
    pub fn is_window_state(&self, flag: ffi::ConfigFlags) -> bool { unsafe { ffi::IsWindowState(flag as u32) } }                      
    /// Set window configuration state using flags (only PLATFORM_DESKTOP)
    pub fn set_window_state(&self, flags: ConfigFlags) { unsafe { ffi::SetWindowState(flags.bits()) } }
    /// Clear the window configuration state flags
    pub fn clear_window_state(&self, flags: ConfigFlags) { unsafe { ffi::ClearWindowState(flags.bits()) } }
    /// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
    pub fn toggle_fullscreen(&self) { unsafe { ffi::ToggleFullscreen() } }
    /// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
    pub fn toggle_borderless_windowed(&self) { unsafe { ffi::ToggleBorderlessWindowed() } }
    /// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
    pub fn maximize_window(&self) { unsafe { ffi::MaximizeWindow() } }
    /// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
    pub fn minimize_window(&self) { unsafe { ffi::MinimizeWindow() } }
    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
    pub fn restore_window(&self) { unsafe { ffi::RestoreWindow() } }
    /// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
    pub fn set_window_icon(&self, image: Image) { unsafe { ffi::SetWindowIcon(image) } }
    /// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
    pub fn set_window_icons(&self, images: &[Image]) {
        unsafe { ffi::SetWindowIcons(images.as_ptr() as *mut Image, images.len() as i32) }
    }
    /// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
    pub fn set_window_title(&self, title: &CStr) { unsafe { ffi::SetWindowTitle(title.as_ptr()) } }
    /// Set window position on screen (only PLATFORM_DESKTOP)
    pub fn set_window_position(&self, x: i32, y: i32) { unsafe { ffi::SetWindowPosition(x, y) } }
    /// TODO: Safer handling for monitor indices
    /// Set monitor for the current window
    pub fn set_window_monitor(&self, monitor: i32) { unsafe { ffi::SetWindowMonitor(monitor) } }
    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
    pub fn set_window_min_size(&self, width: i32, height: i32) { unsafe { ffi::SetWindowMinSize(width, height) } }
    /// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
    pub fn set_window_max_size(&self, width: i32, height: i32) { unsafe { ffi::SetWindowMaxSize(width, height) } }
    /// Set window dimensions
    pub fn set_window_size(&self, width: i32, height: i32) { unsafe { ffi::SetWindowSize(width, height) } }
    /// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
    pub fn set_window_opacity(&self, opacity: f32) { unsafe { ffi::SetWindowOpacity(opacity) } }


    /// Set window focused (only PLATFORM_DESKTOP)
    pub fn set_window_focused(&self) { unsafe { ffi::SetWindowFocused() } }

    // TODO:
    // /// Get native window handle
    // pub fn GetWindowHandle(void) -> void * ;

    /// Gets the current screen (window) width
    /// If you need to consider HiDPI, use `Raylib::get_render_width`
    pub fn get_screen_width(&self) -> f32 { unsafe { ffi::GetScreenWidth() as f32 } }
    /// Gets the current screen (window) height
    /// If you need to consider HiDPI, use `Raylib::get_render_width`
    pub fn get_screen_height(&self) -> f32 { unsafe { ffi::GetScreenHeight() as f32 } }
    /// Gets the current screen (window) size as a vector
    /// If you need to consider HiDPI, use `Raylib::get_render_size`
    pub fn get_screen_size(&self) -> Vector2 { Vector2::new(self.get_screen_width(), self.get_screen_height()) }
    /// Gets the current render width (considering HiDPI)
    pub fn get_render_width(&self) -> f32 { unsafe { ffi::GetRenderWidth() as f32 } }
    /// Gets the current render height (considering HiDPI)
    pub fn get_render_height(&self) -> f32 { unsafe { ffi::GetRenderHeight() as f32 } }
    /// Gets the current render size as vector (considering HiDPI)
    pub fn get_render_size(&self) -> Vector2 { Vector2::new(self.get_render_width(), self.get_render_height()) }

    /// Get number of connected monitors
    pub fn get_monitor_count(&self) -> i32 { unsafe { ffi::GetMonitorCount() } }
    /// Get current connected monitor
    pub fn get_current_monitor(&self) -> i32 { unsafe { ffi::GetCurrentMonitor() } }
    /// Get specified monitor position
    pub fn get_monitor_position(&self, monitor: i32) -> Vector2 { unsafe { ffi::GetMonitorPosition(monitor) } }
    /// Get specified monitor width (current video mode used by monitor)
    pub fn get_monitor_width(&self, monitor: i32) -> i32 { unsafe { ffi::GetMonitorWidth(monitor) } }
    /// Get specified monitor height (current video mode used by monitor)
    pub fn get_monitor_height(&self, monitor: i32) -> i32 { unsafe { ffi::GetMonitorHeight(monitor) } }
    /// Get specified monitor physical width in millimetres
    pub fn get_monitor_physical_width(&self, monitor: i32) -> i32 { unsafe { ffi::GetMonitorPhysicalWidth(monitor) } }
    /// Get specified monitor physical height in millimetres
    pub fn get_monitor_physical_height(&self, monitor: i32) -> i32 { unsafe { ffi::GetMonitorPhysicalHeight(monitor) } }
    /// Get specified monitor refresh rate
    pub fn get_monitor_refresh_rate(&self, monitor: i32) -> i32 { unsafe { ffi::GetMonitorRefreshRate(monitor) } }
    /// Get window position XY on monitor
    pub fn get_window_position(&self) -> Vector2 { unsafe { ffi::GetWindowPosition() } }
    /// Get window scale DPI factor
    pub fn get_window_scale_dpi(&self) -> Vector2 { unsafe { ffi::GetWindowScaleDPI() } }
    /// Get the human-readable, UTF-8 encoded name of the specified monitor
    pub fn get_monitor_name(&self, monitor: i32) -> &CStr {
        unsafe { CStr::from_ptr(ffi::GetMonitorName(monitor)) }
    }
    /// Set clipboard text content   
    pub fn set_clipboard_text(&self, text: &CStr) { unsafe { ffi::SetClipboardText(text.as_ptr()) } }
    /// Get clipboard text content   
    pub fn get_clipboard_text(&self) -> &'static CStr { 
        unsafe { CStr::from_ptr(ffi::GetClipboardText()) }
    }
    /// Enable waiting for events on EndDrawing(), no automatic event polling   
    pub fn enable_event_waiting(&self) { unsafe { ffi::EnableEventWaiting() } }
    /// Disable waiting for events on EndDrawing(), automatic events polling   
    pub fn disable_event_waiting(&self) { unsafe { ffi::DisableEventWaiting() } }
}

