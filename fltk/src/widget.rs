//! Basic empty widget
//!
//! **Multithreaded** applications can call widget methods from non-main
//! threads, but will need to call [`app::awake()`](`crate::app::awake`) to make
//! the main thread run the event loop.

use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::widget::*;
use std::ffi::{CStr, CString};

/// An abstract type, shouldn't be instantiated in user code
#[derive(Debug)]
pub struct Widget {
    inner: *mut Fl_Widget,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Widget, Fl_Widget);
crate::macros::widget::impl_widget_base!(Widget, Fl_Widget);

/// An alias exposing the Widget tracker
pub type WidgetTrackerPtr = *mut fltk_sys::fl::Fl_Widget_Tracker;
