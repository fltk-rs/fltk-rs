use crate::utils;
use std::process::Command;

pub fn link(target_os: &str, target_triple: &str) {
    if !cfg!(feature = "fltk-shared") {
        println!("cargo:rustc-link-lib=static=cfltk");
    } else {
        println!("cargo:rustc-link-lib=dylib=cfltk");
    }

    if cfg!(feature = "system-fltk")
        && (target_triple.contains("gnu") || target_triple.contains("darwin"))
    {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    if !cfg!(feature = "fltk-shared") {
        println!("cargo:rustc-link-lib=static=fltk");

        if cfg!(feature = "use-images") {
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
            match target_os {
                "macos" => println!("cargo:rustc-link-lib=framework=OpenGL"),
                "windows" => {
                    println!("cargo:rustc-link-lib=dylib=opengl32");
                    println!("cargo:rustc-link-lib=dylib=glu32");
                }
                _ => {
                    if !cfg!(feature = "no-wayland") {
                        println!("cargo:rustc-link-lib=dylib=wayland-egl");
                        println!("cargo:rustc-link-lib=dylib=EGL");
                    }
                    println!("cargo:rustc-link-lib=dylib=GL");
                    println!("cargo:rustc-link-lib=dylib=GLU");
                }
            }
        }

        match target_os {
            "macos" => {
                let darwin_version: i32 = utils::get_taget_darwin_major_version().unwrap();
                if darwin_version > 19 {
                    println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
                }
                if darwin_version > 23 {
                    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
                }
                println!("cargo:rustc-link-lib=framework=Carbon");
                println!("cargo:rustc-link-lib=framework=Cocoa");
                println!("cargo:rustc-link-lib=framework=ApplicationServices");
                println!("cargo:rustc-link-lib=c++");
            }
            "windows" => {
                let linkage = if crate::utils::use_static_msvcrt() {
                    "="
                } else {
                    "=dylib="
                };
                println!("cargo:rustc-link-lib{}ws2_32", linkage);
                println!("cargo:rustc-link-lib{}comctl32", linkage);
                println!("cargo:rustc-link-lib{}gdi32", linkage);
                println!("cargo:rustc-link-lib{}oleaut32", linkage);
                println!("cargo:rustc-link-lib{}ole32", linkage);
                println!("cargo:rustc-link-lib{}uuid", linkage);
                println!("cargo:rustc-link-lib{}shell32", linkage);
                println!("cargo:rustc-link-lib{}advapi32", linkage);
                println!("cargo:rustc-link-lib{}comdlg32", linkage);
                println!("cargo:rustc-link-lib{}winspool", linkage);
                println!("cargo:rustc-link-lib{}user32", linkage);
                println!("cargo:rustc-link-lib{}kernel32", linkage);
                println!("cargo:rustc-link-lib{}odbc32", linkage);
                if !cfg!(feature = "no-gdiplus") {
                    println!("cargo:rustc-link-lib{}gdiplus", linkage);
                }
                if target_triple.contains("gnu") {
                    println!("cargo:rustc-link-lib=stdc++");
                    println!("cargo:rustc-link-lib=gcc");
                }
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
            "emscripten" => {
                // Experimental
                println!("cargo:rustc-link-lib=embind");
            }
            _ => {
                println!("cargo:rustc-link-lib=dylib=pthread");
                let mut link_x11 = true;
                if !cfg!(feature = "no-wayland") {
                    println!("cargo:rustc-link-lib=dylib=wayland-client");
                    println!("cargo:rustc-link-lib=dylib=wayland-cursor");
                    println!("cargo:rustc-link-lib=dylib=xkbcommon");
                    println!("cargo:rustc-link-lib=dylib=dbus-1");
                    if cfg!(feature = "wayland-only") {
                        link_x11 = false;
                    }
                    if cfg!(feature = "gtk-decor") {
                        allow_gtk_plugin();
                    }
                }
                if link_x11 {
                    println!("cargo:rustc-link-lib=dylib=X11");
                    println!("cargo:rustc-link-lib=dylib=Xext");
                    println!("cargo:rustc-link-lib=dylib=Xinerama");
                    println!("cargo:rustc-link-lib=dylib=Xcursor");
                    println!("cargo:rustc-link-lib=dylib=Xrender");
                    println!("cargo:rustc-link-lib=dylib=Xfixes");
                    println!("cargo:rustc-link-lib=dylib=Xft");
                }
                println!("cargo:rustc-link-lib=dylib=fontconfig");
                if !cfg!(feature = "no-pango-cairo") {
                    println!("cargo:rustc-link-lib=dylib=pango-1.0");
                    println!("cargo:rustc-link-lib=dylib=pangoxft-1.0");
                    println!("cargo:rustc-link-lib=dylib=gobject-2.0");
                    println!("cargo:rustc-link-lib=dylib=cairo");
                    println!("cargo:rustc-link-lib=dylib=pangocairo-1.0");
                }
                if target_triple.contains("gnu") || target_triple.contains("musl") {
                    println!("cargo:rustc-link-lib=stdc++");
                } else {
                    println!("cargo:rustc-link-lib=c++");
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn allow_gtk_plugin() {
    if let Ok(lflags) = Command::new("pkg-config")
        .args(["--libs", "gtk+-3.0"])
        .output()
    {
        let lflags = String::from_utf8_lossy(&lflags.stdout).to_string();
        let lflags: Vec<&str> = lflags.split_ascii_whitespace().collect();
        for flag in lflags {
            println!(
                "cargo:rustc-link-lib=dylib={}",
                flag.strip_prefix("-l").unwrap()
            );
        }
    }
}
