use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::input::*;
use std::ffi::{CStr, CString};

/// Sets the input widget's type
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OutputType {
    /// Normal input
    Normal = 8,
    /// Multiline input
    Multiline = 12,
}

crate::macros::widget::impl_widget_type!(OutputType);

/// Creates an output widget
#[derive(Debug)]
pub struct Output {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Output, Fl_Output);
crate::macros::widget::impl_widget_base!(Output, Fl_Output);
crate::macros::widget::impl_widget_default!(Output);
crate::macros::input::impl_input_ext!(Output, Fl_Output);

/// Creates a multiline-output widget
#[derive(Debug)]
pub struct MultilineOutput {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MultilineOutput, Fl_Multiline_Output);
crate::macros::widget::impl_widget_base!(MultilineOutput, Fl_Multiline_Output);
crate::macros::widget::impl_widget_default!(MultilineOutput);
crate::macros::input::impl_input_ext!(MultilineOutput, Fl_Multiline_Output);
