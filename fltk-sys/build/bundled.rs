use std::{env, path::PathBuf, process::Command};

pub fn get(target_os: String, out_dir: PathBuf) {
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    if let Ok(cfltk_path) = env::var("CFLTK_BUNDLE_DIR") {
        println!("cargo:rustc-link-search=native={}", cfltk_path);
    } else {
        let url = if let Ok(cfltk_url) = env::var("CFLTK_BUNDLE_URL") {
            PathBuf::from(cfltk_url)
        } else {
            let mut platform = target_os.to_string();

            if target_os.as_str() == "windows" {
                if cfg!(target_env = "gnu") {
                    platform.push_str("-gnu");
                } else {
                    platform.push_str("-msvc");
                }
            }

            PathBuf::from(format!(
                "https://github.com/fltk-rs/fltk-rs/releases/download/{}/lib_x64-{}.tar.gz",
                pkg_version, platform
            ))
        };

        Command::new("curl")
            .args(&["-LOk", url.to_str().unwrap()])
            .current_dir(out_dir.clone())
            .status()
            .expect("Curl and Tar are needed to download and upack the bundled libraries!");

        Command::new("tar")
            .args(&["-xzvf", url.file_name().unwrap().to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Curl and Tar are needed to download and upack the bundled libraries!");
    }
}
