use std::{env, path::Path, process::Command};
use crate::utils;

pub fn build(manifest_dir: &Path, target_triple: &str, out_dir: &Path) {
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
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
    println!("cargo:rerun-if-changed=cfltk/src/cfl_lock.hpp");
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

    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .current_dir(manifest_dir)
        .status()
        .expect("Git is needed to retrieve the fltk source files!");

    if target_triple.contains("android") || target_triple.contains("windows") {
        Command::new("git")
            .args(&["apply", "../fltk.patch"])
            .current_dir(manifest_dir.join("cfltk").join("fltk"))
            .status()
            .expect("Git is needed to retrieve the fltk source files!");
    }

    if !target_triple.contains("android") {
        let cmake_build_dir = out_dir.to_str().unwrap().to_string();
        let mut cmd = vec![];
        cmd.push(format!("-B{}", cmake_build_dir));

        if cfg!(feature = "fltk-shared") {
            cmd.push("-DCFLTK_BUILD_SHARED=ON".to_string());
        }

        if cfg!(feature = "use-ninja") || crate::utils::has_program("ninja") {
            cmd.push("-GNinja".to_string());
        }

        if cfg!(feature = "system-fltk") {
            cmd.push("-DUSE_SYSTEM_FLTK=ON".to_string());
        }

        if cfg!(feature = "system-libpng")
            || (!target_triple.contains("apple")
                && !target_triple.contains("windows")
                && !target_triple.contains("android"))
        {
            cmd.push("-DOPTION_USE_SYSTEM_LIBPNG=ON".to_string());
        } else {
            cmd.push("-DOPTION_USE_SYSTEM_LIBPNG=OFF".to_string());
        }

        if cfg!(feature = "system-libjpeg") {
            cmd.push("-DOPTION_USE_SYSTEM_LIBJPEG=ON".to_string());
        } else {
            cmd.push("-DOPTION_USE_SYSTEM_LIBJPEG=OFF".to_string());
        }

        if cfg!(feature = "system-zlib") {
            cmd.push("-DOPTION_USE_SYSTEM_ZLIB=ON".to_string());
        } else {
            cmd.push("-DOPTION_USE_SYSTEM_ZLIB=OFF".to_string());
        }

        if cfg!(feature = "no-images") {
            cmd.push("-DCFLTK_LINK_IMAGES=OFF".to_string());
        } else {
            cmd.push("-DCFLTK_LINK_IMAGES=ON".to_string());
        }

        if cfg!(feature = "legacy-opengl") {
            cmd.push("-DOpenGL_GL_PREFERENCE=LEGACY".to_string());
        } else {
            cmd.push("-DOpenGL_GL_PREFERENCE=GLVND".to_string());
        }

        if cfg!(feature = "enable-glwindow") {
            cmd.push("-DOPTION_USE_GL=ON".to_string());
            cmd.push("-DCFLTK_USE_OPENGL=ON".to_string());
        } else {
            cmd.push("-DOPTION_USE_GL=OFF".to_string());
            cmd.push("-DCFLTK_USE_OPENGL=OFF".to_string());
        }

        if let Ok(toolchain) = env::var("CFLTK_TOOLCHAIN") {
            cmd.push(format!("-DCMAKE_TOOLCHAIN_FILE={}", &toolchain));
        }

        if target_triple.contains("linux") && !target_triple.contains("android") {
            if cfg!(feature = "no-pango") {
                cmd.push("-DOPTION_USE_PANGO=OFF".to_string());
            } else {
                cmd.push("-DOPTION_USE_PANGO=ON".to_string());
            }
        }

        if target_triple.contains("unknown-linux-musl") {
            cmd.push("-DCMAKE_C_COMPILER=musl-gcc".to_string());
            cmd.push("-DCMAKE_CXX_COMPILER=musl-gcc".to_string());
            cmd.push("-DHAVE_STRLCPY=False".to_string());
            cmd.push("-DHAVE_STRLCAT=False".to_string());
        }

        if cfg!(feature = "no-gdiplus") {
            cmd.push("-DOPTION_USE_GDIPLUS=OFF".to_string());
        }

        if cfg!(feature = "single-threaded") {
            cmd.push("-DCFLTK_SINGLE_THREADED=ON".to_string());
        } else {
            cmd.push("-DCFLTK_SINGLE_THREADED=OFF".to_string());
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

        cmd.push(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            out_dir.to_str().unwrap()
        ));
        cmd.push(format!("-DCMAKE_BUILD_TYPE={}", profile));
        cmd.push("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON".to_string());
        cmd.push("-DCFLTK_CARGO_BUILD=ON".to_string());
        cmd.push("-DFLTK_BUILD_EXAMPLES=OFF".to_string());
        cmd.push("-DFLTK_BUILD_TEST=OFF".to_string());
        cmd.push("-DOPTION_LARGE_FILE=ON".to_string());
        cmd.push("-DOPTION_USE_THREADS=ON".to_string());
        cmd.push("-DOPTION_BUILD_HTML_DOCUMENTATION=OFF".to_string());
        cmd.push("-DOPTION_BUILD_PDF_DOCUMENTATION=OFF".to_string());

        Command::new("cmake")
            .args(&cmd)
            .current_dir("cfltk")
            .status()
            .expect("CMake is needed for from-source builds!");

        Command::new("cmake")
            .args(&[
                "--build",
                &cmake_build_dir,
                if utils::cmake_has_parallel() {
                    "--parallel"
                } else {
                    ""
                },
                "--target",
                "install",
            ])
            .current_dir("cfltk")
            .status()
            .expect("CMake is needed for from-source builds!");
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
