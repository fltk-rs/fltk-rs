use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn check_cfltk_empty() {
    if PathBuf::from("cfltk")
        .read_dir()
        .map(|mut i| i.next().is_none())
        .unwrap_or(false)
    {
        panic!("cfltk submodule not initialized! Run: git submodule update --init --recursive");
    }
}

pub fn has_program(prog: &str) -> bool {
    match Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => {
            println!("cargo:warning=Could not find invokable {}!", prog);
            false
        }
    }
}

pub fn proc_output(args: &[&str]) -> String {
    let out = match Command::new(args[0]).args(&args[1..]).output() {
        Ok(out) => out.stdout,
        _ => vec![],
    };
    String::from_utf8_lossy(&out).to_string().trim().to_string()
}

pub fn use_static_msvcrt() -> bool {
    cfg!(target_feature = "crt-static") || cfg!(feature = "static-msvcrt")
}
pub fn get_macos_deployment_target() -> i32 {
    let env = env::var("MACOSX_DEPLOYMENT_TARGET");
    if let Ok(env) = env {
        let val: i32 = env
            .trim()
            .split('.')
            .next()
            .expect("Couldn't get macos version!")
            .parse()
            .expect("Counldn't get macos version!");
        val
    } else {
        11
    }
}

pub fn link_macos_framework_if_exists(frameworks: &[(&str, i32)]) {
    let target = get_macos_deployment_target();
    let sdk = if let Ok(p) = env::var("SDKROOT") {
        p
    } else {
        Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim_end().to_owned())
            .unwrap_or_default()
    };

    for f in frameworks {
        let framework = f.0;
        if target >= f.1 {
            let candidates = [
                &format!("/System/Library/Frameworks/{framework}.framework"),
                &format!("/System/Library/PrivateFrameworks/{framework}.framework"),
                &format!("{sdk}/System/Library/Frameworks/{framework}.framework"),
                &format!("{sdk}/System/Library/PrivateFrameworks/{framework}.framework"),
            ];

            let found_path = candidates.iter().find(|p| Path::new(p).exists());

            if let Some(path) = found_path {
                println!("cargo:rustc-link-lib=framework={framework}");
                if path.starts_with("/System/Library/PrivateFrameworks") {
                    println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
                }
            } else {
                println!("cargo:warning={framework} not found ─ building without it");
            }
        }
    }
}
