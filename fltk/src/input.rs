use crate::enums::FrameType;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::input::*;
use std::{
    ffi::{CStr, CString},
    mem,
};

/// Creates an input widget
#[derive(Debug)]
pub struct Input {
    inner: *mut Fl_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Input, Fl_Input);
crate::macros::widget::impl_widget_base!(Input, Fl_Input);
crate::macros::input::impl_input_ext!(Input, Fl_Input);

/// Sets the input widget's type
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

crate::macros::widget::impl_widget_type!(InputType);

/// Creates an input widget which takes only integers
#[derive(Debug)]
pub struct IntInput {
    inner: *mut Fl_Int_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(IntInput, Fl_Int_Input);
crate::macros::widget::impl_widget_base!(IntInput, Fl_Int_Input);
crate::macros::input::impl_input_ext!(IntInput, Fl_Int_Input);

/// Creates an input widget which takes only floats
#[derive(Debug)]
pub struct FloatInput {
    inner: *mut Fl_Float_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(FloatInput, Fl_Float_Input);
crate::macros::widget::impl_widget_base!(FloatInput, Fl_Float_Input);
crate::macros::input::impl_input_ext!(FloatInput, Fl_Float_Input);

/// Creates a multiline-input widget
#[derive(Debug)]
pub struct MultilineInput {
    inner: *mut Fl_Multiline_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MultilineInput, Fl_Multiline_Input);
crate::macros::widget::impl_widget_base!(MultilineInput, Fl_Multiline_Input);
crate::macros::input::impl_input_ext!(MultilineInput, Fl_Multiline_Input);

/// Creates a File-input widget
#[derive(Debug)]
pub struct FileInput {
    inner: *mut Fl_File_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(FileInput, Fl_File_Input);
crate::macros::widget::impl_widget_base!(FileInput, Fl_File_Input);
crate::macros::input::impl_input_ext!(FileInput, Fl_File_Input);

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
#[derive(Debug)]
pub struct SecretInput {
    inner: *mut Fl_Secret_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SecretInput, Fl_Secret_Input);
crate::macros::widget::impl_widget_base!(SecretInput, Fl_Secret_Input);
crate::macros::input::impl_input_ext!(SecretInput, Fl_Secret_Input);
