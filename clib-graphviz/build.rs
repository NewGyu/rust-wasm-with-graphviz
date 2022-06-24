use std::{env, path::PathBuf};
use cmake::Config;

fn main() {
    let mut cm = Config::new("cpp");
    cm.define("CMAKE_BUILD_TYPE", "MinSizeRel");
    let target = env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        let em_cmake = PathBuf::from(env!("EMSDK"))
        .join("upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake");
        cm.define("CMAKE_TOOLCHAIN_FILE", em_cmake);
        cm.target("wasm32-unknown-emscripten");
    }
    let dst = &cm.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    for lib in LIBS {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

const LIBS:[&str;25] = [
    "graphvizlib",
    "cdt",
    "circogen",
    "common",
    "dotgen",
    "fdpgen",
    "ingraphs",
    "label",
    "neatogen",
    "ortho",
    "osage",
    "pack",
    "patchwork",
    "pathplan",
    "rbtree",
    "sparse",
    "sfdpgen",
    "twopigen",
    "vpsc",
    "xdot",
    "cgraph",
    "gvc",
    "gvplugin_core",
    "gvplugin_dot_layout",
    "gvplugin_neato_layout"
];
