use crate::enums::{Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::widget::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

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
