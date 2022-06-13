use cmake;

fn main() {
    let csrc = cmake::build("cpp");

    println!("cargo:rustc-link-search=native={}", csrc.display());
    println!("cargo:rust-link-lib=static=");
}
