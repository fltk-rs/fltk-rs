pub use crate::prelude::*;
use fltk_sys::group::*;
use std::{ffi, mem, os::raw, ptr};

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Group {
    _inner: *mut Fl_Group,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Pack {
    _inner: *mut Fl_Pack,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Scroll {
    _inner: *mut Fl_Scroll,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tabs {
    _inner: *mut Fl_Tabs,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tile {
    _inner: *mut Fl_Tile,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct TextEditor {
    _inner: *mut Fl_Text_Editor,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl TextEditor {
    pub fn init(&mut self) {
        unsafe { Fl_Text_Editor_init(self._inner) }
    }
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = ffi::CString::new(txt).unwrap();
            Fl_Text_Editor_set_text(self._inner, txt.as_ptr() as *const raw::c_char)
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            String::from(ffi::CStr::from_ptr(Fl_Text_Editor_text(self._inner)).to_string_lossy())
        }
    }
}

impl TextDisplay {
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = ffi::CString::new(txt).unwrap();
            Fl_Text_Display_set_text(self._inner, txt.as_ptr() as *const raw::c_char)
        }
    }
    pub fn text(&self) -> String {
        unsafe {
            String::from(ffi::CStr::from_ptr(Fl_Text_Display_text(self._inner)).to_string_lossy())
        }
    }
    pub fn init(&mut self) {
        unsafe { Fl_Text_Display_init(self._inner) }
    }
}
