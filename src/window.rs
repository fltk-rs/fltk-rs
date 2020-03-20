pub use crate::prelude::*;
use fltk_sys::window::*;
use std::{ffi::CString, mem, os::raw};

#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
pub struct Window {
    _inner: *mut Fl_Window,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum WindowType {
    NormalWindow = 240,
    DoubleWindow = 241,
}

#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
}
