pub use crate::prelude::*;
use fltk_sys::group::*;
use std::{ffi::CString, mem, os::raw};

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Group {
    _inner: *mut Fl_Group,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Pack {
    _inner: *mut Fl_Pack,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Scroll {
    _inner: *mut Fl_Scroll,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tabs {
    _inner: *mut Fl_Tabs,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tile {
    _inner: *mut Fl_Tile,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
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
            Fl_Text_Display_set_text(self._inner, txt.as_ptr() as *const raw::c_char)
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
