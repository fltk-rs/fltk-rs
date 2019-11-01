pub use crate::prelude::*;
use fltk_sys::window::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, GroupTrait, WindowTrait, Debug, Clone)]
pub struct Window {
    _inner: *mut Fl_Window,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
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
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}
