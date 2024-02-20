pub mod window;
pub mod cursor;
pub mod draw;
pub mod shader;
pub mod vr;
pub mod automation;
pub mod input;
pub mod other;

use std::{ptr::NonNull, ffi::c_void};

use crate::ffi;

pub struct Raylib {
    /// Keeps track of wether an automation event list is currently set
    /// See `core::automation`
    #[allow(unused)]
    automation_event_set: bool,
    /// Keeps track of wether an automation event is currently recording
    /// See `core::automation`
    #[allow(unused)]
    automation_event_recording: bool,
    // disallows initialization from outside
    _private: ()
}

impl Drop for Raylib {
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}

/// Represents memory alloced using the internal raylib allocator.
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
