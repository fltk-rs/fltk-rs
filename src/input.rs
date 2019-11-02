pub use crate::prelude::*;
use fltk_sys::input::*;
use std::{ffi, mem, os::raw, ptr};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Input {
    _inner: *mut Fl_Input,
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
    Multiline = 4,
    Secret = 5,
    InputType = 7,
    Readonly = 8,
    Wrap = 16,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct IntInput {
    _inner: *mut Fl_Int_Input,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct FloatInput {
    _inner: *mut Fl_Float_Input,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineInput {
    _inner: *mut Fl_Multiline_Input,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}
