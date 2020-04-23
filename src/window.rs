use crate::app::*;
use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::window::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct Window {
    _inner: *mut Fl_Window,
}

/// Defines the window type, can be set dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum WindowType {
    NormalWindow = 240,
    DoubleWindow = 241,
}

/// Creates a double window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
}

/// Creates a Menu window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct MenuWindow {
    _inner: *mut Fl_Menu_Window,
}
