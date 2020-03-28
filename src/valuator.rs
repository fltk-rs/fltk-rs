pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates a slider widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Slider {
    _inner: *mut Fl_Slider,
}

/// Defines slider types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum SliderType {
    VerticalSlider = 0,
    HorizontalSlider = 1,
    VerticalFillSlider = 2,
    HorizontalFillSlider = 3,
    VerticalNiceSlider = 4,
    HorizontalNiceSlider = 5,
}

/// Creates a dial widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Dial {
    _inner: *mut Fl_Dial,
}

/// Defines dial types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum DialType {
    NormalDial = 0,
    LineDial = 1,
    FillDial = 2,
}

/// Creates a counter widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Counter {
    _inner: *mut Fl_Counter,
}

/// Defines counter types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum CounterType {
    NormalCounter = 0,
    SimpleCounter = 1,
}

/// Creates a scrollbar widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Scrollbar {
    _inner: *mut Fl_Scrollbar,
}

/// Defines scrollbar types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ScrollBarType {
    VerticalScrollBar = 0,
    HorizontalScrollBar = 1,
    VerticalFillScrollBar = 2,
    HorizontalFillScrollBar = 3,
    VerticalNiceScrollBar = 4,
    HorizontalNiceScrollBar = 5,
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

/// Creates an adjuster widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct Adjuster {
    _inner: *mut Fl_Adjuster,
}

/// Creates an adjuster widget
#[derive(WidgetTrait, ValuatorTrait, Debug, Clone)]
pub struct ValueInput {
    _inner: *mut Fl_Value_Input,
}