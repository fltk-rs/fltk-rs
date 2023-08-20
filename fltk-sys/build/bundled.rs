use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn get(target_triple: &str, out_dir: &Path) {
    if let Ok(cfltk_path) = env::var("CFLTK_BUNDLE_DIR") {
        println!("cargo:rustc-link-search=native={}", cfltk_path);
    } else {
        let url = if let Ok(cfltk_url) = env::var("CFLTK_BUNDLE_URL") {
            PathBuf::from(cfltk_url)
        } else {
            PathBuf::from(format!(
                "{}/lib_{}.tar.gz",
                env::var("CFLTK_BUNDLE_URL_PREFIX").unwrap_or_else(|_| String::from(
                    "https://github.com/MoAlyousef/cfltk/releases/latest/download"
                )),
                target_triple,
            ))
        };

        let curl_status = Command::new("curl")
            .args(["-LOkf", url.to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Curl is needed to download the bundled libraries!");

        if !curl_status.success() {
            panic!("Download bundled libraries from {:?} failed", url)
        }

        let tar_status = Command::new("tar")
            .args(["-xzvf", url.file_name().unwrap().to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Tar is needed to upack the bundled libraries!");

        if !tar_status.success() {
            panic!("Unpack bundled libraries failed")
        }
    }
}
