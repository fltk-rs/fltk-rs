#![allow(unused_imports, dead_code, unused_variables)]

extern crate cmake;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;


fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cfltk/cfl.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_window.h");
    
    Command::new("git").args(&["submodule", "update", "--init"])
                      .current_dir(manifest_dir)
                      .status().unwrap();

    let dst = cmake::Config::new("cfltk")
                 .define("OPTION_BUILD_EXAMPLES","OFF")
                 .build();
    println!("cargo:rustc-link-search=native={}", dst.join("build").display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=cfltk");
    if cfg!(debug_assertions) && cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-lib=static=fltkd");
    } else {
        println!("cargo:rustc-link-lib=static=fltk");
    }
    
    match target_os.unwrap().as_str() {
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
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=oleaut32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=shell32");
            println!("cargo:rustc-link-lib=dylib=advapi32");
            println!("cargo:rustc-link-lib=dylib=comdlg32");
            println!("cargo:rustc-link-lib=dylib=winspool");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=kernel32");
        },
        _ => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=X11");
            println!("cargo:rustc-link-lib=dylib=Xext");
            println!("cargo:rustc-link-lib=dylib=Xinerama");
            println!("cargo:rustc-link-lib=dylib=Xcursor");
            println!("cargo:rustc-link-lib=dylib=Xrender");
            println!("cargo:rustc-link-lib=dylib=Xfixes");
        }
    }
}
