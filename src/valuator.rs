pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{ffi, mem, os::raw, ptr};

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Slider {
    _inner: *mut Fl_Slider,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Dial {
    _inner: *mut Fl_Dial,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Counter {
    _inner: *mut Fl_Counter,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Scrollbar {
    _inner: *mut Fl_Scrollbar,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Roller {
    _inner: *mut Fl_Roller,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct ValueSlider {
    _inner: *mut Fl_Value_Slider,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}
