pub use crate::ffi::VrDeviceInfo;
use crate::{ffi, core::Raylib};

pub struct VrStereoConfig(ffi::VrStereoConfig);

impl std::ops::Deref for VrStereoConfig {
    type Target = ffi::VrStereoConfig;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for VrStereoConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for VrStereoConfig {
    fn drop(&mut self) {
        unsafe { ffi::UnloadVrStereoConfig(self.0) }
    }
}

/// # Vr config functions (module: `rcore`)
///
/// ---
pub trait RaylibVrFunctions {
    fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
        let conf = unsafe { ffi::LoadVrStereoConfig(device) };
        VrStereoConfig(conf)
    }
}

impl RaylibVrFunctions for Raylib {}
