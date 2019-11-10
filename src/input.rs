pub use crate::prelude::*;
use fltk_sys::input::*;
use std::{ffi, mem, os::raw};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Input {
    _inner: *mut Fl_Input,
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
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct FloatInput {
    _inner: *mut Fl_Float_Input,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineInput {
    _inner: *mut Fl_Multiline_Input,
}
