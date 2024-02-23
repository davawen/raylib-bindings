use std::ffi::CStr;

#[doc(hidden)]
pub const fn validate_cstr(bytes: &[u8]) -> &CStr {
    let s = CStr::from_bytes_with_nul(bytes);
    match s {
        Ok(s) => s,
        Err(_) => panic!("cstr cannot contain null bytes")
    }
}

#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        crate::cstr::validate_cstr(concat!($s, "\0").as_bytes())
    };
}
