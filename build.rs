extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=mcp2210");
    println!("cargo:rustc-link-lib=udev");

    gcc::Build::new()
        // .cpp(true)
        .flag("-ludev")
        .flag("-w")
        .file("mcp2210-cpp/mcp2210.cpp")
        .file("mcp2210-cpp/hid.c")
        .compile("libmcp2210.a");


    let bindings = bindgen::Builder::default()
        .clang_args(&["-Imcp2210-cpp"]);

    let bindings = bindings
        .header("wrapper.hpp")
        .derive_default(true)
        .derive_debug(true)
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
