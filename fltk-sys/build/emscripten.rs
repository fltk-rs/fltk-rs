use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(out_dir: &Path, target_triple: &str) {
    let emscripten = PathBuf::from("emscripten")
        .join("cmake")
        .join("Modules")
        .join("Platform")
        .join("Emscripten.cmake");

    let toolchain_file = if let Ok(emsdk) = env::var("EMSDK") {
        PathBuf::from(emsdk).join("upstream").join(emscripten)
    } else if let Ok(mingw_prefix) = env::var("MINGW_PREFIX") {
        println!("cargo:warning=EMSDK not set, looking for default msys2 installation path!");
        PathBuf::from(mingw_prefix).join("lib").join(emscripten)
    } else if target_triple.contains("linux") {
        println!("cargo:warning=EMSDK not set, looking for default linux installation path!");
        PathBuf::from("/usr/share").join(emscripten)
    } else {
        println!("cargo:warning=EMSDK not set, checking EMSCRIPTEN_TOOLCHAIN_FILE env variable!");
        PathBuf::from(
            env::var("EMSCRIPTEN_TOOLCHAIN_FILE")
                .expect("Couldn't find Emscripten.cmake toolchain file!"),
        )
    };

    Command::new("git")
        .args([
            "clone",
            "-b",
            "emscripten",
            "https://github.com/MoAlyousef/fltk_wasm32_emscripten",
            "--depth=1",
        ])
        .current_dir(out_dir)
        .status()
        .ok();
    Command::new("cmake")
        .args([
            "-Bbin",
            "-GNinja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DFLTK_USE_PTHREADS=OFF",
            "-DFLTK_BUILD_FLUID=OFF",
            "-DFLTK_BUILD_FLTK_OPTIONS=OFF",
            "-DFLTK_BUILD_TEST=OFF",
            "-DFLTK_BUILD_GL=OFF",
            "-DFLTK_BACKEND_WAYLAND=OFF",
            "-DFLTK_BACKEND_X11=OFF",
            &format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file.display()),
            &format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()),
        ])
        .current_dir(out_dir.join("fltk_wasm32_emscripten"))
        .status()
        .ok();
    Command::new("cmake")
        .args(["--build", "bin", "--target", "install"])
        .current_dir(out_dir.join("fltk_wasm32_emscripten"))
        .status()
        .ok();

    Command::new("cmake")
        .args([
            "-Bbin",
            "-GNinja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DUSE_SYSTEM_FLTK=ON",
            "-DCFLTK_USE_OPENGL=OFF",
            "-DCFLTK_SINGLE_THREADED=ON",
            "-DCFLTK_CARGO_BUILD=ON",
            &format!("-DCMAKE_PREFIX_PATH={}", out_dir.display()),
            &format!(
                "-DFLTK_DIR={}",
                out_dir.join("share").join("fltk").display()
            ),
            &format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file.display()),
            &format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()),
        ])
        .current_dir("cfltk")
        .status()
        .ok();
    Command::new("cmake")
        .args(["--build", "bin", "--target", "install"])
        .current_dir("cfltk")
        .status()
        .ok();
}
