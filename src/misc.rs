pub use crate::prelude::*;
use fltk_sys::misc::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates a spinner widget
#[derive(WidgetTrait, Debug, Clone)]
pub struct Spinner {
    _inner: *mut Fl_Spinner,
}

impl Spinner {
    pub fn minimum(&self) -> f64 {
        unsafe {
            Fl_Spinner_minimum(self._inner)
        }
    }
    pub fn set_minimum(&mut self, a: f64) {
        unsafe {
            Fl_Spinner_set_minimum(self._inner, a)
        }
    }
    pub fn maximum(&self) -> f64 {
        unsafe {
            Fl_Spinner_maximum(self._inner)
        }
    }
    pub fn set_maximum(&mut self, a: f64) {
        unsafe {
            Fl_Spinner_set_maximum(self._inner, a)
        }
    }
    pub fn set_range(&mut self, a: f64, b: f64) {
        unsafe {
            Fl_Spinner_set_range(self._inner, a, b)
        }
    }
    pub fn set_step(&mut self, a: f64) {
        unsafe {
            Fl_Spinner_set_step(self._inner, a)
        }
    }
    pub fn step(&self) -> f64 {
        unsafe {
            Fl_Spinner_step(self._inner)
        }
    }
    pub fn maximum_size(&self) -> u32 {
        unsafe {
            Fl_Spinner_maxsize(self._inner) as u32
        }
    }

    pub fn set_maximum_size(&mut self, s: u32) {
        unsafe {
            Fl_Spinner_set_maxsize(self._inner, s as i32)
        }
    }

    pub fn text_font(&self) -> Font {
        unsafe {
            std::mem::transmute(Fl_Spinner_text_font(self._inner))
        }
    }

    pub fn set_text_font(&mut self, f: Font) {
        unsafe {
            Fl_Spinner_set_text_font(self._inner, f as i32)
        }
    }

    pub fn text_size(&self) -> u32 {
        unsafe {
            Fl_Spinner_text_size(self._inner) as u32
        }
    }

    pub fn set_text_size(&mut self, s: u32) {
        unsafe {
            Fl_Spinner_set_textsize(self._inner, s as i32)
        }
    }

    pub fn text_color(&self) -> Color {
        unsafe {
            std::mem::transmute(Fl_Spinner_text_color(self._inner))
        }
    }

    pub fn set_text_color(&mut self, color: Color) {
        unsafe {
            Fl_Spinner_set_text_color(self._inner, color as u32)
        }
    }
}

/// Creates a clock widget
#[derive(WidgetTrait, Debug, Clone)]
pub struct Clock {
    _inner: *mut Fl_Clock,
}

/// Creates a chart widget
#[derive(WidgetTrait, Debug, Clone)]
pub struct Chart {
    _inner: *mut Fl_Chart,
}

impl Chart {
    pub fn clear(&mut self) {
        unsafe {
            Fl_Chart_clear(self._inner)
        }
    }

    pub fn add(&mut self, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe {
            Fl_Chart_add(self._inner, val, txt.into_raw() as *const raw::c_char, col)
        }
    }

    pub fn insert(&mut self, idx: usize, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe {
            Fl_Chart_insert(self._inner, idx as i32, val, txt.into_raw() as *const raw::c_char, col)
        }
    }

    pub fn replace(&mut self, idx: usize, val: f64, txt: &str, col: u32) {
        let txt = std::ffi::CString::new(txt).unwrap();
        unsafe {
            Fl_Chart_replace(self._inner, idx as i32, val, txt.into_raw() as *const raw::c_char, col)
        }
    }

    pub fn set_bounds(&mut self, a: f64, b: f64) {
        unsafe {
            Fl_Chart_set_bounds(self._inner, a, b)
        }
    }

    pub fn size(&self) -> u32 {
        unsafe {
            Fl_Chart_size(self._inner) as u32
        }
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        unsafe {
            Fl_Chart_set_size(self._inner, w as i32, h as i32)
        }
    }

    pub fn maximum_size(&self) -> u32 {
        unsafe {
            Fl_Chart_maxsize(self._inner) as u32
        }
    }

    pub fn set_maximum_size(&mut self, s: u32) {
        unsafe {
            Fl_Chart_set_maxsize(self._inner, s as i32)
        }
    }

    pub fn text_font(&self) -> Font {
        unsafe {
            std::mem::transmute(Fl_Chart_text_font(self._inner))
        }
    }

    pub fn set_text_font(&mut self, f: Font) {
        unsafe {
            Fl_Chart_set_text_font(self._inner, f as i32)
        }
    }

    pub fn text_size(&self) -> u32 {
        unsafe {
            Fl_Chart_text_size(self._inner) as u32
        }
    }

    pub fn set_text_size(&mut self, s: u32) {
        unsafe {
            Fl_Chart_set_textsize(self._inner, s as i32)
        }
    }

    pub fn text_color(&self) -> Color {
        unsafe {
            std::mem::transmute(Fl_Chart_text_color(self._inner))
        }
    }

    pub fn set_text_color(&mut self, color: Color) {
        unsafe {
            Fl_Chart_set_text_color(self._inner, color as u32)
        }
    }

    pub fn is_autosize(&self) -> bool {
        unsafe {
            match Fl_Chart_is_autosize(self._inner,) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn make_autosize(&mut self, val: bool) {
        unsafe {
            Fl_Chart_make_autosize(self._inner, val as i32)
        }
    }
}