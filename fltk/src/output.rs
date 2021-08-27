use crate::enums::{Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::input::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Sets the input widget's type
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum OutputType {
    /// Normal input
    Normal = 8,
    /// Multiline input
    Multiline = 12,
}

/// Creates an output widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct Output {
    inner: *mut Fl_Output,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

/// Creates a multiline-output widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct MultilineOutput {
    inner: *mut Fl_Multiline_Output,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}
