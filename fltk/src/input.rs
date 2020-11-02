use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::input::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates an input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct Input {
    _inner: *mut Fl_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Sets the input widget's type, which can be changed dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum InputType {
    Normal = 0,
    Float = 1,
    Int = 2,
    Multiline = 4,
    Secret = 5,
    Input = 7,
    Readonly = 8,
    Wrap = 16,
}

/// Creates an input widget which takes only integers
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct IntInput {
    _inner: *mut Fl_Int_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an input widget which takes only floats
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct FloatInput {
    _inner: *mut Fl_Float_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a multiline-input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct MultilineInput {
    _inner: *mut Fl_Multiline_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a File-input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct FileInput {
    _inner: *mut Fl_File_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a secret input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct SecretInput {
    _inner: *mut Fl_Secret_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
