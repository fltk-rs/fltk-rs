pub use crate::prelude::*;
use fltk_sys::browser::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates a normal browser
#[derive(WidgetTrait, BrowserTrait, Debug, Clone)]
pub struct Browser {
    _inner: *mut Fl_Browser,
}

/// Defines the browser type, which can be changed dynamically using the set_type function().
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum BrowserType {
    NormalBrowser = 0,
    SelectBrowser = 1,
    HoldBrowser = 2,
    MultiBrowser = 3,
}

/// Creates a radio browser
#[derive(WidgetTrait, BrowserTrait, Debug, Clone)]
pub struct SelectBrowser {
    _inner: *mut Fl_Select_Browser,
}

/// Creates a multi-browser
#[derive(WidgetTrait, BrowserTrait, Debug, Clone)]
pub struct MultiBrowser {
    _inner: *mut Fl_Multi_Browser,
}

/// Creates a hold browser
#[derive(WidgetTrait, BrowserTrait, Debug, Clone)]
pub struct HoldBrowser {
    _inner: *mut Fl_Hold_Browser,
}
