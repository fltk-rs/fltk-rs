use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::browser::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a normal browser
#[derive(WidgetExt, BrowserExt, Debug)]
pub struct Browser {
    _inner: *mut Fl_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
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

/// Creates a select browser
#[derive(WidgetExt, BrowserExt, Debug)]
pub struct SelectBrowser {
    _inner: *mut Fl_Select_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a multi-browser
#[derive(WidgetExt, BrowserExt, Debug)]
pub struct MultiBrowser {
    _inner: *mut Fl_Multi_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a hold browser
#[derive(WidgetExt, BrowserExt, Debug)]
pub struct HoldBrowser {
    _inner: *mut Fl_Hold_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a file browser
#[derive(WidgetExt, BrowserExt, Debug)]
pub struct FileBrowser {
    _inner: *mut Fl_File_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
