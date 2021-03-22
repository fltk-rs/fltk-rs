use std::{env, path::PathBuf};

mod builder;
use builder::*;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(feature = "fltk-bundled") {
        bundled::get(target_os.clone(), out_dir.clone());
    } else {
        source::build(manifest_dir.clone(), target_triple.clone(), out_dir.clone());
    }

    link::link(target_os, out_dir);
}
