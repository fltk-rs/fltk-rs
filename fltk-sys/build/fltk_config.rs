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
    "cfltk/src/cfl_printer.cpp",
];

pub fn build() {
    let mut args = vec![];
    let mut use_gl = false;
    if cfg!(feature = "enable-glwindow") {
        args.push("--use-gl");
        use_gl = true;
    }
    if !cfg!(feature = "no-images") {
        args.push("--use-images");
    }
    let cflags = get_cflags(&args);
    let mut include_paths = vec![];
    let mut defs = vec![];
    for cflag in cflags.into_iter() {
        if let Some(stripped) = cflag.strip_prefix("-I") {
            include_paths.push(stripped.to_string());
        } else if let Some(stripped) = cflag.strip_prefix("-I") {
            defs.push(stripped.to_string());
        } else {
        }
    }
    let lflags = get_lflags(&args);
    let mut search_dirs = vec![];
    let mut libs = vec![];
    for lflag in lflags {
        if let Some(stripped) = lflag.strip_prefix("-l") {
            libs.push(stripped.to_string());
        } else if let Some(stripped) = lflag.strip_prefix("-L") {
            search_dirs.push(stripped.to_string());
        } else {
        }
    }
    let mut b = cc::Build::new();
    b.cpp(true);
    b.files(CPP_SRC);
    if cfg!(target_os = "macos") {
        b.file("cfltk/src/cfl_nswindow.m");
    }
    if use_gl {
        b.define("CFLTK_USE_GL", None);
        b.file("cfltk/src/glad.c");
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

fn get_cflags(args: &[&str]) -> Vec<String> {
    if let Ok(cflags) = Command::new("fltk-config")
        .args(args)
        .arg("--cflags")
        .output()
    {
        let cflags = String::from_utf8_lossy(&cflags.stdout).to_string();
        let cflags: Vec<String> = cflags
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        cflags
    } else {
        vec![]
    }
}

fn get_lflags(args: &[&str]) -> Vec<String> {
    if let Ok(lflags) = Command::new("fltk-config")
        .args(args)
        .arg("--ldflags")
        .output()
    {
        let lflags = String::from_utf8_lossy(&lflags.stdout).to_string();
        let lflags: Vec<String> = lflags
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        lflags
    } else {
        vec![]
    }
}
