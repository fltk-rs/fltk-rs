use crate::utils;
use std::env;
use std::path::Path;
use std::process::Command;

const CPP_SRC: &[&str] = &[
    "cfltk/src/cfl_new.cpp",
    "cfltk/src/cfl_lock.cpp",
    "cfltk/src/cfl.cpp",
    "cfltk/src/cfl_window.cpp",
    "cfltk/src/cfl_button.cpp",
    "cfltk/src/cfl_widget.cpp",
    "cfltk/src/cfl_group.cpp",
    "cfltk/src/cfl_text.cpp",
    "cfltk/src/cfl_box.cpp",
    "cfltk/src/cfl_input.cpp",
    "cfltk/src/cfl_menu.cpp",
    "cfltk/src/cfl_dialog.cpp",
    "cfltk/src/cfl_valuator.cpp",
    "cfltk/src/cfl_browser.cpp",
    "cfltk/src/cfl_misc.cpp",
    "cfltk/src/cfl_image.cpp",
    "cfltk/src/cfl_draw.cpp",
    "cfltk/src/cfl_table.cpp",
    "cfltk/src/cfl_tree.cpp",
    "cfltk/src/cfl_surface.cpp",
    "cfltk/src/cfl_font.cpp",
    "cfltk/src/cfl_utils.cpp",
    "cfltk/src/cfl_prefs.cpp",
    "cfltk/src/cfl_printer.cpp",
    "cfltk/src/cfl_term.cpp",
    "cfltk/src/Fl_Simple_Terminal.cxx",
];

pub fn build(target_triple: &str) {
    println!("cargo:rerun-if-changed=cfltk/include");
    println!("cargo:rerun-if-changed=cfltk/src");
    utils::check_cfltk_empty();

    let mut args = vec![];
    let mut use_gl = false;
    let mut use_images = false;
    if cfg!(feature = "enable-glwindow") {
        args.push("--use-gl");
        use_gl = true;
    }
    if !cfg!(feature = "no-images") {
        args.push("--use-images");
        use_images = true;
    }

    let cflags = get_cflags(&args);
    let mut include_paths = vec![];
    let mut defs = vec![];
    for cflag in cflags {
        if let Some(stripped) = cflag.strip_prefix("-I") {
            include_paths.push(stripped.to_string());
        } else if let Some(stripped) = cflag.strip_prefix("-D") {
            defs.push(stripped.to_string());
        }
    }

    let mut lflags = get_lflags(&args);
    if target_triple.contains("windows") {
        lflags.push("-lgdi32".to_string());
    }
    let mut search_dirs = vec![];
    let mut libs = vec![];
    for lflag in lflags {
        if let Some(stripped) = lflag.strip_prefix("-l") {
            libs.push(stripped.to_string());
        } else if let Some(stripped) = lflag.strip_prefix("-L") {
            search_dirs.push(stripped.to_string());
        } else if lflag == "-static" || lflag == "-static-libstdc++" || lflag == "-static-libgcc" {
        } else {
            let lpath = Path::new(&lflag);
            if lpath.exists() {
                if let Some(dir) = lpath.parent() {
                    search_dirs.push(dir.to_string_lossy().to_string());
                }
                let stem = lpath.file_stem().unwrap().to_string_lossy().to_string();
                let stem = if target_triple.contains("msvc") {
                    stem
                } else {
                    stem.strip_prefix("lib").unwrap_or(&stem).to_string()
                };
                libs.push(stem);
            }
        }
    }

    let mut b = cc::Build::new();
    b.cpp(true);
    b.files(CPP_SRC);
    if cfg!(target_os = "macos") {
        b.file("cfltk/src/cfl_nswindow.m");
    }
    if !(target_triple.contains("windows") || target_triple.contains("darwin")) {
        b.file("cfltk/src/cfl_platform.cpp");
    }
    if use_gl {
        b.define("CFLTK_USE_GL", None);
        b.file("cfltk/src/glad.c");
    }
    if use_images {
        b.define("CFLTK_USE_IMAGES", None);
    }
    b.includes(&include_paths);
    b.include("cfltk/include");
    for def in defs {
        b.define(&def, None);
    }
    b.flag_if_supported("-std=c++17");
    b.warnings(false);
    b.flag_if_supported("-w");
    b.compile("cfltk");

    for dir in search_dirs {
        println!("cargo:rustc-link-search=native={}", dir);
    }
    for lib in libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}

fn call_fltk_config(args: &str) -> Option<String> {
    if let Ok(out) = Command::new("fltk-config")
        .args(args.split_ascii_whitespace())
        .output()
    {
        if out.status.success() {
            return Some(String::from_utf8_lossy(&out.stdout).into_owned());
        }
    }
    if let Ok(out) = Command::new("sh")
        .args(["-lc", &format!("fltk-config {}", args)])
        .output()
    {
        if out.status.success() {
            return Some(String::from_utf8_lossy(&out.stdout).into_owned());
        } else {
            eprintln!(
                "fltk-config via sh failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
    None
}

fn get_cflags(args: &[&str]) -> Vec<String> {
    let args = {
        let mut s = args.join(" ");
        if !s.is_empty() {
            s.push(' ');
        }
        s.push_str("--cflags");
        s
    };
    if let Some(out) = call_fltk_config(&args) {
        out.split_ascii_whitespace().map(|s| s.to_string()).collect()
    } else {
        println!("cargo:warning=Could not run fltk-config --cflags");
        vec![]
    }
}

fn get_lflags(args: &[&str]) -> Vec<String> {
    let args = {
        let mut s = args.join(" ");
        if !s.is_empty() {
            s.push(' ');
        }
        s.push_str("--ldflags");
        s
    };
    if let Some(out) = call_fltk_config(&args) {
        out.split_ascii_whitespace().map(|s| s.to_string()).collect()
    } else {
        println!("cargo:warning=Could not run fltk-config --ldflags");
        vec![]
    }
}
