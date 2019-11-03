pub use crate::prelude::*;
use fltk_sys::dialog::*;
use std::{ffi, mem, os::raw};

#[derive(Debug, Clone)]
pub struct FileDialog {
    _inner: *mut Fl_Native_File_Chooser,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum FileDialogType {
    BrowseFile = 0,
    BrowseDir,
    BrowseMultiFile,
    BrowseMultiDir,
    BrowseSaveFile,
    BrowseSaveDir,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum FileDialogOptions {
    NoOptions = 0,
    SaveAsConfirm = 1,
    NewFolder = 2,
    Preview = 4,
    UseFilterExt = 8,
}

impl FileDialog {
    pub fn new(op: FileDialogType) -> FileDialog {
        FileDialog {
            _inner: unsafe { Fl_Native_File_Chooser_new(mem::transmute(op)) },
        }
    }
    pub fn filename(&self) -> String {
        unsafe {
            String::from(ffi::CStr::from_ptr(Fl_Native_File_Chooser_filename(self._inner)).to_string_lossy())
        }
    }
    pub fn directory(&self) -> String {
        unsafe {
            String::from(ffi::CStr::from_ptr(Fl_Native_File_Chooser_directory(self._inner)).to_string_lossy())
        }
    }
    pub fn set_directory(&mut self, dir: &str) {
        unsafe {
            Fl_Native_File_Chooser_set_directory(self._inner, ffi::CString::new(dir).unwrap().as_ptr())
        }
    }
    pub fn show(&mut self) {
        unsafe {
            Fl_Native_File_Chooser_show(self._inner);
        }
    }
    pub fn set_option(&mut self, opt: FileDialogOptions) {
        unsafe {
            Fl_Native_File_Chooser_set_option(self._inner, opt as i32)
        }
    }
    pub fn set_type(&mut self, op: FileDialogType) {
        unsafe {
            Fl_Native_File_Chooser_set_type(self._inner, op as i32)
        }
    }
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            Fl_Native_File_Chooser_set_title(self._inner, title.as_ptr() as *const raw::c_char)
        }
    }
    pub fn set_filter(&mut self, f: &str) {
        unsafe {
            Fl_Native_File_Chooser_set_filter(self._inner, f.as_ptr() as *const raw::c_char)
        }
    }
    pub fn set_preset_file(&mut self, f: &str) {
        unsafe {
            Fl_Native_File_Chooser_set_preset_file(self._inner, f.as_ptr() as *const raw::c_char)
        }
    }
    pub fn errmsg(&self) -> String {
        unsafe {
            String::from(ffi::CStr::from_ptr(Fl_Native_File_Chooser_errmsg(self._inner)).to_string_lossy())
        }
    }
}

pub fn message(txt: &str) {
    unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        cfl_message(txt.as_ptr() as *const raw::c_char)
    }
}

pub fn alert(txt: &str) {
    unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        cfl_alert(txt.as_ptr() as *const raw::c_char)
    }
}

pub fn choice(
        txt: &str,
        b0: &str,
        b1: &str,
        b2: &str,
    ) -> usize {
        unsafe {
            let txt = ffi::CString::new(txt).unwrap();
            let b0 = ffi::CString::new(b0).unwrap();
            let b1 = ffi::CString::new(b1).unwrap();
            let b2 = ffi::CString::new(b2).unwrap();
            cfl_choice(txt.as_ptr() as *const raw::c_char, b0.as_ptr() as *const raw::c_char, b1.as_ptr() as *const raw::c_char, b2.as_ptr() as *const raw::c_char) as usize
        }
    }

pub fn input(
        txt: &str,
        deflt: &str,
    ) -> String {
        unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        let deflt = ffi::CString::new(deflt).unwrap();
        String::from(ffi::CStr::from_ptr(cfl_input(txt.as_ptr() as *const raw::c_char, deflt.as_ptr() as *const raw::c_char)).to_string_lossy())
        }
    }

pub fn password(
        txt: &str,
        deflt: &str,
    ) -> String {
        unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        let deflt = ffi::CString::new(deflt).unwrap();
            String::from(ffi::CStr::from_ptr(cfl_password(txt.as_ptr() as *const raw::c_char, deflt.as_ptr() as *const raw::c_char)).to_string_lossy())
        }
    }


