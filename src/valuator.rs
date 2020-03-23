pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{ffi::CString, mem, os::raw};

/// Creates a slider widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Slider {
    _inner: *mut Fl_Slider,
}

/// Creates a dial widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Dial {
    _inner: *mut Fl_Dial,
}

/// Creates a counter widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Counter {
    _inner: *mut Fl_Counter,
}

/// Creates a scrollbar widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Scrollbar {
    _inner: *mut Fl_Scrollbar,
}

/// Creates a roller widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Roller {
    _inner: *mut Fl_Roller,
}

/// Creates a value slider widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct ValueSlider {
    _inner: *mut Fl_Value_Slider,
}
