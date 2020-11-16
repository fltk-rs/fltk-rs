use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_triple = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let mut dst = cmake::Config::new("cfltk");

    println!("cargo:rerun-if-changed=build.rs");

    let mut platform = target_os.to_string();

    if target_os.as_str() == "windows" {
        if cfg!(target_env = "gnu") {
            platform.push_str("-gnu");
        } else {
            platform.push_str("-msvc");
        }
    }

    if cfg!(feature = "fltk-bundled") {
        let url = PathBuf::from(format!(
            "https://github.com/MoAlyousef/fltk-rs/releases/download/{}/lib_x64-{}.tar.gz",
            pkg_version, platform
        ));

        Command::new("curl")
            .args(&["-LOk", url.to_str().unwrap()])
            .current_dir(out_dir.clone())
            .status()
            .expect("Curl and Tar are needed to download and upack the bundled libraries!");

        Command::new("tar")
            .args(&["-xzvf", url.file_name().unwrap().to_str().unwrap()])
            .current_dir(out_dir.clone())
            .status()
            .expect("Curl and Tar are needed to download and upack the bundled libraries!");
    } else {
        println!("cargo:rerun-if-changed=cfltk/CMakeLists.txt");
        println!("cargo:rerun-if-changed=cfltk/include/cfl.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_widget.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_group.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_input.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_output.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_window.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_button.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_box.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_menu.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_dialog.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_valuator.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_browser.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_misc.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_text.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_image.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_draw.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_table.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_surface.h");
        println!("cargo:rerun-if-changed=cfltk/include/cfl_printer.h");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_global.hpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_new.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_widget.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_group.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_window.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_button.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_box.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_menu.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_dialog.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_valuator.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_browser.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_misc.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_text.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_image.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_input.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_output.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_draw.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_table.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_tree.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_surface.cpp");
        println!("cargo:rerun-if-changed=cfltk/src/cfl_printer.cpp");

        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .current_dir(manifest_dir.clone())
            .status()
            .expect("Git is needed to retrieve the fltk source files!");

        Command::new("git")
            .args(&["apply", "../fltk.patch"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .expect("Git is needed to retrieve the fltk source files!");

        if cfg!(feature = "fltk-shared") {
            dst.define("CFLTK_BUILD_SHARED", "ON");
        }

        if cfg!(feature = "use-ninja") {
            dst.generator("Ninja");
        }

        if cfg!(feature = "system-fltk") {
            dst.define("USE_SYSTEM_FLTK", "ON");
        }

        if cfg!(feature = "system-libpng") {
            dst.define("OPTION_USE_SYSTEM_LIBPNG", "ON");
        } else {
            dst.define("OPTION_USE_SYSTEM_LIBPNG", "OFF");
        }

        if cfg!(feature = "system-libjpeg") {
            dst.define("OPTION_USE_SYSTEM_LIBJPEG", "ON");
        } else {
            dst.define("OPTION_USE_SYSTEM_LIBJPEG", "OFF");
        }

        if cfg!(feature = "system-zlib") {
            dst.define("OPTION_USE_SYSTEM_ZLIB", "ON");
        } else {
            dst.define("OPTION_USE_SYSTEM_ZLIB", "OFF");
        }

        if cfg!(feature = "no-images") {
            dst.define("CFLTK_LINK_IMAGES", "OFF");
        } else {
            dst.define("CFLTK_LINK_IMAGES", "ON");
        }

        if cfg!(feature = "legacy-opengl") {
            dst.define("OpenGL_GL_PREFERENCE", "LEGACY");
        } else {
            dst.define("OpenGL_GL_PREFERENCE", "GLVND");
        }

        if cfg!(feature = "enable-glwindow") {
            dst.define("OPTION_USE_GL", "ON");
            dst.define("CFLTK_USE_OPENGL", "ON");
        } else {
            dst.define("OPTION_USE_GL", "OFF");
            dst.define("CFLTK_USE_OPENGL", "OFF");
        }

        if let Ok(toolchain) = env::var("CFLTK_TOOLCHAIN") {
            dst.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
        }

        if target_triple.contains("android") {
            handle_android(&target_triple, &mut dst);
        }

        if target_triple.contains("linux") && !target_triple.contains("android") {
            if cfg!(feature = "no-pango") {
                dst.define("OPTION_USE_PANGO", "OFF");
            } else {
                dst.define("OPTION_USE_PANGO", "ON");
            }
            
        }

        if target_triple.contains("unknown-linux-musl") {
            dst.define("CMAKE_C_COMPILER", "musl-gcc");
            dst.define("CMAKE_CXX_COMPILER", "musl-gcc");
            dst.define("HAVE_STRLCPY", "False");
            dst.define("HAVE_STRLCAT", "False");
        }

        let _dst = dst
            .profile("Release")
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("FLTK_BUILD_EXAMPLES", "OFF")
            .define("FLTK_BUILD_TEST", "OFF")
            .define("FLTK_BUILD_FLUID", "OFF")
            .define("OPTION_USE_THREADS", "ON")
            .define("OPTION_LARGE_FILE", "ON")
            .define("OPTION_BUILD_HTML_DOCUMENTATION", "OFF")
            .define("OPTION_BUILD_PDF_DOCUMENTATION", "OFF")
            .build();
    }

    Command::new("git")
        .args(&["reset", "--hard"])
        .current_dir(manifest_dir.join("cfltk").join("fltk"))
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").join("Release").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib64").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").join("Release").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib64").join("Release").display()
    );

    if !cfg!(feature = "fltk-shared") {
        println!("cargo:rustc-link-lib=static=cfltk");
    } else {
        println!("cargo:rustc-link-lib=dylib=cfltk");
    }

    if !cfg!(feature = "fltk-shared") {
        println!("cargo:rustc-link-lib=static=fltk");

        if !cfg!(features = "no-images") {
            println!("cargo:rustc-link-lib=static=fltk_images");

            if cfg!(feature = "system-libpng") {
                println!("cargo:rustc-link-lib=dylib=png");
            } else {
                println!("cargo:rustc-link-lib=static=fltk_png");
            }

            if cfg!(feature = "system-libjpeg") {
                println!("cargo:rustc-link-lib=dylib=jpeg");
            } else {
                println!("cargo:rustc-link-lib=static=fltk_jpeg");
            }

            if cfg!(feature = "system-zlib") {
                println!("cargo:rustc-link-lib=dylib=z");
            } else {
                println!("cargo:rustc-link-lib=static=fltk_z");
            }
        }

        if cfg!(feature = "enable-glwindow") {
            println!("cargo:rustc-link-lib=static=fltk_gl");
        }

        match target_os.as_str() {
            "macos" => {
                println!("cargo:rustc-link-lib=framework=Carbon");
                println!("cargo:rustc-link-lib=framework=Cocoa");
                println!("cargo:rustc-link-lib=framework=ApplicationServices");
            }
            "windows" => {
                println!("cargo:rustc-link-lib=dylib=ws2_32");
                println!("cargo:rustc-link-lib=dylib=comctl32");
                println!("cargo:rustc-link-lib=dylib=gdi32");
                println!("cargo:rustc-link-lib=dylib=oleaut32");
                println!("cargo:rustc-link-lib=dylib=ole32");
                println!("cargo:rustc-link-lib=dylib=uuid");
                println!("cargo:rustc-link-lib=dylib=shell32");
                println!("cargo:rustc-link-lib=dylib=advapi32");
                println!("cargo:rustc-link-lib=dylib=comdlg32");
                println!("cargo:rustc-link-lib=dylib=winspool");
                println!("cargo:rustc-link-lib=dylib=user32");
                println!("cargo:rustc-link-lib=dylib=kernel32");
                println!("cargo:rustc-link-lib=dylib=odbc32");
            }
            "android" => {
                println!("cargo:rustc-link-lib=log");
                println!("cargo:rustc-link-lib=android");
                println!("cargo:rustc-link-lib=c++_shared");
            }
            "ios" => {
                // Experimental
                println!("cargo:rustc-link-lib=framework=UIKit");
            }
            _ => {
                println!("cargo:rustc-link-lib=dylib=pthread");
                println!("cargo:rustc-link-lib=dylib=X11");
                println!("cargo:rustc-link-lib=dylib=Xext");
                println!("cargo:rustc-link-lib=dylib=Xinerama");
                println!("cargo:rustc-link-lib=dylib=Xcursor");
                println!("cargo:rustc-link-lib=dylib=Xrender");
                println!("cargo:rustc-link-lib=dylib=Xfixes");
                println!("cargo:rustc-link-lib=dylib=Xft");
                println!("cargo:rustc-link-lib=dylib=fontconfig");
                if !cfg!(feature = "no-pango") {
                    println!("cargo:rustc-link-lib=dylib=pango-1.0");
                    println!("cargo:rustc-link-lib=dylib=pangoxft-1.0");
                    println!("cargo:rustc-link-lib=dylib=gobject-2.0");
                    println!("cargo:rustc-link-lib=dylib=cairo");
                    println!("cargo:rustc-link-lib=dylib=pangocairo-1.0");
                }
            }
        }
    }
}

fn handle_android(triple: &str, dst: &mut cmake::Config) {
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

    dst.generator("Ninja");
    dst.define("CMAKE_SYSTEM_NAME", "Android");
    dst.define("CMAKE_SYSTEM_VERSION", "21");
    dst.define("ANDROID_PLATFORM", "android-21");
    dst.define("CMAKE_ANDROID_NDK", &ndk);
    dst.define("ANDROID_NDK", &ndk);
    dst.define(
        "CMAKE_MAKE_PROGRAM",
        find_ninja(&sdk).expect("Couldn't find NDK ninja!"),
    );
    dst.define(
        "CMAKE_TOOLCHAIN_FILE",
        ndk.join("build")
            .join("cmake")
            .join("android.toolchain.cmake"),
    );

    match triple {
        "i686-linux-android" => {
            dst.define("ANDROID_ABI", "x86");
            dst.define("CMAKE_ANDROID_ARCH_ABI", "x86");
        }
        "aarch64-linux-android" => {
            dst.define("ANDROID_ABI", "arm64-v8a");
            dst.define("CMAKE_ANDROID_ARCH_ABI", "arm64-v8a");
        }
        "armv7-linux-androideabi" => {
            dst.define("ANDROID_ABI", "armeabi-v7a");
            dst.define("CMAKE_ANDROID_ARCH_ABI", "armeabi-v7a");
        }
        "x86_64-linux-android" => {
            dst.define("ANDROID_ABI", "x86_64");
            dst.define("CMAKE_ANDROID_ARCH_ABI", "x86_64");
        }
        _ => panic!("Unknown android triple"),
    }
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
