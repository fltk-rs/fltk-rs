pub use crate::prelude::*;
use fltk_sys::group::*;
use std::{ffi::CString, mem, os::raw};

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Group {
    _inner: *mut Fl_Group,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Pack {
    _inner: *mut Fl_Pack,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Scroll {
    _inner: *mut Fl_Scroll,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tabs {
    _inner: *mut Fl_Tabs,
}

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tile {
    _inner: *mut Fl_Tile,
}
