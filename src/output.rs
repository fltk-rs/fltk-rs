pub use crate::prelude::*;
use fltk_sys::output::*;
use std::{ffi::CString, mem, os::raw};

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct Output {
    _inner: *mut Fl_Output,
}

#[derive(WidgetTrait, InputTrait, Debug, Clone)]
pub struct MultilineOutput {
    _inner: *mut Fl_Multiline_Output,
}
