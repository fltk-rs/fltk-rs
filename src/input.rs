pub use crate::prelude::*;
use fltk_sys::input::*;
use std::{ffi::CString, mem, os::raw};

/// Creates an input widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Input {
    _inner: *mut Fl_Input,
}

/// Sets the input widget's type, which can be changed dynamically using the set_type() method
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

/// An input widget which takes only integers
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct IntInput {
    _inner: *mut Fl_Int_Input,
}

/// An input widget which takes only floats
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct FloatInput {
    _inner: *mut Fl_Float_Input,
}

/// An multiline-input widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineInput {
    _inner: *mut Fl_Multiline_Input,
}

/// An File-input widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct FileInput {
    _inner: *mut Fl_File_Input,
}

/// An secret input widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct SecretInput {
    _inner: *mut Fl_Secret_Input,
}
