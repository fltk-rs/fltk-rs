use fltk_sys::fl;

/// Gets FLTK version
pub fn version() -> f64 {
    unsafe { fl::Fl_version() }
}

/// Gets FLTK version string
pub fn version_str() -> String {
    let v = api_version().to_string();
    let start = v.len() - 4;
    // should never panic!
    format!(
        "{}.{}.{}",
        &v[0..start].parse::<i32>().unwrap(),
        &v[start..start + 2].parse::<i32>().unwrap(),
        &v[start + 2..].parse::<i32>().unwrap()
    )
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
