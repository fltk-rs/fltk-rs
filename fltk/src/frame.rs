use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::frame::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a new frame, an equivalent of Fl_Box
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct Frame {
    inner: *mut Fl_Box,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
