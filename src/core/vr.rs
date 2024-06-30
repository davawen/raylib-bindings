//! VR config functions

pub use crate::ffi::{VrDeviceInfo, VrStereoConfig};
use crate::ffi;

use super::Raylib;

impl VrStereoConfig {
    pub fn load(_: &Raylib, device: VrDeviceInfo) -> VrStereoConfig {
        unsafe { ffi::LoadVrStereoConfig(device) }
    }
}
