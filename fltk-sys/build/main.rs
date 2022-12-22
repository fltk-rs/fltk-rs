#![allow(clippy::needless_borrow)]
use std::{env, path::PathBuf};

mod android;
mod bundled;
mod link;
mod source;
mod utils;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rerun-if-changed=build/android.rs");
    println!("cargo:rerun-if-changed=build/bundled.rs");
    println!("cargo:rerun-if-changed=build/link.rs");
    println!("cargo:rerun-if-changed=build/main.rs");
    println!("cargo:rerun-if-changed=build/source.rs");
    println!("cargo:rerun-if-changed=build/utils.rs");

    if cfg!(feature = "fltk-bundled") {
        bundled::get(&target_os, &target_triple, &out_dir);
    } else if !crate::utils::has_program("git") || !crate::utils::has_program("cmake") {
        println!("cargo:warning=Could not find invokable CMake or Git, building using the fltk-bundled feature flag!");
        println!("cargo:warning=If this is not desirable, please ensure CMake and Git are installed and in your PATH!");
        bundled::get(&target_os, &target_triple, &out_dir);
    } else {
        source::build(&manifest_dir, &target_triple, &out_dir);
    }

    link::link(&target_os, &target_triple, &out_dir);
}
