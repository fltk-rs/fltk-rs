use std::path::PathBuf;

pub fn link(target_os: String, out_dir: PathBuf) {
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
            match target_os.as_str() {
                "macos" => println!("cargo:rustc-link-lib=framework=OpenGL"),
                "windows" => {
                    println!("cargo:rustc-link-lib=dylib=opengl32");
                    println!("cargo:rustc-link-lib=dylib=glu32");
                }
                _ => {
                    println!("cargo:rustc-link-lib=dylib=GL");
                    println!("cargo:rustc-link-lib=dylib=GLU");
                }
            }
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
