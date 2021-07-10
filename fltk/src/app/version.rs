use fltk_sys::fl;

/// Gets FLTK version
pub fn version() -> f64 {
    unsafe { fl::Fl_version() }
}

/// Gets FLTK API version
pub fn api_version() -> i32 {
    unsafe { fl::Fl_api_version() }
}

/// Gets FLTK ABI version
pub fn abi_version() -> i32 {
    unsafe { fl::Fl_abi_version() }
}

/// Gets FLTK crate version
pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
