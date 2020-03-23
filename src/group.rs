pub use crate::prelude::*;
use fltk_sys::group::*;
use std::{ffi::CString, mem, os::raw};

/// Creates an widget group
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Group {
    _inner: *mut Fl_Group,
}

/// Creates an widget pack
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Pack {
    _inner: *mut Fl_Pack,
}

/// Creates a scroll group
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Scroll {
    _inner: *mut Fl_Scroll,
}

/// Creates a tab which can contain widgets
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tabs {
    _inner: *mut Fl_Tabs,
}

/// Creates a tile which can contain widgets
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Tile {
    _inner: *mut Fl_Tile,
}
