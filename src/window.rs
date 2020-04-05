pub use crate::prelude::*;
use crate::widget::*;
use crate::app::*;
use fltk_sys::window::*;
use crate::image::Image;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates a window widget
#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
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
#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
}

/// Creates a Menu window widget
#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
pub struct MenuWindow {
    _inner: *mut Fl_Menu_Window,
}

