use std::{ffi::{CStr, CString}, ptr::NonNull, sync::Mutex};

use crate::{core::Raylib, ffi::{self, TraceLogLevel}};

use super::RaylibAlloc;

/// Sets the maximum FPS
pub fn set_target_fps(_: &Raylib, fps: i32) {
    unsafe { ffi::SetTargetFPS(fps) }
}

/// Gets the time elapsed in seconds since the last frame was drawn (delta time)
pub fn get_frame_time(_: &Raylib) -> f32 {
    unsafe { ffi::GetFrameTime() }
}

/// Gets the time elapsed in seconds since `Raylib::init_window` was called 
/// If you need greater precision, use `Raylib::get_time_f64`
pub fn get_time(rl: &Raylib) -> f32 {
    get_time_f64(rl) as f32
}

/// Gets the time elapsed in seconds since `Raylib::init_window` was called 
pub fn get_time_f64(_: &Raylib) -> f64 {
    unsafe { ffi::GetTime() }
}

/// Halt program execution for the given number of seconds
/// If you need more precision, use `Raylib::wait_time_f64`
pub fn wait_time(rl: &Raylib, seconds: f32) {
    wait_time_f64(rl, seconds as f64)
}

/// Halt program execution for the given number of seconds
pub fn wait_time_f64(_: &Raylib, seconds: f64) {
    unsafe { ffi::WaitTime(seconds) }
}

// TODO:
// // === Custom frame control functions ===
// impl Raylib {
//     /// Swap back buffer with front buffer (show what was drawn to the screen)
//     /// NOTE: Frame control functions are intented for advenced users that want full control over the frame processing
//     /// By default EndDrawing() does this job: draws everything + SwapScreenBuffer() + manage frame timing + PollInputEvents()
//     /// To avoid that behaviour and control frame processes manually, enable in config.h: SUPPORT_CUSTOM_FRAME_CONTROL
//     pub fn swap_scren_buffer(&self) {
//         unsafe { ffi::SwapScreenBuffer() }
//     }
//
//     pub fn poll_input_events(&self) {
//         unsafe { ffi::PollInputEvents() }
//     }
// }

pub struct RandomSequence<'a>(&'a mut [i32]);

/// # Random value generation functions (module: `rcore`)
///
/// ---
impl Raylib {
    /// Sets the seed for the random number generator
    /// # Examples
    /// Set random seed with current time:
    /// ```
    /// use std::time::SystemTime;
    /// use raylib::prelude::*;
    /// let rl = &mut init_window(800, 800, "Random value", 60);
    /// rl.set_random_seed(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32)
    /// ```
    pub fn set_random_seed(&self, seed: u32) {
        unsafe { ffi::SetRandomSeed(seed) }
    }

    /// Gets a random value between min and max (both included)
    pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
        unsafe { ffi::GetRandomValue(min, max) }
    }

    /// Creates a sequence of random values, no values repeated
    pub fn random_sequence(&self, count: u32, min: i32, max: i32) -> RandomSequence {
        let values = unsafe {
            let ptr = ffi::LoadRandomSequence(count, min, max);
            std::slice::from_raw_parts_mut(ptr, count as usize)
        };
        RandomSequence(values)
    }
}

impl RandomSequence<'_> {
    pub fn slice(&self) -> &[i32] {
        self.0
    }

    pub fn mut_slice(&mut self) -> &mut [i32] {
        self.0
    }
}

impl Drop for RandomSequence<'_> {
    fn drop(&mut self) {
        unsafe { ffi::UnloadRandomSequence(self.0.as_mut_ptr()) }
    }
}

// # Misc functions (module: `rcore`)

/// Takes a screenshot of the current screen (filename extension defines format)
/// # Panics
/// Panics if the given string contains nulls
pub fn take_screenshot(rl: &Raylib, filename: &str) {
    take_screenshot_cstr(rl, CString::new(filename).expect("a filename without nuls").as_c_str());
}

/// Takes a screenshot of the current screen (filename extension defines format)
pub fn take_screenshot_cstr(_: &Raylib, filename: &CStr) {
    unsafe { ffi::TakeScreenshot(filename.as_ptr()) }
}

/// Opens a URL with the default system browser (if available)
/// # Panics
/// Panics if the given string contains nulls
pub fn open_url(rl: &Raylib, url: &str) {
    open_url_cstr(rl, CString::new(url).expect("a url without nuls").as_c_str())
}

pub fn open_url_cstr(_: &Raylib, url: &CStr) {
    unsafe { ffi::OpenURL(url.as_ptr()) }
}

/// Show a log message of the given level
/// # Panics
/// Panics if the given string contains nulls
pub fn trace_log(level: TraceLogLevel, text: &str) {
    trace_log_cstr(level, CString::new(text).expect("a message without nuls").as_c_str())
}

/// Show a log message of the given level
pub fn trace_log_cstr(level: TraceLogLevel, text: &CStr) {
    unsafe { ffi::TraceLog(level as i32, text.as_ptr()) }
}

/// Sets the minimum log level (log message under this level won't be printed)
pub fn set_trace_log_level(level: TraceLogLevel) {
    unsafe { ffi::SetTraceLogLevel(level as i32) }
}

static CUSTOM_CALLBACK: Mutex<Option<fn(TraceLogLevel, &str, va_list::VaList)>> = Mutex::new(None);

extern "C" fn internal_trace_log_callback(level: i32, msg: *const i8, va: va_list::VaList) {
    let callback = CUSTOM_CALLBACK.lock().unwrap();
    let callback = callback.unwrap();

    let msg = unsafe { CStr::from_ptr(msg) }.to_str().unwrap();

    callback(level.try_into().unwrap(), msg, va);
}

pub fn set_trace_log_callback(callback: fn(TraceLogLevel, &str, va_list::VaList)) {
    let mut lock = CUSTOM_CALLBACK.lock().unwrap();
    *lock = Some(callback);
    drop(lock);
    unsafe {
        ffi::SetTraceLogCallback(internal_trace_log_callback)
    }
}

/// Compress data using the DEFLATE algorithm
/// # Panics
/// This function panics if raylib was not compiled with support for the compression api
pub fn compress_data(data: &[u8]) -> RaylibAlloc<[u8]> {
    let mut comp_data_size: i32 = 0;
    let compressed = unsafe { ffi::CompressData(data.as_ptr(), data.len() as i32, &mut comp_data_size as *mut i32) };
    let slice = std::ptr::slice_from_raw_parts_mut(compressed, comp_data_size as usize);

    RaylibAlloc(NonNull::new(slice).expect("expected a valid pointer returned by CompressData"))
}

/// Decompress data compressed using the DEFLATE algorithm
/// # Panics
/// This function panics if raylib was not compiled with support for the compression api
pub fn decompress_data(compressed: &[u8]) -> RaylibAlloc<[u8]> {
    let mut data_size: i32 = 0;
    let data = unsafe { ffi::DecompressData(compressed.as_ptr(), compressed.len() as i32, &mut data_size as *mut i32) };

    let slice = std::ptr::slice_from_raw_parts_mut(data, data_size as usize);
    RaylibAlloc(NonNull::new(slice).unwrap())
}

/// Encode data into a base 64 string
pub fn encode_data_base64(data: &[u8]) -> RaylibAlloc<[u8]> {
    let mut comp_data_size: i32 = 0;
    let compressed = unsafe { ffi::EncodeDataBase64(data.as_ptr(), data.len() as i32, &mut comp_data_size as *mut i32) };
    let slice = std::ptr::slice_from_raw_parts_mut(compressed as *mut u8, comp_data_size as usize);

    RaylibAlloc(NonNull::new(slice).unwrap())
}

/// Decode data from a base 64 string
pub fn decode_data_base64(compressed: &[u8]) -> RaylibAlloc<[u8]> {
    let mut data_size: i32 = 0;
    let data = unsafe { ffi::DecodeDataBase64(compressed.as_ptr(), &mut data_size as *mut i32) };

    let slice = std::ptr::slice_from_raw_parts_mut(data, data_size as usize);
    RaylibAlloc(NonNull::new(slice).unwrap())
}
