pub use crate::prelude::*;
use fltk_sys::output::*;
use crate::image::Image;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates an output widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Output {
    _inner: *mut Fl_Output,
}

/// Creates a multiline-output widget
#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineOutput {
    _inner: *mut Fl_Multiline_Output,
}
