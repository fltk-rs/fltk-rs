pub use crate::prelude::*;
use fltk_sys::dialog::*;
// use std::{ffi, mem, os::raw, ptr};

#[derive(Debug, Clone)]
pub struct FileDialog {
    _inner: *mut Fl_Native_File_Chooser,
}

impl FileDialog {
    pub fn new(op: i32) -> FileDialog {
        FileDialog {
            _inner : unsafe {
                Fl_Native_File_Chooser_new(op)
            }
        }
    }
    pub fn show(&mut self) {
        unsafe {
            Fl_Native_File_Chooser_show(self._inner);
        }
    } 
}