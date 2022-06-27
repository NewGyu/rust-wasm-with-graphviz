use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    let mut cm = Config::new("cpp");
    cm.define("CMAKE_BUILD_TYPE", "MinSizeRel");
    let target = env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        let em_cmake = PathBuf::from(env!("EMSDK"))
            .join("upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake");
        cm.define("CMAKE_TOOLCHAIN_FILE", em_cmake);
        cm.target("wasm32-unknown-emscripten");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    let dst = &cm.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    for lib in LIBS {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    /*
    let bindings = bindgen::Builder::default()
        .header("cpp/graphvizlib/main.hpp")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings!");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    */
}

const LIBS: [&str; 25] = [
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
    "gvplugin_neato_layout",
];
