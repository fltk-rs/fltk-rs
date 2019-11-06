#![allow(unused_imports, dead_code, unused_variables)]

extern crate cmake;

use std::{env, path::{Path, PathBuf}, process::Command};

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cfltk/cfl.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_widget.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_group.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_window.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_button.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_box.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_menu.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_dialog.h");
    println!("cargo:rerun-if-changed=cfltk/cfl_valuator.h");
    println!("cargo:rerun-if-changed=cfltk/global.h");
    println!("cargo:rerun-if-changed=cfltk/CMakeLists.txt");
    
    Command::new("git").args(&["submodule", "update", "--init"])
                      .current_dir(manifest_dir.clone())
                      .status().unwrap();

    Command::new("git").args(&["checkout", "master"])
                    .current_dir(manifest_dir.join("cfltk").join("fltk"))
                    .status().unwrap();

    let dst = cmake::Config::new("cfltk")
                 .generator("Ninja")
                 .define("OPTION_BUILD_EXAMPLES","OFF")
                 .define("OPTION_USE_SYSTEM_ZLIB","OFF")
                 .define("OPTION_USE_SYSTEM_LIBPNG","OFF")
                 .define("OPTION_USE_SYSTEM_LIBJPEG","OFF")
                 .build();
    println!("cargo:rustc-link-search=native={}", dst.join("build").display());

    // Change static to dylib to link dynamically, also change CMakeLists STATIC to SHARED

    println!("cargo:rustc-link-lib=static=cfltk");

    // Comment out all following code to link dynamically


    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
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
            if cfg!(target_env = "gnu") {
                println!("cargo:rustc-link-lib=dylib=stdc++");
            }
            println!("cargo:rustc-link-lib=dylib=wsock32");
            println!("cargo:rustc-link-lib=dylib=comctl32");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=oleaut32");
            println!("cargo:rustc-link-lib=dylib=ole32");
            println!("cargo:rustc-link-lib=dylib=uuid");
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
            println!("cargo:rustc-link-lib=dylib=Xft");
            println!("cargo:rustc-link-lib=dylib=fontconfig");
        }
    }
}

