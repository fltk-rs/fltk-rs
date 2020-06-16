pub use fltk_sys::glu::*;

pub fn gl_start() {
    unsafe { cgl_start(); }
}

pub fn gl_finish() {
    unsafe { cgl_finish(); }
}