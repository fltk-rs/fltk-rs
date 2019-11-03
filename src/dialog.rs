pub use crate::prelude::*;
use fltk_sys::dialog::*;
use std::{ffi, mem, os::raw, ptr};

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
            ffi::CString::from_raw(Fl_Native_File_Chooser_filename(self._inner) as *mut raw::c_char).into_string().unwrap()
        }
    }
    pub fn directory(&self) -> String {
        unsafe {
            ffi::CString::from_raw(Fl_Native_File_Chooser_directory(self._inner) as *mut raw::c_char).into_string().unwrap()
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
}
