use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(out_dir: &Path) {
    let host = env::var("HOST").unwrap();
    const TOOLCHAIN_SUBPATH: &str = "cmake/Modules/Platform/Emscripten.cmake";
    let emscripten_root = if let Ok(emsdk) = env::var("EMSDK") {
        PathBuf::from(emsdk).join("upstream/emscripten")
    } else if let Ok(emscripten) = env::var("EMSCRIPTEN_ROOT") {
        // Users can define EMSCRIPTEN_ROOT as with godot engine
        PathBuf::from(emscripten)
    } else {
        // Assume emscripten is globally installed. In that case we need to invoke em-config
        let em_config = if host.contains("windows") {
            "em-config.bat"
        } else {
            "em-config"
        };
        let output = Command::new(em_config)
            .arg("EMSCRIPTEN_ROOT")
            .output()
            .expect("Failed to find emscripten toolchain!")
            .stdout;
        PathBuf::from(std::str::from_utf8(&output).unwrap().trim())
    };

    let toolchain_file = emscripten_root.join(TOOLCHAIN_SUBPATH);

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
    cmk::Config::new(out_dir.join("fltk_wasm32_emscripten"))
        .profile("Release")
        .generator("Ninja")
        .define("FLTK_USE_PTHREADS", "OFF")
        .define("FLTK_BUILD_FLUID", "OFF")
        .define("FLTK_BUILD_FLTK_OPTIONS", "OFF")
        .define("FLTK_BUILD_TEST", "OFF")
        .define("FLTK_BUILD_GL", "OFF")
        .define("FLTK_BACKEND_WAYLAND", "OFF")
        .define("FLTK_BACKEND_X11", "OFF")
        .define("CMAKE_TOOLCHAIN_FILE", &toolchain_file)
        .build();

    cmk::Config::new("cfltk")
        .profile("Release")
        .generator("Ninja")
        .define("USE_SYSTEM_FLTK", "ON")
        .define("CFLTK_USE_OPENGL", "OFF")
        .define("CFLTK_SINGLE_THREADED", "ON")
        .define("CFLTK_CARGO_BUILD", "ON")
        .define("FLTK_DIR", out_dir.join("share").join("fltk"))
        .define("CMAKE_TOOLCHAIN_FILE", toolchain_file)
        .build();
}
