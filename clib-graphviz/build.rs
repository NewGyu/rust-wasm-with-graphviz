use cmake::Config;

fn main() {
    let dst = Config::new("cpp")
        .define("CMAKE_BUILD_TYPE", "MinSizeRel")
        .define("CMAKE_TOOLCHAIN_FILE", "../emsdk/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rust-link-lib=static=");
}
