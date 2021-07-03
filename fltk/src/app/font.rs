use crate::enums::Font;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::fl;
use std::{
    ffi::{CStr, CString},
    os::raw,
    path,
    sync::Mutex,
};

lazy_static! {
    /// The currently chosen font
    pub(crate) static ref CURRENT_FONT: Mutex<i32> = Mutex::new(0);

    /// Currently loaded fonts
    pub(crate) static ref LOADED_FONT: Option<&'static str> = None;

    /// The fonts associated with the application
    pub(crate) static ref FONTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// Set the app's font
pub fn set_font(new_font: Font) {
    unsafe {
        let new_font = new_font.bits() as i32;
        let mut f = CURRENT_FONT.lock().unwrap();
        fl::Fl_set_font(15, *f);
        fl::Fl_set_font(0, new_font);
        fl::Fl_set_font(new_font, *f);
        *f = new_font;
    }
}

/// Set the app's font size
pub fn set_font_size(sz: u8) {
    unsafe { fl::Fl_set_font_size(sz as i32) }
}

/// Get the font's name
pub fn get_font(font: Font) -> String {
    unsafe {
        CStr::from_ptr(fl::Fl_get_font(font.bits() as i32))
            .to_string_lossy()
            .to_string()
    }
}

/// Initializes loaded fonts of a certain pattern `name`
pub fn set_fonts(name: &str) -> u8 {
    let name = CString::safe_new(name);
    unsafe { fl::Fl_set_fonts(name.as_ptr() as *mut raw::c_char) as u8 }
}

/// Gets the name of a font through its index
pub fn font_name(idx: usize) -> Option<String> {
    let f = FONTS.lock().unwrap();
    Some(f[idx].clone())
}

/// Returns a list of available fonts to the application
pub fn get_font_names() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let cnt = set_fonts("*") as usize;
    for i in 0..cnt {
        let temp = unsafe {
            CStr::from_ptr(fl::Fl_get_font(i as i32))
                .to_string_lossy()
                .to_string()
        };
        vec.push(temp);
    }
    vec
}

/// Finds the index of a font through its name
pub fn font_index(name: &str) -> Option<usize> {
    let f = FONTS.lock().unwrap();
    f.iter().position(|i| i == name)
}

/// Gets the number of loaded fonts
pub fn font_count() -> usize {
    (*FONTS.lock().unwrap()).len()
}

/// Gets a Vector<String> of loaded fonts
pub fn fonts() -> Vec<String> {
    (*FONTS.lock().unwrap()).clone()
}

/// Load a font from a file
pub(crate) fn load_font(path: &str) -> Result<String, FltkError> {
    unsafe {
        let path = CString::new(path)?;
        if let Some(load_font) = *LOADED_FONT {
            unload_font(load_font)?;
        }
        let ptr = fl::Fl_load_font(path.as_ptr());
        if ptr.is_null() {
            Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let name = CStr::from_ptr(ptr as *mut _).to_string_lossy().to_string();
            let mut f = FONTS.lock().unwrap();
            if f.len() < 17 {
                f.push(name.clone());
            } else {
                f[16] = name.clone();
            }
            fl::Fl_set_font2(16, ptr);
            Ok(name)
        }
    }
}

/// Unload a loaded font
pub(crate) fn unload_font(path: &str) -> Result<(), FltkError> {
    unsafe {
        let check = path::Path::new(path);
        if !check.exists() {
            return Err::<(), FltkError>(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        let path = CString::new(path)?;
        fl::Fl_unload_font(path.as_ptr());
        Ok(())
    }
}
