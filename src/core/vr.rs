use crate::ffi::{self, VrDeviceInfo, VrStereoConfig};
use super::Raylib;

/// # Vr config functions (module: [rcore])
///
/// ---
impl Raylib {
    pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
        unsafe { ffi::LoadVrStereoConfig(device) }
    }

    pub fn unload_vr_stereo_config(config: VrStereoConfig) {
        unsafe { ffi::UnloadVrStereoConfig(config) }
    }
}

