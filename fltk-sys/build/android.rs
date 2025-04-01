use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn build(out_dir: &Path, target_triple: &str) {
    println!("cargo:rerun-if-env-changed=ANDROID_SDK_ROOT");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_ROOT");

    let sdk =
        PathBuf::from(env::var("ANDROID_SDK_ROOT").expect("ANDROID_SDK_ROOT needs to be set!"));
    let ndk = env::var("ANDROID_NDK_ROOT")
        .or_else(|_| env::var("NDK_HOME"))
        .expect("ANDROID_NDK_ROOT or NDK_HOME need to be set!");
    let ndk = PathBuf::from(ndk);

    let toolchain_file = ndk
        .join("build")
        .join("cmake")
        .join("android.toolchain.cmake");

    let abi = match target_triple {
        "i686-linux-android" => "x86",
        "aarch64-linux-android" => "arm64-v8a",
        "armv7-linux-androideabi" => "armeabi-v7a",
        "x86_64-linux-android" => "x86_64",
        _ => panic!("Unknown android triple"),
    };

    let ninja_path = find_ninja(&sdk).expect("Couldn't find NDK ninja!");

    let mut cfg = cmake::Config::new("cfltk");

    cfg.generator("Ninja")
        .out_dir(out_dir.join("cmake_build"))
        .define("CMAKE_SYSTEM_NAME", "Android")
        .define("CMAKE_SYSTEM_VERSION", "21")
        .define("ANDROID_PLATFORM", "android-21")
        .define("ANDROID_ABI", abi)
        .define("CMAKE_ANDROID_ARCH_ABI", abi)
        .define("CMAKE_ANDROID_NDK", ndk.to_str().unwrap())
        .define("ANDROID_NDK", ndk.to_str().unwrap())
        .define("CMAKE_TOOLCHAIN_FILE", toolchain_file.to_str().unwrap())
        .define("CMAKE_MAKE_PROGRAM", ninja_path.to_str().unwrap())
        .define("CMAKE_INSTALL_PREFIX", out_dir.to_str().unwrap())
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .define("CFLTK_CARGO_BUILD", "ON")
        .define("CFLTK_SINGLE_THREADED", "OFF")
        .define("CFLTK_USE_OPENGL", "OFF")
        .define("FLTK_BUILD_GL", "OFF")
        .define("FLTK_BUILD_EXAMPLES", "OFF")
        .define("FLTK_BUILD_TEST", "OFF")
        .define("FLTK_BUILD_HTML_DOCS", "OFF")
        .define("FLTK_BUILD_PDF_DOCS", "OFF")
        .define("FLTK_USE_SYSTEM_ZLIB", "OFF")
        .define("FLTK_OPTION_LARGE_FILE", "ON")
        .very_verbose(true)
        .build();
}

fn find_ninja(sdk_path: &Path) -> Option<PathBuf> {
    let cmk = sdk_path.join("cmake");
    for subdir in fs::read_dir(&cmk).ok()? {
        let subdir = subdir.ok()?.path();
        let name = subdir.file_name()?.to_str()?;
        if name.starts_with("3.") {
            return Some(subdir.join("bin").join("ninja"));
        }
    }
    None
}
