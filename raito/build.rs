use std::path::PathBuf;

fn main() {
    let pwd = PathBuf::from(".")
        .canonicalize()
        .expect("cannot canonicalize path");

    println!("cargo:rustc-link-search={}", pwd.to_str().unwrap());
    println!("cargo:rustc-link-lib=sdl2");
    println!("cargo:rerun-if-changed=wrapper.h");
}
