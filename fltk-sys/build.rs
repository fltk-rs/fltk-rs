#![allow(unused_imports, dead_code, unused_variables)]

extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::fs;



fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let sys_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cfltk/cfl.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_window.h");

    let bindings = bindgen::Builder::default()
    .header("cfltk/cfl.h")
    // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

    bindings
        .write_to_file(sys_dir.join("src").join("fl.rs"))
        .expect("Couldn't write bindings!");

    let bindings = bindgen::Builder::default()
        .header("cfltk/cfl_window.h")
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(sys_dir.join("src").join("window.rs"))
        .expect("Couldn't write bindings!");

    let dst = cmake::Config::new("cfltk")
                 .generator("Ninja")
                 .define("OPTION_BUILD_EXAMPLES","OFF")
                 .build();
    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=cfltk");
    println!("cargo:rustc-link-lib=static=fltk");

    match target_os.unwrap().as_str() {
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=Xinerama");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
            println!("cargo:rustc-link-lib=dylib=Xrender");
            println!("cargo:rustc-link-lib=dylib=Xfixes");
        },
        "macos" => {
            println!("cargo:rustc-link-lib=dylib=c++");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=ApplicationServices");
            println!("cargo:rustc-link-lib=dylib=z");
        },
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=wsock32");
            println!("cargo:rustc-link-lib=dylib=comctl32");
        },
        _ => panic!("OS not supported!")
    }
}
