pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::group::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

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

/// Creates a wizard widget
#[derive(WidgetTrait, GroupTrait, Debug, Clone)]
pub struct Wizard {
    _inner: *mut Fl_Wizard,
}

impl Wizard {
    /// Gets the next view of the wizard
    pub fn next(&mut self) {
        unsafe {
            Fl_Wizard_next(self._inner)
        }
    }
    /// Gets the previous view of the wizard
    pub fn prev(&mut self) {
        unsafe {
            Fl_Wizard_prev(self._inner)
        }
    }
    /// Gets the underlying widget of the current view
    pub fn current_widget(&mut self) -> Widget {
        unsafe {
            Widget::from_raw(Fl_Wizard_value(self._inner) as *mut fltk_sys::widget::Fl_Widget)
        }
    }
    /// Sets the underlying widget of the current view
    pub fn set_current_widget<W: WidgetTrait>(&mut self, w: W) {
        unsafe {
            Fl_Wizard_set_value(self._inner, w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget)
        }
    } 
}