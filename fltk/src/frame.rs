use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::frame::*;
use std::ffi::{CStr, CString};

/// Creates a new frame, an equivalent of `Fl_Box`
#[derive(Debug)]
pub struct Frame {
    inner: *mut Fl_Box,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_base!(Frame, Fl_Box);
crate::macros::widget::impl_widget_ext!(Frame, Fl_Box);
