pub use crate::prelude::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Input {
    _inner: *mut fltk_sys::input::Fl_Input,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum InputType {
    NormalInput = 0,
    FloatInput = 1,
    IntInput = 2,
    HiddenInput = 3,
    Multiline = 4,
    Secret = 5,
    InputType = 7,
    Readonly = 8,
    Wrap = 16,
}

impl Input {
    pub fn as_ptr(&self) -> *mut fltk_sys::input::Fl_Input {
        self._inner
    }
}
