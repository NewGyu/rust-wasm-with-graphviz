use std::ffi::CStr;

use graphviz_sys::GraphvizSys;
fn main() {
    let v = unsafe {CStr::from_ptr( GraphvizSys::version())}.to_str().unwrap();
    println!("Hello, Graphviz! {}", v);
}
