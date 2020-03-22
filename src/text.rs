pub use crate::prelude::*;
use fltk_sys::text::*;
use std::{ffi::CString, mem, os::raw};

#[derive(WidgetTrait, Debug, Clone)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct TextEditor {
    _inner: *mut Fl_Text_Editor,
}

impl TextEditor {
    pub fn init(&mut self) {
        unsafe { Fl_Text_Editor_init(self._inner) }
    }
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Editor_set_text(self._inner, txt.into_raw() as *const raw::c_char)
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            CString::from_raw(Fl_Text_Editor_text(self._inner) as *mut raw::c_char).into_string().unwrap()
        }
    }
}

impl TextDisplay {
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Display_set_text(self._inner, txt.into_raw() as *const raw::c_char)
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            CString::from_raw(Fl_Text_Display_text(self._inner) as *mut raw::c_char)
                .into_string()
                .unwrap()
        }
    }
    pub fn init(&mut self) {
        unsafe { Fl_Text_Display_init(self._inner) }
    }
}
