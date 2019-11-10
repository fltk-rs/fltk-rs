pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{ffi, mem, os::raw};

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Slider {
    _inner: *mut Fl_Slider,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Dial {
    _inner: *mut Fl_Dial,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Counter {
    _inner: *mut Fl_Counter,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Scrollbar {
    _inner: *mut Fl_Scrollbar,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Roller {
    _inner: *mut Fl_Roller,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct ValueSlider {
    _inner: *mut Fl_Value_Slider,
}
