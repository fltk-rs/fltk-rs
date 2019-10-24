#![allow(unused_imports, dead_code, unused_variables)]

extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::fs;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let sys_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cfltk/cfl_wrapper.h");
    println!("cargo:rerun-if-changed=cfltk/cfl.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_window.h");

    let bindings = bindgen::Builder::default()
    .header("cfltk/cfl_wrapper.h")
    // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

    bindings
        .write_to_file(sys_dir.join("src").join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = cmake::Config::new("cfltk")
                 .define("OPTION_BUILD_EXAMPLES","OFF")
                 .build();
    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-lib=dylib=cfltk");
}