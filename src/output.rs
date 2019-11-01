pub use crate::prelude::*;
use fltk_sys::output::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Output {
    _inner: *mut Fl_Output,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineOutput {
    _inner: *mut Fl_Multiline_Output,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}
