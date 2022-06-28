use std::ffi::{CStr, CString};
use std::fmt;

use graphviz_sys::GraphvizSys;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn gvz_version() -> String {
    unsafe { CStr::from_ptr(GraphvizSys::version()) }
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn gvz_last_error() -> String {
    unsafe { CStr::from_ptr(GraphvizSys::lastError()) }
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg_attr(all(target_arch = "wasm32", not(target_os = "wasi")), wasm_bindgen)]
pub fn gvz_layout(dot: String) -> String {
    let cs_dot = CString::new(dot).unwrap();
    unsafe {
        let mut g = GraphvizSys::new(false, 0);
        let layouted = g.layout(
            cs_dot.as_ptr(),
            OutputFormat::default().to_cstring().as_ptr(),
            LayoutEngine::default().to_cstring().as_ptr(),
        );
        g.destruct();
        CStr::from_ptr(layouted).to_str().unwrap().to_string()
    }
}

#[allow(unused)]
enum OutputFormat {
    Dot,
    DotJson,
    Json,
    Svg,
    XdotJson,
}

#[allow(unused)]
enum LayoutEngine {
    Circo,
    Dot,
    Fdp,
    Sfdp,
    Neato,
    Osage,
    Patchwork,
    Twopi,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Svg => write!(f, "svg"),
            OutputFormat::Dot => write!(f, "dot"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::DotJson => write!(f, "dot_json"),
            OutputFormat::XdotJson => write!(f, "xdot_json"),
        }
    }
}
impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Svg
    }
}

impl fmt::Display for LayoutEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutEngine::Dot => write!(f, "dot"),
            LayoutEngine::Circo => write!(f, "circo"),
            LayoutEngine::Fdp => write!(f, "fdp"),
            LayoutEngine::Sfdp => write!(f, "sfdp"),
            LayoutEngine::Neato => write!(f, "neato"),
            LayoutEngine::Osage => write!(f, "osage"),
            LayoutEngine::Patchwork => write!(f, "patchwork"),
            LayoutEngine::Twopi => write!(f, "twopi"),
        }
    }
}
impl Default for LayoutEngine {
    fn default() -> Self {
        LayoutEngine::Dot
    }
}

trait ToCstring<T = Self> 
    where Self: fmt::Display
{
    fn to_cstring(&self) -> CString {
        let stringified = self.to_string();
        CString::new(stringified.as_str()).unwrap()
    }
}

impl ToCstring for OutputFormat{}
impl ToCstring for LayoutEngine {}