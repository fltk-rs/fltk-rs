pub use crate::prelude::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, Debug, Clone)]
pub struct Button {
    _inner: *mut fltk_sys::button::Fl_Button,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum ButtonType {
    NormalButton = 0,
    ToggleButton = 1,
    RadioButton = 102,
    HiddenButton = 3,
}

impl Button {
    pub fn as_ptr(&self) -> *mut fltk_sys::button::Fl_Button {
        self._inner
    }
}
