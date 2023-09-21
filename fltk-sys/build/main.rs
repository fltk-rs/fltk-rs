#![allow(clippy::uninlined_format_args)]

use std::{env, path::PathBuf};

mod android;
mod bundled;
#[cfg(feature = "fltk-config")]
mod fltk_config;
mod link;
mod pkg_config;
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
    println!("cargo:rerun-if-changed=build/fltk_config.rs");
    println!("cargo:rerun-if-changed=build/pkg_config.rs");

    if cfg!(feature = "fltk-bundled") {
        bundled::get(&target_triple, &out_dir);
    } else if cfg!(feature = "fltk-config") {
        let version = utils::fltk_config_version();
        if version != "1.4" {
            panic!(
                "The fltk-config feature requires FLTK 1.4. The current version is {}",
                version
            );
        }
        #[cfg(feature = "fltk-config")]
        fltk_config::build();
        return;
    } else if cfg!(feature = "pkg-config") {
        pkg_config::build();
        return;
    } else {
        const MSG: &str = r#"Perhaps you would prefer to use a bundled version of fltk. 
            You would need to enable the fltk-bundled feature.
            Or if you have an installation of FLTK 1.4 and a working fltk-config executable, you can use the fltk-config feature.
            Features can be enabled in your Cargo.toml or from the command line using the --features=fltk/fltk-bundled argument to cargo."#;

        if !utils::has_program("cmake") {
            panic!(
                "CMake was not found. It's needed to build fltk and cfltk. \n{}",
                MSG
            );
        }
        source::build(&manifest_dir, &target_triple, &out_dir);
    }

    link::link(&target_os, &target_triple, &out_dir);
}
