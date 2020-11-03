use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::widget::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// An abstract type, shouldn't be instantiated in user code
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct Widget {
    _inner: *mut Fl_Widget,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}