use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::valuator::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a slider widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Slider {
    _inner: *mut Fl_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a nice slider widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct NiceSlider {
    _inner: *mut Fl_Nice_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines slider types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum SliderType {
    Vertical = 0,
    Horizontal = 1,
    VerticalFill = 2,
    HorizontalFill = 3,
    VerticalNice = 4,
    HorizontalNice = 5,
}

/// Creates a dial widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Dial {
    _inner: *mut Fl_Dial,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a line dial widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct LineDial {
    _inner: *mut Fl_Line_Dial,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines dial types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum DialType {
    Normal = 0,
    Line = 1,
    Fill = 2,
}

/// Creates a counter widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Counter {
    _inner: *mut Fl_Counter,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines counter types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum CounterType {
    Normal = 0,
    Simple = 1,
}

/// Creates a scrollbar widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Scrollbar {
    _inner: *mut Fl_Scrollbar,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines scrollbar types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ScrollbarType {
    Vertical = 0,
    Horizontal = 1,
    VerticalFill = 2,
    HorizontalFill = 3,
    VerticalNice = 4,
    HorizontalNice = 5,
}

/// Creates a roller widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Roller {
    _inner: *mut Fl_Roller,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a value slider widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueSlider {
    _inner: *mut Fl_Value_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an adjuster widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Adjuster {
    _inner: *mut Fl_Adjuster,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an value input widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueInput {
    _inner: *mut Fl_Value_Input,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an value output widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueOutput {
    _inner: *mut Fl_Value_Output,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a fill slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct FillSlider {
    _inner: *mut Fl_Fill_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a fill dial
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct FillDial {
    _inner: *mut Fl_Fill_Dial,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a horizontal slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorSlider {
    _inner: *mut Fl_Hor_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a horizontal fill slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorFillSlider {
    _inner: *mut Fl_Hor_Fill_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a horizontal nice slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorNiceSlider {
    _inner: *mut Fl_Hor_Nice_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a horizontal value slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorValueSlider {
    _inner: *mut Fl_Hor_Value_Slider,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
