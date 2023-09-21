use std::process::Command;

pub fn build() {
    let lflags = get_lflags();
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
    for dir in search_dirs {
        println!("cargo:rustc-link-search=native={}", dir);
    }
    for lib in libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}

fn get_lflags() -> Vec<String> {
    if let Ok(lflags) = Command::new("pkg-config")
        .args([
            "--libs",
            if !cfg!(feature = "fltk-shared") {
                "--static"
            } else {
                ""
            },
            "cfltk",
        ])
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
