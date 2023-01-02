use std::{env, path::Path, process::Command};

pub fn build(manifest_dir: &Path, target_triple: &str, out_dir: &Path) {
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
    println!("cargo:rerun-if-env-changed=CFLTK_TOOLCHAIN");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR");
    println!("cargo:rerun-if-env-changed=CFLTK_WAYLAND_ONLY");
    println!("cargo:rerun-if-env-changed=CFLTK_GENERATE_BUNDLE_DIR");
    println!("cargo:rerun-if-changed=cfltk/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cfltk/include/cfl.h");
    println!("cargo:rerun-if-changed=cfltk/include/cfl_widget.h");
    println!("cargo:rerun-if-changed=cfltk/include/cfl_group.h");
    println!("cargo:rerun-if-changed=cfltk/include/cfl_input.h");
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
    println!("cargo:rerun-if-changed=cfltk/include/cfl_utils.h");
    println!("cargo:rerun-if-changed=cfltk/include/cfl_macros.h");
    println!("cargo:rerun-if-changed=cfltk/include/cfl_lock.h");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_widget.hpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_lock.cpp");
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
    println!("cargo:rerun-if-changed=cfltk/src/cfl_draw.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_table.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_tree.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_surface.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_printer.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_font.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_utils.cpp");
    println!("cargo:rerun-if-changed=cfltk/src/cfl_nswindow.m");
    println!("cargo:rerun-if-changed=cfltk/fltk.patch");

    if target_triple.contains("windows") {
        Command::new("git")
            .args(&["apply", "../fltk.patch"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .expect("Git is needed to retrieve the fltk source files!");
    }

    if !target_triple.contains("android") {
        let mut dst = cmake::Config::new("cfltk");

        if cfg!(feature = "fltk-shared") {
            dst.define("CFLTK_BUILD_SHARED", "ON");
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

        if target_triple.contains("linux") && !target_triple.contains("android") {
            if cfg!(feature = "no-pango") {
                dst.define("OPTION_USE_PANGO", "OFF");
                dst.define("OPTION_USE_CAIRO", "OFF");
                // dst.define("FLTK_USE_CAIROXLIB", "OFF");
            } else {
                dst.define("OPTION_USE_PANGO", "ON");
                dst.define("OPTION_USE_CAIRO", "ON");
                // dst.define("FLTK_USE_CAIROXLIB", "ON");
            }
            if cfg!(feature = "use-wayland") {
                dst.define("OPTION_USE_WAYLAND", "ON");
                dst.define("OPTION_ALLOW_GTK_PLUGIN", "OFF");
                if let Ok(wayland_only) = std::env::var("CFLTK_WAYLAND_ONLY") {
                    if wayland_only == "1" {
                        dst.define("OPTION_WAYLAND_ONLY", "ON");
                    }
                }
            } else {
                dst.define("OPTION_USE_WAYLAND", "OFF");
            }
        }

        if target_triple.contains("unknown-linux-musl") {
            dst.define("CMAKE_C_COMPILER", "musl-gcc");
            dst.define("CMAKE_CXX_COMPILER", "musl-g++");
            dst.define("HAVE_STRLCPY", "False");
            dst.define("HAVE_STRLCAT", "False");
        }

        if target_triple.contains("windows") {
            if cfg!(feature = "no-gdiplus") {
                dst.define("OPTION_USE_GDIPLUS", "OFF");
            }
        }

        if cfg!(feature = "single-threaded") {
            dst.define("CFLTK_SINGLE_THREADED", "ON");
        } else {
            dst.define("CFLTK_SINGLE_THREADED", "OFF");
        }

        let profile = if let Ok(prof) = env::var("OPT_LEVEL") {
            if prof == "z" || prof == "s" {
                "MinSizeRel"
            } else {
                "Release"
            }
        } else {
            "Release"
        };

        if target_triple == "aarch64-apple-darwin" {
            dst.define("CMAKE_OSX_ARCHITECTURES", "arm64");
        } else if target_triple == "x86_64-apple-darwin" {
            dst.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
        }

        let _dst = dst
            .profile(profile)
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("CFLTK_CARGO_BUILD", "ON")
            .define("FLTK_BUILD_EXAMPLES", "OFF")
            .define("FLTK_BUILD_TEST", "OFF")
            .define("FLTK_BUILD_FLUID", "OFF")
            .define("OPTION_LARGE_FILE", "ON")
            .define("OPTION_USE_THREADS", "ON")
            .define("OPTION_BUILD_HTML_DOCUMENTATION", "OFF")
            .define("OPTION_BUILD_PDF_DOCUMENTATION", "OFF")
            .build();
    } else {
        crate::android::build(out_dir, target_triple);
    }

    if target_triple.contains("android") || target_triple.contains("windows") {
        Command::new("git")
            .args(&["reset", "--hard"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .expect("Git is needed to retrieve the fltk source files!");
    }
}
