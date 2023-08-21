
const CPP_SRC: &[&str] = &[
    "cfltk/src/cfl_new.cpp",
    "cfltk/src/cfl_lock.cpp",
    "cfltk/src/cfl.cpp",
    "cfltk/src/cfl_window.cpp",
    "cfltk/src/cfl_button.cpp",
    "cfltk/src/cfl_widget.cpp",
    "cfltk/src/cfl_group.cpp",
    "cfltk/src/cfl_text.cpp",
    "cfltk/src/cfl_box.cpp",
    "cfltk/src/cfl_input.cpp",
    "cfltk/src/cfl_menu.cpp",
    "cfltk/src/cfl_dialog.cpp",
    "cfltk/src/cfl_valuator.cpp",
    "cfltk/src/cfl_browser.cpp",
    "cfltk/src/cfl_misc.cpp",
    "cfltk/src/cfl_image.cpp",
    "cfltk/src/cfl_draw.cpp",
    "cfltk/src/cfl_table.cpp",
    "cfltk/src/cfl_tree.cpp",
    "cfltk/src/cfl_surface.cpp",
    "cfltk/src/cfl_font.cpp",
    "cfltk/src/cfl_utils.cpp",
    "cfltk/src/cfl_printer.cpp",
];

pub fn compile() {
    let mut use_gl = false;
    if cfg!(feature = "enable-glwindow") {
        use_gl = true;
    }
    let mut b = cc::Build::new();
    b.cpp(true);
    b.files(CPP_SRC);
    if cfg!(target_os = "macos") {
        b.file("cfltk/src/cfl_nswindow.m");
    }
    if use_gl {
        b.define("CFLTK_USE_GL", None);
        b.file("cfltk/src/glad.c");
    }
    b.include("cfltk/include");
    b.flag_if_supported("-std=c++17");
    b.warnings(false);
    b.flag_if_supported("-w");
    b.compile("cfltk");
}