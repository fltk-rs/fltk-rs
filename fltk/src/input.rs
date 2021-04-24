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

/// Creates an input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct Input {
    inner: *mut Fl_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Sets the input widget's type
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum InputType {
    /// Normal input
    Normal = 0,
    /// Float input
    Float = 1,
    /// Int input
    Int = 2,
    /// Multiline input
    Multiline = 4,
    /// Secret input
    Secret = 5,
    /// Input!
    Input = 7,
    /// Readonly input
    Readonly = 8,
    /// Wrap input
    Wrap = 16,
}

/// Creates an input widget which takes only integers
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct IntInput {
    inner: *mut Fl_Int_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an input widget which takes only floats
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct FloatInput {
    inner: *mut Fl_Float_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a multiline-input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct MultilineInput {
    inner: *mut Fl_Multiline_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a File-input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct FileInput {
    inner: *mut Fl_File_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl FileInput {
    /// Set the `down_box` of the widget
    pub fn set_down_frame(&mut self, f: FrameType) {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Input_set_down_box(self.inner, f as i32) }
    }

    /// Get the `down_box` of the widget
    pub fn down_frame(&self) -> FrameType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_File_Input_down_box(self.inner)) }
    }
}

/// Creates a secret input widget
#[derive(WidgetBase, WidgetExt, InputExt, Debug)]
pub struct SecretInput {
    inner: *mut Fl_Secret_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
