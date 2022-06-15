use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn version() -> *const c_char;
//    fn layout(&str dot, &str format, &str engine) -> &str;
}

pub fn ver()->&'static str {
    unsafe {
        let cstr = CStr::from_ptr(version());
        cstr.to_str().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::ver;

    #[test]
    fn test_version() {
        assert_eq!("", ver());
    }
}