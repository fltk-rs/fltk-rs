pub(crate) mod oncelock;

use fltk_sys::utils::*;
use std::ffi::CString;
use std::os::raw;

use crate::prelude::FltkError;
use crate::prelude::FltkErrorKind;

#[doc(hidden)]
/// A helper trait to get CStrings from Strings without panicking
pub trait FlString {
    /// Get CStrings from Strings without panicking
    fn safe_new(s: &str) -> CString;
}

impl FlString for CString {
    fn safe_new(s: &str) -> CString {
        match CString::new(s) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        }
    }
}

/**
    Convenience function to convert rgb to hex.
    Example:
    ```rust,no_run
    use fltk::utils::rgb2hex;
    let ret = rgb2hex(0, 255, 0); println!("0x{:06x}", ret);
    ```
*/
pub const fn rgb2hex(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    ((r & 0xff) << 16) + ((g & 0xff) << 8) + (b & 0xff)
}

/**
    Convenience function to convert rgba to hex.
    Example:
    ```rust,no_run
    use fltk::utils::rgba2hex;
    let ret = rgba2hex(0, 255, 0, 255); println!("0x{:08x}", ret);
    ```
*/
pub const fn rgba2hex(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    let a = a as u32;
    ((r & 0xff) << 24) + ((g & 0xff) << 16) + ((b & 0xff) << 8) + (a & 0xff)
}

/**
    Convenience function to convert hex to rgb.
    Example:
    ```rust,no_run
    use fltk::utils::hex2rgb;
    let (r, g, b) = hex2rgb(0x000000);
    ```
*/
pub const fn hex2rgb(val: u32) -> (u8, u8, u8) {
    let r = ((val >> 16) & 0xff) as u8;
    let g = ((val >> 8) & 0xff) as u8;
    let b = (val & 0xff) as u8;
    (r, g, b)
}

/**
    Convenience function to convert hex to rgba.
    Example:
    ```rust,no_run
    use fltk::utils::hex2rgba;
    let (r, g, b, a) = hex2rgba(0xff0000ff);
    ```
*/
pub const fn hex2rgba(val: u32) -> (u8, u8, u8, u8) {
    let r = ((val >> 24) & 0xff) as u8;
    let g = ((val >> 16) & 0xff) as u8;
    let b = ((val >> 8) & 0xff) as u8;
    let a = (val & 0xff) as u8;
    (r, g, b, a)
}

/// Expand a filename
pub fn filename_expand(path: &str) -> Result<String, FltkError> {
    assert!(path.len() <= 2048);
    let mut out: Vec<u8> = vec![0u8; 2048];
    let path = CString::safe_new(path);
    unsafe {
        let ret = Fl_filename_expand(
            out.as_mut_ptr() as *mut raw::c_char,
            2048,
            path.as_ptr() as _,
        );
        if ret == 0 {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let val = out.iter().position(|&x| x == 0).unwrap();
            let out = out.split_at(val);
            match String::from_utf8(out.0.to_vec()) {
                Ok(s) => Ok(s),
                Err(err) => Err(FltkError::Utf8Error(err)),
            }
        }
    }
}

/// Get the length of a char in terms of C strings
pub fn char_len(c: char) -> usize {
    extern "C" {
        pub fn strlen(s: *const std::os::raw::c_char) -> usize;
    }
    let s = std::ffi::CString::new(c.to_string()).unwrap();
    unsafe { strlen(s.as_ptr() as _) }
}

#[cfg(target_os = "macos")]
/// Get a window's content view
pub fn content_view<W: crate::prelude::WindowExt>(w: &W) -> *const raw::c_void {
    extern "C" {
        pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
    }
    unsafe { cfltk_getContentView(w.raw_handle() as _) as _ }
}

/// Check whether a widget is of a certain type
pub fn is<W: crate::prelude::WidgetBase>(w: &impl crate::prelude::WidgetBase) -> bool {
    W::from_dyn_widget(w).is_some()
}

/// Check whether a widget is of a certain type
pub fn is_ptr_of<W: crate::prelude::WidgetBase>(w: *mut fltk_sys::widget::Fl_Widget) -> bool {
    W::from_dyn_widget_ptr(w).is_some()
}
