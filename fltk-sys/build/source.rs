use crate::utils;
use std::{env, path::Path, process::Command};

pub fn build(manifest_dir: &Path, target_triple: &str, out_dir: &Path) {
    utils::check_cfltk_empty();
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CFLTK_TOOLCHAIN");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR");
    println!("cargo:rerun-if-env-changed=CFLTK_WAYLAND_ONLY");
    println!("cargo:rerun-if-env-changed=CFLTK_GENERATE_BUNDLE_DIR");
    println!("cargo:rerun-if-changed=cfltk/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cfltk/fltk.patch");
    println!("cargo:rerun-if-changed=cfltk/include");
    println!("cargo:rerun-if-changed=cfltk/src");
    if target_triple.contains("darwin") {
        println!("cargo:rerun-if-env-changed=SDKROOT");
    }

    if target_triple.contains("windows") {
        if !crate::utils::has_program("git") {
            println!("cargo:warning=Could not find invokable git. It's needed to apply a security patch on windows!");
        }
        Command::new("git")
            .args(["apply", "../fltk.patch"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .ok();
    }

    if !target_triple.contains("android") && !target_triple.contains("emscripten") {
        let mut dst = cmake::Config::new("cfltk");

        if crate::utils::use_static_msvcrt() && target_triple.contains("windows-msvc") {
            dst.define("CFLTK_MSVC_CRT_STATIC", "ON");
        }

        if cfg!(feature = "fltk-shared") {
            dst.define("CFLTK_BUILD_SHARED", "ON");
        }

        if cfg!(feature = "cairoext") {
            dst.define("FLTK_OPTION_CAIRO_EXT", "ON");
            dst.define("CFLTK_USE_CAIROEXT", "ON");
        }

        if (cfg!(feature = "use-ninja") && crate::utils::has_program("ninja"))
            || (target_triple.contains("windows-msvc") && crate::utils::has_program("ninja"))
        {
            dst.generator("Ninja");
        }

        if cfg!(feature = "system-fltk") {
            dst.define("USE_SYSTEM_FLTK", "ON");
        }

        if cfg!(feature = "system-libpng") {
            dst.define("FLTK_USE_SYSTEM_LIBPNG", "ON");
        } else {
            dst.define("FLTK_USE_SYSTEM_LIBPNG", "OFF");
        }

        if cfg!(feature = "system-libjpeg") {
            dst.define("FLTK_USE_SYSTEM_LIBJPEG", "ON");
        } else {
            dst.define("FLTK_USE_SYSTEM_LIBJPEG", "OFF");
        }

        if cfg!(feature = "system-zlib") {
            dst.define("FLTK_USE_SYSTEM_ZLIB", "ON");
        } else {
            dst.define("FLTK_USE_SYSTEM_ZLIB", "OFF");
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
            dst.define("FLTK_BUILD_GL", "ON");
            dst.define("CFLTK_USE_OPENGL", "ON");
        } else {
            dst.define("FLTK_BUILD_GL", "OFF");
            dst.define("CFLTK_USE_OPENGL", "OFF");
        }

        if let Ok(toolchain) = env::var("CFLTK_TOOLCHAIN") {
            dst.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
        }

        if target_triple.contains("linux") {
            if cfg!(feature = "no-pango") {
                dst.define("FLTK_USE_PANGO", "OFF");
                dst.define("FLTK_GRAPHICS_CAIRO", "OFF");
            } else {
                dst.define("FLTK_USE_PANGO", "ON");
                dst.define("FLTK_GRAPHICS_CAIRO", "ON");
            }
            if cfg!(feature = "use-wayland") {
                dst.define("FLTK_BACKEND_WAYLAND", "ON");
                dst.define("FLTK_USE_LIBDECOR_GTK", "OFF");
                dst.define("FLTK_USE_SYSTEM_LIBDECOR", "OFF");
                if let Ok(wayland_only) = std::env::var("CFLTK_WAYLAND_ONLY") {
                    if wayland_only == "1" {
                        dst.define("FLTK_BACKEND_X11", "OFF");
                    }
                }
            } else {
                dst.define("FLTK_BACKEND_WAYLAND", "OFF");
            }
        }

        if target_triple.contains("unknown-linux-musl") {
            dst.define("CMAKE_C_COMPILER", "musl-gcc");
            dst.define("CMAKE_CXX_COMPILER", "musl-g++");
            dst.define("HAVE_STRLCPY", "False");
            dst.define("HAVE_STRLCAT", "False");
        }

        if target_triple.contains("windows") && cfg!(feature = "no-gdiplus") {
            dst.define("FLTK_GRAPHICS_GDIPLUS", "OFF");
        }

        if cfg!(feature = "single-threaded") {
            dst.define("CFLTK_SINGLE_THREADED", "ON");
            dst.define("FLTK_USE_PTHREADS", "OFF");
        }

        let profile = if let Ok(prof) = env::var("OPT_LEVEL") {
            match prof.as_str() {
                "z" | "s" => "MinSizeRel",
                "0" if !target_triple.contains("msvc") => "Debug",
                _ => "Release",
            }
        } else {
            "Release"
        };

        if target_triple.contains("darwin") {
            let deployment_target = utils::get_macos_deployment_target();
            dst.define(
                "CMAKE_OSX_DEPLOYMENT_TARGET",
                &format!("{}", deployment_target),
            );
            if target_triple == "aarch64-apple-darwin" {
                dst.define("CMAKE_OSX_ARCHITECTURES", "arm64");
            } else if target_triple == "x86_64-apple-darwin" {
                dst.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
            }
        }

        let _dst = dst
            .profile(profile)
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("CFLTK_CARGO_BUILD", "ON")
            .define("FLTK_BUILD_EXAMPLES", "OFF")
            .define("FLTK_BUILD_TEST", "OFF")
            .define("FLTK_BUILD_FLUID", "OFF")
            .define("FLTK_BUILD_FLTK_OPTIONS", "OFF")
            .define("FLTK_OPTION_LARGE_FILE", "ON")
            .define("FLTK_BUILD_HTML_DOCS", "OFF")
            .define("FLTK_BUILD_PDF_DOCS", "OFF")
            .build();
    } else if target_triple.contains("android") {
        crate::android::build(out_dir, target_triple);
    } else if target_triple.contains("emscripten") {
        crate::emscripten::build(out_dir);
    }

    if target_triple.contains("android") || target_triple.contains("windows") {
        Command::new("git")
            .args(["reset", "--hard"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .ok();
    }
}
