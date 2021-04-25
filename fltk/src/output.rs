use crate::enums::{Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::output::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates an output widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct Output {
    inner: *mut Fl_Output,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a multiline-output widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct MultilineOutput {
    inner: *mut Fl_Multiline_Output,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
