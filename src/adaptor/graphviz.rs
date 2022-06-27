use std::ffi::{CStr, CString};
use std::fmt;

use graphviz_sys::GraphvizSys;

pub fn gvz_version() -> String {
    unsafe { CStr::from_ptr(GraphvizSys::version()) }
        .to_str()
        .unwrap()
        .to_string()
}

pub fn gvz_lastError() -> String {
    unsafe { CStr::from_ptr(GraphvizSys::lastError()) }
        .to_str()
        .unwrap()
        .to_string()
}

pub fn gvz_layout(dot: String) -> String {
    let cs_dot = CString::new(dot).unwrap();
    unsafe {
        let mut g = GraphvizSys::new(false, 0);
        let layouted = g.layout(
            cs_dot.as_ptr(),
            to_cstring(OutputFormat::default()).as_ptr(),
            to_cstring(LayoutEngine::default()).as_ptr(),
        );
        g.destruct();
        CStr::from_ptr(layouted).to_str().unwrap().to_string()
    }
}

#[allow(non_camel_case_types)]
enum OutputFormat {
    dot,
    dot_json,
    json,
    svg,
    xdot_json,
}

#[allow(non_camel_case_types)]
enum LayoutEngine {
    circo,
    dot,
    fdp,
    sfdp,
    neato,
    osage,
    patchwork,
    twopi,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::svg => write!(f, "svg"),
            _ => write!(f, "unko"),
        }
    }
}
impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::svg
    }
}

impl fmt::Display for LayoutEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutEngine::dot => write!(f, "dot"),
            _ => write!(f, "unko"),
        }
    }
}
impl Default for LayoutEngine {
    fn default() -> Self {
        LayoutEngine::dot
    }
}

fn to_cstring<T>(val: T) -> CString
where
    T: fmt::Display,
{
    let stringified = val.to_string();
    CString::new(stringified.as_str()).unwrap()
}
