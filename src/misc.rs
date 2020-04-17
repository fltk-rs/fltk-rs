use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::misc::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a spinner widget
#[derive(WidgetExt, Debug, Clone)]
pub struct Spinner {
    _inner: *mut Fl_Spinner,
}

impl Spinner {
    /// Returns the minimum value of the spinner widget
    pub fn minimum(&self) -> f64 {
        unsafe { Fl_Spinner_minimum(self._inner) }
    }
    /// Sets the minimu value of the spinner widget
    pub fn set_minimum(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_minimum(self._inner, a) }
    }
    /// Returns the maximum value of the spinner widget
    pub fn maximum(&self) -> f64 {
        unsafe { Fl_Spinner_maximum(self._inner) }
    }
    /// Sets the minimum value of the spinner widget
    pub fn set_maximum(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_maximum(self._inner, a) }
    }
    /// Sets the range of the spinner widget
    pub fn set_range(&mut self, a: f64, b: f64) {
        unsafe { Fl_Spinner_set_range(self._inner, a, b) }
    }
    /// Sets the step of the spinner widget
    pub fn set_step(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_step(self._inner, a) }
    }
    /// Gets the range of the spinner widget
    pub fn step(&self) -> f64 {
        unsafe { Fl_Spinner_step(self._inner) }
    }
    /// Returns the maximum size supported by the spinner widget
    pub fn maximum_size(&self) -> u32 {
        unsafe { Fl_Spinner_maxsize(self._inner) as u32 }
    }
    /// Sets the maximum size supported by the spinner widget
    pub fn set_maximum_size(&mut self, s: u32) {
        unsafe { Fl_Spinner_set_maxsize(self._inner, s as i32) }
    }
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Spinner_text_font(self._inner)) }
    }
    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Spinner_set_text_font(self._inner, f as i32) }
    }
    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        unsafe { Fl_Spinner_text_size(self._inner) as u32 }
    }
    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        unsafe { Fl_Spinner_set_textsize(self._inner, s as i32) }
    }
    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Spinner_text_color(self._inner)) }
    }
    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Spinner_set_text_color(self._inner, color as u32) }
    }
}

/// Creates a clock widget
#[derive(WidgetExt, Debug, Clone)]
pub struct Clock {
    _inner: *mut Fl_Clock,
}

/// Creates a chart widget
#[derive(WidgetExt, Debug, Clone)]
pub struct Chart {
    _inner: *mut Fl_Chart,
}

impl Chart {
    /// Clears the chart
    pub fn clear(&mut self) {
        unsafe { Fl_Chart_clear(self._inner) }
    }
    /// Adds an entry
    pub fn add(&mut self, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe { Fl_Chart_add(self._inner, val, txt.into_raw() as *const raw::c_char, col) }
    }
    /// Inserts an entry at an index
    pub fn insert(&mut self, idx: u32, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe {
            Fl_Chart_insert(
                self._inner,
                idx as i32,
                val,
                txt.into_raw() as *const raw::c_char,
                col,
            )
        }
    }
    /// Replaces an entry at an index
    pub fn replace(&mut self, idx: u32, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe {
            Fl_Chart_replace(
                self._inner,
                idx as i32,
                val,
                txt.into_raw() as *const raw::c_char,
                col,
            )
        }
    }
    /// Sets the bounds of the chart
    pub fn set_bounds(&mut self, a: f64, b: f64) {
        unsafe { Fl_Chart_set_bounds(self._inner, a, b) }
    }
    /// Returns the size of the chart
    pub fn size(&self) -> u32 {
        unsafe { Fl_Chart_size(self._inner) as u32 }
    }
    /// Sets the size of the chart
    pub fn set_size(&mut self, w: u32, h: u32) {
        unsafe { Fl_Chart_set_size(self._inner, w as i32, h as i32) }
    }
    /// Gets the maximum supported size of the chart
    pub fn maximum_size(&self) -> u32 {
        unsafe { Fl_Chart_maxsize(self._inner) as u32 }
    }
    /// Sets the maximum supported size of the chart
    pub fn set_maximum_size(&mut self, s: u32) {
        unsafe { Fl_Chart_set_maxsize(self._inner, s as i32) }
    }
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Chart_text_font(self._inner)) }
    }
    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Chart_set_text_font(self._inner, f as i32) }
    }
    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        unsafe { Fl_Chart_text_size(self._inner) as u32 }
    }
    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        unsafe { Fl_Chart_set_textsize(self._inner, s as i32) }
    }
    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Chart_text_color(self._inner)) }
    }
    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Chart_set_text_color(self._inner, color as u32) }
    }
    /// Returns wheter the chart is autosizable
    pub fn is_autosize(&self) -> bool {
        unsafe {
            match Fl_Chart_is_autosize(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }
    /// Sets the ability of the chart to be autosizable
    pub fn make_autosize(&mut self, val: bool) {
        unsafe { Fl_Chart_make_autosize(self._inner, val as i32) }
    }
}

/// Creates a progress bar
#[derive(WidgetExt, Debug, Clone)]
pub struct Progress {
    _inner: *mut Fl_Progress,
}

impl Progress {
    /// Returns the minimum value of the progress bar
    pub fn minimum(&self) -> f64 {
        unsafe { Fl_Progress_minimum(self._inner) }
    }
    /// Sets the minimu value of the progress bar
    pub fn set_minimum(&mut self, a: f64) {
        unsafe { Fl_Progress_set_minimum(self._inner, a) }
    }
    /// Returns the maximum value of the progress bar
    pub fn maximum(&self) -> f64 {
        unsafe { Fl_Progress_maximum(self._inner) }
    }
    /// Sets the minimum value of the progress bar
    pub fn set_maximum(&mut self, a: f64) {
        unsafe { Fl_Progress_set_maximum(self._inner, a) }
    }
    /// Returns the value of the progress bar
    pub fn value(&self) -> f64 {
        unsafe { Fl_Progress_value(self._inner) }
    }
    /// Sets the value of the progress bar
    pub fn set_value(&mut self, arg2: f64) {
        unsafe {
            Fl_Progress_set_value(self._inner, arg2);
        }
    }
}
