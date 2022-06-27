use std::ffi::CStr;

use clib_graphviz::GraphvizSys;
fn main() {
    let v = unsafe {CStr::from_ptr( GraphvizSys::version())}.to_str().unwrap();
    println!("Hello, Graphviz! {}", v);
}
