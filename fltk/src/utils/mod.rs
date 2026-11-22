use fltk_sys::utils::*;
use std::ffi::{CStr, CString};
use std::os::raw;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::enums::ColorDepth;
use crate::prelude::FltkError;
use crate::prelude::FltkErrorKind;
use crate::prelude::ImageExt;

#[doc(hidden)]
/// A helper trait to get CStrings from Strings without panicking
pub trait FlString {
    /// Get `CStrings` from Strings without panicking
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

/// Open a uri using the system's browser
pub fn open_uri(s: &str) -> Result<String, FltkError> {
    let s = CString::safe_new(s);
    let mut v: Vec<u8> = vec![0u8; 255];
    unsafe {
        let ret = Fl_open_uri(s.as_ptr(), v.as_mut_ptr() as _, 255);
        let v: Vec<u8> = v.into_iter().partition(|x| *x == 0).1;
        let s = String::from_utf8(v)?;
        if ret == 1 {
            Ok(s)
        } else {
            Err(FltkError::Unknown(s))
        }
    }
}

/// Decode a uri
pub fn decode_uri(s: &str) -> String {
    let mut s = s.as_bytes().to_vec();
    s.push(0);
    unsafe {
        Fl_decode_uri(s.as_mut_ptr() as _);
        CStr::from_ptr(s.as_ptr() as _)
            .to_string_lossy()
            .to_string()
    }
}

/// Get the length of a char in terms of C strings
pub fn char_len(c: char) -> usize {
    unsafe extern "C" {
        pub fn strlen(s: *const std::os::raw::c_char) -> usize;
    }
    let s = CString::new(c.to_string()).unwrap();
    unsafe { strlen(s.as_ptr() as _) }
}

#[cfg(target_os = "macos")]
/// Get a window's content view
pub fn content_view<W: crate::prelude::WindowExt>(w: &W) -> *const raw::c_void {
    unsafe extern "C" {
        pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
    }
    unsafe { cfltk_getContentView(w.raw_handle() as _) as _ }
}

/// Check whether a widget is of a certain type
pub fn is<W: crate::prelude::WidgetBase>(w: &W) -> bool {
    W::from_dyn_widget(w).is_some()
}

/// Check whether a widget is of a certain type
pub fn is_ptr_of<W: crate::prelude::WidgetBase>(w: *mut fltk_sys::widget::Fl_Widget) -> bool {
    W::from_dyn_widget_ptr(w).is_some()
}

/// Get the dynamic typeinfo of a widget, useful for debugging.
/// This prints the mangled name, which can be unmangled using a crate like `cpp_demangle`
pub fn type_name<W: crate::prelude::WidgetExt>(w: &W) -> String {
    unsafe {
        let p = Fl_type_name(w.as_widget_ptr() as _);
        CStr::from_ptr(p as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

#[cfg(target_os = "emscripten")]
unsafe extern "C" {
    fn fl_read_to_string(empath: *const raw::c_char) -> *mut raw::c_char;
    fn fl_read_to_binary(empath: *const raw::c_char, len: *mut i32) -> *mut u8;
    fn fl_write_to_file(empath: *const raw::c_char, data: *const u8, len: i32) -> i32;
    fn free(data: *mut raw::c_void);
}

/// Read a web text file (chosen from the browser's file picker) to a string
#[cfg(target_os = "emscripten")]
pub fn em_file_read_to_string<S: AsRef<str>>(path: S) -> Result<String, FltkError> {
    unsafe {
        let path = CString::safe_new(path.as_ref());
        let ptr = fl_read_to_string(path.as_ptr());
        if ptr.is_null() {
            Err(FltkError::Unknown(String::from("Failed to read from file")))
        } else {
            let s = CStr::from_ptr(ptr).to_string_lossy().to_string();
            free(ptr as _);
            Ok(s)
        }
    }
}

/// Read a web binary file (chosen from the browser's file picker) to a Vec
#[cfg(target_os = "emscripten")]
pub fn em_file_read_to_binary(path: &str) -> Result<Vec<u8>, FltkError> {
    unsafe {
        let path = CString::safe_new(path);
        let mut len = 0;
        let ptr = fl_read_to_binary(path.as_ptr(), &mut len as _);
        if ptr.is_null() {
            Err(FltkError::Unknown(String::from("Failed to read from file")))
        } else {
            Ok(std::slice::from_raw_parts(ptr, len as _).to_vec())
        }
    }
}

/// Write to a file chosen by the browser's file picker
#[cfg(target_os = "emscripten")]
pub fn em_write_to_file(path: &str, data: &[u8]) -> Result<(), FltkError> {
    unsafe {
        let path = CString::safe_new(path);
        let ret = fl_write_to_file(path.as_ptr(), data.as_ptr(), data.len() as _);
        if ret == -1 {
            Err(FltkError::Unknown(String::from("Failed to write to file")))
        } else {
            Ok(())
        }
    }
}

/// Draw a framebuffer (rgba) into a widget
/// # Errors
/// Errors on invalid or unsupported image formats
pub fn blit_rgba<'a, T: crate::prelude::WidgetBase>(
    wid: &'a mut T,
    fb: &'a [u8],
) -> Result<(), FltkError> {
    let width = wid.w();
    let height = wid.h();
    let mut img = crate::image::RgbImage::new(fb, width, height, ColorDepth::Rgba8)?;
    wid.draw(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.w();
        let h = s.h();
        img.scale(w, h, false, true);
        img.draw(x, y, w, h);
    });
    Ok(())
}

/// Draw a framebuffer (rgba) into a widget
/// # Safety
/// The data passed should be valid and outlive the widget
pub unsafe fn blit_rgba_nocopy<T: crate::prelude::WidgetBase>(wid: &mut T, fb: &[u8]) {
    unsafe {
        let ptr = fb.as_ptr();
        let len = fb.len();
        let width = wid.w();
        let height = wid.h();
        wid.draw(move |s| {
            let x = s.x();
            let y = s.y();
            let w = s.w();
            let h = s.h();
            if let Ok(mut img) = crate::image::RgbImage::from_data(
                std::slice::from_raw_parts(ptr, len),
                width,
                height,
                ColorDepth::Rgba8,
            ) {
                img.scale(w, h, false, true);
                img.draw(x, y, w, h);
            }
        });
    }
}

/// Draw a framebuffer (rgba) into a widget
/// # Errors
/// Errors on invalid or unsupported image formats
pub fn blit_rgb<'a, T: crate::prelude::WidgetBase>(
    wid: &'a mut T,
    fb: &'a [u8],
) -> Result<(), FltkError> {
    let width = wid.w();
    let height = wid.h();
    let mut img = crate::image::RgbImage::new(fb, width, height, ColorDepth::Rgb8)?;
    wid.draw(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.w();
        let h = s.h();
        img.scale(w, h, false, true);
        img.draw(x, y, w, h);
    });
    Ok(())
}

/// Draw a framebuffer (rgba) into a widget
/// # Safety
/// The data passed should be valid and outlive the widget
pub unsafe fn blit_rgb_nocopy<T: crate::prelude::WidgetBase>(wid: &mut T, fb: &[u8]) {
    unsafe {
        let ptr = fb.as_ptr();
        let len = fb.len();
        let width = wid.w();
        let height = wid.h();
        wid.draw(move |s| {
            let x = s.x();
            let y = s.y();
            let w = s.w();
            let h = s.h();
            if let Ok(mut img) = crate::image::RgbImage::from_data(
                std::slice::from_raw_parts(ptr, len),
                width,
                height,
                ColorDepth::Rgb8,
            ) {
                img.scale(w, h, false, true);
                img.draw(x, y, w, h);
            }
        });
    }
}

/// Basically a check for image support
#[allow(dead_code)]
pub(crate) static IMAGES_REGISTERED: AtomicBool = AtomicBool::new(false);

/// Check if fltk-rs was initialized
#[allow(dead_code)]
pub(crate) fn images_registered() -> bool {
    IMAGES_REGISTERED.load(Ordering::Relaxed)
}

/// Registers all images supported by `SharedImage`
#[allow(dead_code)]
pub(crate) fn register_images() {
    #[cfg(feature = "use-images")]
    unsafe {
        fltk_sys::image::Fl_register_images();
        fltk_sys::fl::Fl_load_system_icons();
    }
}

/// Reads the full name (name ID 4) of the first face in TrueType/OpenType font data.
/// Only the `name` table is parsed, every read is bounds-checked.
pub(crate) fn font_full_name(data: &[u8]) -> Option<String> {
    fn u16_at(d: &[u8], off: usize) -> Option<usize> {
        let b = d.get(off..off.checked_add(2)?)?;
        Some(u16::from_be_bytes([b[0], b[1]]) as usize)
    }
    fn u32_at(d: &[u8], off: usize) -> Option<usize> {
        let b = d.get(off..off.checked_add(4)?)?;
        usize::try_from(u32::from_be_bytes([b[0], b[1], b[2], b[3]])).ok()
    }
    const FULL_NAME: usize = 4;
    const PLATFORM_UNICODE: usize = 0;
    const PLATFORM_WINDOWS: usize = 3;

    // A collection starts with a list of face offsets, take the first face
    let mut face = 0;
    if data.get(..4)? == b"ttcf" {
        if u32_at(data, 8)? == 0 {
            return None;
        }
        face = u32_at(data, 12)?;
    }
    match data.get(face..face.checked_add(4)?)? {
        [0, 1, 0, 0] | b"true" | b"OTTO" => (),
        _ => return None,
    }

    // Table directory: 12 byte header, then 16 byte records (tag, checksum, offset, length)
    let num_tables = u16_at(data, face.checked_add(4)?)?;
    let name = (0..num_tables).find_map(|i| {
        let rec = face.checked_add(12 + i * 16)?;
        if data.get(rec..rec.checked_add(4)?)? == b"name" {
            let off = u32_at(data, rec + 8)?;
            let len = u32_at(data, rec + 12)?;
            data.get(off..off.checked_add(len)?)
        } else {
            None
        }
    })?;

    // Name table: version, count, storage offset, then 12 byte records
    // (platform, encoding, language, name id, length, offset into storage)
    let count = u16_at(name, 2)?;
    let storage = name.get(u16_at(name, 4)?..)?;
    (0..count).find_map(|i| {
        let rec = 6 + i * 12;
        let platform = u16_at(name, rec)?;
        let encoding = u16_at(name, rec + 2)?;
        // Both of these store the string as UTF-16BE
        let is_unicode =
            platform == PLATFORM_UNICODE || (platform == PLATFORM_WINDOWS && encoding <= 1);
        if u16_at(name, rec + 6)? != FULL_NAME || !is_unicode {
            return None;
        }
        let len = u16_at(name, rec + 8)?;
        let off = u16_at(name, rec + 10)?;
        let units: Vec<u16> = storage
            .get(off..off + len)?
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16(&units).ok()
    })
}
