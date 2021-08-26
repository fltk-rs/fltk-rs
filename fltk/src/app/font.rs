use crate::app::init::{CURRENT_FONT, FONTS, LOADED_FONT};
use crate::enums::Font;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::fl;
use std::{
    ffi::{CStr, CString},
    os::raw,
    path,
    sync::atomic::Ordering,
};

/// Set the app's font
pub fn set_font(new_font: Font) {
    unsafe {
        let new_font = new_font.bits() as i32;
        let f = CURRENT_FONT.load(Ordering::Relaxed);
        fl::Fl_set_font(15, f);
        fl::Fl_set_font(0, new_font);
        fl::Fl_set_font(new_font, f);
        CURRENT_FONT.store(new_font, Ordering::Relaxed);
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

/// Get the font's name
pub fn get_font_name(font: Font) -> String {
    unsafe {
        CStr::from_ptr(fl::Fl_get_font_name(font.bits() as i32))
            .to_string_lossy()
            .to_string()
    }
}

/// Get a font's sizes
pub fn get_font_sizes(font: Font) -> Vec<i32> {
    unsafe {
        let start = vec![0i32; 128];
        let mut start = std::mem::ManuallyDrop::new(start);
        let size = fl::Fl_get_font_sizes(font.bits(), &mut start.as_mut_ptr()) as usize;
        Vec::from_raw_parts(start.as_mut_ptr(), size, 128)
    }
}

/// Initializes loaded fonts of a certain pattern `name`
pub fn set_fonts(name: &str) -> u8 {
    let name = CString::safe_new(name);
    unsafe { fl::Fl_set_fonts(name.as_ptr() as *mut raw::c_char) as u8 }
}

/// Gets the name of a font through its index
pub fn font_name(idx: usize) -> Option<String> {
    if let Some(f) = unsafe { &FONTS } {
        let f = f.lock().unwrap();
        Some(f[idx].clone())
    } else {
        None
    }
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
    if let Some(f) = unsafe { &FONTS } {
        let f = f.lock().unwrap();
        f.iter().position(|i| i == name)
    } else {
        None
    }
}

/// Gets the number of loaded fonts
pub fn font_count() -> usize {
    if let Some(f) = unsafe { &FONTS } {
        let f = f.lock().unwrap();
        f.len()
    } else {
        0
    }
}

/// Gets a Vector<String> of loaded fonts
pub fn fonts() -> Vec<String> {
    if let Some(f) = unsafe { &FONTS } {
        (f.lock().unwrap()).clone()
    } else {
        vec![]
    }
}

/// Load a font from a file
pub(crate) fn load_font(path: &str) -> Result<String, FltkError> {
    unsafe {
        let path = CString::new(path)?;
        if let Some(load_font) = LOADED_FONT {
            unload_font(load_font)?;
        }
        let ptr = fl::Fl_load_font(path.as_ptr());
        if ptr.is_null() {
            Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let name = CStr::from_ptr(ptr as *mut _).to_string_lossy().to_string();
            if let Some(f) = &FONTS {
                let mut f = f.lock().unwrap();
                if f.len() < 17 {
                    f.push(name.clone());
                } else {
                    f[16] = name.clone();
                }
                fl::Fl_set_font2(16, ptr);
                Ok(name)
            } else {
                Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
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
