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
        bundled::get(target_os.clone(), out_dir.clone());
    } else {
        source::build(manifest_dir, target_triple, out_dir.clone());
    }

    link::link(target_os, out_dir);
}
