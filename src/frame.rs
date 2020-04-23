use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::frame::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a new frame, an equivalent of Fl_Box
#[derive(WidgetExt, Debug)]
pub struct Frame {
    _inner: *mut Fl_Box,
}
