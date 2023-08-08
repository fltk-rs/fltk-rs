#![allow(clippy::uninlined_format_args)]

use std::{env, path::PathBuf};

mod android;
mod bundled;
mod config;
mod link;
mod source;
mod utils;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // hack for recent build failure with cross-rs, as of 2023-01-12
    if env::var("CROSS_SYSROOT").is_ok() {
        env::remove_var("CROSS_SYSROOT");
    }

    println!("cargo:rerun-if-changed=build/android.rs");
    println!("cargo:rerun-if-changed=build/bundled.rs");
    println!("cargo:rerun-if-changed=build/link.rs");
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/source.rs");
    println!("cargo:rerun-if-changed=build/utils.rs");
    println!("cargo:rerun-if-changed=build/config.rs");

    let mut used_fltk_config = false;
    if cfg!(feature = "fltk-bundled") {
        bundled::get(&target_os, &target_triple, &out_dir);
    } else if cfg!(feature = "fltk-config") {
        used_fltk_config = config::build();
    } else if !crate::utils::has_program("git") || !crate::utils::has_program("cmake") {
        if utils::fltk_config_version() == "1.4" {
            used_fltk_config = config::build();
        } else {
            println!("cargo:warning=Could not find invokable CMake or Git, building using the fltk-bundled feature flag!");
            println!("cargo:warning=If this is not desirable, please ensure CMake and Git are installed and in your PATH!");
            bundled::get(&target_os, &target_triple, &out_dir);
        }
    } else {
        source::build(&manifest_dir, &target_triple, &out_dir);
    }

    if !used_fltk_config {
        link::link(&target_os, &target_triple, &out_dir);
    }
}
