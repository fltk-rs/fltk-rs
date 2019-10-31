pub use crate::prelude::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Output {
    _inner: *mut fltk_sys::output::Fl_Output,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum OutputType {
    NormalOutput = 0,
}

impl Output {
    pub fn as_ptr(&self) -> *mut fltk_sys::output::Fl_Output {
        self._inner
    }
}
