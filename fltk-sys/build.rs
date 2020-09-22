use std::{
    env,
    path::PathBuf,
    process::Command,
};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
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
            pkg_version,
            platform
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
        println!("cargo:rerun-if-changed=cfltk/include/cfl_new.hpp");
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
        println!("cargo:rerun-if-changed=cfltk/CMakeLists.txt");

        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .current_dir(manifest_dir.clone())
            .status()
            .expect("Git is needed to retrieve the fltk source files!");

        Command::new("git")	
            .args(&["checkout", "master"])	
            .current_dir(manifest_dir.join("cfltk").join("fltk"))	
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

        if cfg!(feature = "cpp-testing") {
            println!("cargo:rerun-if-changed=cfltk/tests/test1.cpp");
            dst.define("CFLTK_BUILD_TESTS", "ON");
        }

        if let Ok(toolchain) = env::var("FLTK_TOOLCHAIN_FILE") {
            dst.define("CMAKE_TOOLCHAIN_FILE", toolchain);
        }

        let _dst = dst
            .profile("Release")
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("OPTION_ABI_VERSION:STRING", "10401")
            .define("FLTK_BUILD_EXAMPLES", "OFF")
            .define("FLTK_BUILD_TEST", "OFF")
            .define("FLTK_BUILD_FLUID","OFF")
            .define("OPTION_USE_THREADS", "ON")
            .define("OPTION_LARGE_FILE", "ON")
            .define("OPTION_BUILD_HTML_DOCUMENTATION", "OFF")
            .define("OPTION_BUILD_PDF_DOCUMENTATION", "OFF")
            .build();
    }

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").join("Release").display()
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
                // Experimental, not tried yet!
            }
            "ios" => {
                // Also experimental
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
            }
        }
    }
}
