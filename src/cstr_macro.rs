/// Utility macro to create a static [`CStr`](`std::ffi::CStr`)
/// # Examples
/// ```
/// # use std::ffi::CStr;
/// # use raylib::cstr;
/// const ID: &'static CStr = cstr!("Test string");
/// ```
/// ```compile_fail
/// # use std::ffi::CStr;
/// # use raylib::cstr;
/// const INVALID: &'static CStr = cstr!("it contains\0null terminators");
/// ```
/// ```compile_fail
/// # use raylib::cstr;
/// let str = "it only accepts string literals".to_string();
/// let str = cstr!(str);
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:literal) => {
        {
            let s = CStr::from_bytes_with_nul(concat!($s, "\0").as_bytes());
            match s {
                Ok(s) => s,
                Err(_) => panic!("cstr cannot contain null bytes")
            }
        }
    };
}
