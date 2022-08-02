#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
/* automatically generated by rust-bindgen 0.60.1 */

extern "C" {
    pub fn graphviz_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn graphviz_lastError() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn graphviz_setYInvert(yInvert: bool);
}
extern "C" {
    pub fn graphviz_setNop(nop: ::std::os::raw::c_int);
}
extern "C" {
    pub fn graphviz_layout(
        src: *const ::std::os::raw::c_char,
        format: *const ::std::os::raw::c_char,
        engine: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}