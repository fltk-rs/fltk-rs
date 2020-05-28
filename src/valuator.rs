use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a slider widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct Slider {
    _inner: *mut Fl_Slider,
}

/// Creates a nice slider widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct NiceSlider {
    _inner: *mut Fl_Nice_Slider,
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
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct Dial {
    _inner: *mut Fl_Dial,
}

/// Creates a line dial widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct LineDial {
    _inner: *mut Fl_Line_Dial,
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
#[derive(WidgetExt, ValuatorExt, Debug)]
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
#[derive(WidgetExt, ValuatorExt, Debug)]
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
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct Roller {
    _inner: *mut Fl_Roller,
}

/// Creates a value slider widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct ValueSlider {
    _inner: *mut Fl_Value_Slider,
}

/// Creates an adjuster widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct Adjuster {
    _inner: *mut Fl_Adjuster,
}

/// Creates an value input widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct ValueInput {
    _inner: *mut Fl_Value_Input,
}

/// Creates an value output widget
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct ValueOutput {
    _inner: *mut Fl_Value_Output,
}

/// Creates a fill slider
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct FillSlider {
    _inner: *mut Fl_Fill_Slider,
}

/// Creates a fill dial
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct FillDial {
    _inner: *mut Fl_Fill_Dial,
}

/// Creates a horizontal slider
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct HorSlider {
    _inner: *mut Fl_Hor_Slider,
}

/// Creates a horizontal fill slider
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct HorFillSlider {
    _inner: *mut Fl_Hor_Fill_Slider,
}

/// Creates a horizontal nice slider
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct HorNiceSlider {
    _inner: *mut Fl_Hor_Nice_Slider,
}

/// Creates a horizontal value slider
#[derive(WidgetExt, ValuatorExt, Debug)]
pub struct HorValueSlider {
    _inner: *mut Fl_Hor_Value_Slider,
}
