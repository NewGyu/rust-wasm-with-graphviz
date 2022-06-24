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
    println!("cargo:rustc-link-lib=static=graphvizlib");
}
