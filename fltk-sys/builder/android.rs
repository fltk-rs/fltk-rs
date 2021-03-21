use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(out_dir: PathBuf, target_triple: String) {
    let sdk =
        PathBuf::from(env::var("ANDROID_SDK_ROOT").expect("ANDROID_SDK_ROOT needs to be set!"));
    let mut ndk: Option<PathBuf> = None;
    if let Ok(root) = env::var("ANDROID_NDK_ROOT") {
        ndk = Some(PathBuf::from(root));
    }
    // fallback to NDK_HOME
    if ndk.is_none() {
        ndk = Some(PathBuf::from(
            env::var("NDK_HOME").expect("ANDROID_NDK_ROOT or NDK_HOME need to be set!"),
        ));
    }

    let ndk = ndk.expect("ANDROID_NDK_ROOT or NDK_HOME need to be set!");

    let cmake_build_dir = out_dir.join("cmake_build").to_str().unwrap().to_string();
    let mut cmd = vec![];
    cmd.push(format!("-B{}", cmake_build_dir));
    cmd.push("-DOPTION_USE_GL=OFF".to_string());
    cmd.push("-DOPTION_USE_SYSTEM_ZLIB=OFF".to_string());
    cmd.push("-DCFLTK_USE_OPENGL=OFF".to_string());
    cmd.push("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON".to_string());
    cmd.push("-DFLTK_BUILD_EXAMPLES=OFF".to_string());
    cmd.push("-DFLTK_BUILD_TEST=OFF".to_string());
    cmd.push("-DOPTION_USE_THREADS=ON".to_string());
    cmd.push("-DOPTION_LARGE_FILE=ON".to_string());
    cmd.push("-DOPTION_BUILD_HTML_DOCUMENTATION=OFF".to_string());
    cmd.push("-DOPTION_BUILD_PDF_DOCUMENTATION=OFF".to_string());
    cmd.push("-DCMAKE_BUILD_TYPE=Release".to_string());
    cmd.push(format!(
        "-DCMAKE_INSTALL_PREFIX={}",
        out_dir.to_str().unwrap()
    ));
    cmd.push("-GNinja".to_string());
    cmd.push("-DCMAKE_SYSTEM_NAME=Android".to_string());
    cmd.push("-DCMAKE_SYSTEM_VERSION=21".to_string());
    cmd.push("-DANDROID_PLATFORM=android-21".to_string());
    cmd.push(format!("-DCMAKE_ANDROID_NDK={}", &ndk.to_str().unwrap()));
    cmd.push(format!("-DANDROID_NDK={}", &ndk.to_str().unwrap()));
    cmd.push(format!(
        "-DCMAKE_MAKE_PROGRAM={}",
        find_ninja(&sdk)
            .expect("Couldn't find NDK ninja!")
            .to_str()
            .unwrap()
    ));
    cmd.push(format!(
        "-DCMAKE_TOOLCHAIN_FILE={}",
        ndk.join("build")
            .join("cmake")
            .join("android.toolchain.cmake")
            .to_str()
            .unwrap()
    ));

    match target_triple.as_str() {
        "i686-linux-android" => {
            cmd.push("-DANDROID_ABI=x86".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86".to_string());
        }
        "aarch64-linux-android" => {
            cmd.push("-DANDROID_ABI=arm64-v8a".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=arm64-v8a".to_string());
        }
        "armv7-linux-androideabi" => {
            cmd.push("-DANDROID_ABI=armeabi-v7a".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=armeabi-v7a".to_string());
        }
        "x86_64-linux-android" => {
            cmd.push("-DANDROID_ABI=x86_64".to_string());
            cmd.push("-DCMAKE_ANDROID_ARCH_ABI=x86_64".to_string());
        }
        _ => panic!("Unknown android triple"),
    }

    Command::new("cmake")
        .args(&cmd)
        .current_dir("cfltk")
        .status()
        .expect("CMake is needed for android builds!");

    Command::new("cmake")
        .args(&["--build", &cmake_build_dir, "--target", "install"])
        .current_dir("cfltk")
        .status()
        .expect("CMake is needed for android builds!");
}

fn find_ninja(sdk_path: &Path) -> Option<PathBuf> {
    let cmk = sdk_path.join("cmake");
    for subdir in fs::read_dir(cmk).unwrap() {
        let subdir = subdir
            .unwrap() // Shouldn't fail!
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        if subdir.starts_with("3.") {
            return Some(
                sdk_path
                    .join("cmake")
                    .join(subdir)
                    .join("bin")
                    .join("ninja"),
            );
        }
    }
    None
}
