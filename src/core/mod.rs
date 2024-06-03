pub mod window;
pub mod cursor;
pub mod draw;
pub mod shader;
pub mod vr;
pub mod automation;
pub mod input;
pub mod other;
mod default_font;

use std::{ptr::NonNull, ffi::c_void, mem::ManuallyDrop};

use crate::{ffi, text::bitmap::BitmapFontAtlas};

/// A handle to raylib's internal state.
/// 
/// This struct is passed when functions modify raylib's state, or to allocate objects that will need to modify raylib state.
///
/// Please note that raylib is inherently single threaded, thus this object cannot be sent between threads safely
/// (it doesn't implement [`Send`]).
/// Raylib functions should NEVER be called in a multi threaded environment.
pub struct Raylib {
    /// Keeps track of wether an automation event list is currently set
    /// See `core::automation`
    #[allow(unused)]
    automation_event_set: bool,
    /// Keeps track of wether an automation event is currently recording
    /// See `core::automation`
    #[allow(unused)]
    automation_event_recording: bool,
    /// The user called [`Raylib::quit`].
    quit_requested: bool,
    /// Keeps hold of the default raylib font.
    /// NOTE: The default font needs to be dropped before raylib itself closes, which is why the `ManuallyDrop` is needed.
    default_font: ManuallyDrop<Option<BitmapFontAtlas>>,
    // disallows initialization from outside and makes raylib !Send and !Sync
    _private: std::marker::PhantomData<*const c_void>
}

impl Drop for Raylib {
    fn drop(&mut self) {
        // Drop default font (and free texture) before raylib exits
        unsafe { ManuallyDrop::drop(&mut self.default_font) }
        unsafe { ffi::CloseWindow() }
    }
}

/// Represents memory allocated using the internal raylib allocator.
/// 
/// Its drop implementation calls `ffi::MemFree` to free the memory safely.
pub struct RaylibAlloc<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Drop for RaylibAlloc<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.0.as_ptr()); // call object destructor (objects allocated by raylib should all be Copy but we never know)
            ffi::MemFree(self.0.as_ptr() as *mut c_void)  // free memory
        }
    }
}

impl<T: ?Sized> std::ops::Deref for RaylibAlloc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> std::ops::DerefMut for RaylibAlloc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
