pub use crate::prelude::*;
use std::{ffi, mem, ptr};

#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Group {
    _inner: *mut fltk_sys::group::Fl_Group,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum GroupType {
    NormalGroup = 0,
}

impl Group {
    pub fn as_ptr(&self) -> *mut fltk_sys::group::Fl_Group {
        self._inner
    }
}
