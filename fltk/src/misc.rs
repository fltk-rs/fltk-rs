use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::Widget;
use crate::window::Window;
use fltk_sys::misc::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Defines the chart types supported by fltk
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ChartType {
    Bar = 0,
    HorizontalBar = 1,
    Line = 2,
    Fill = 3,
    Spike = 4,
    Pie = 5,
    SpecialPie = 6,
}

/// Defines the clock types supported by fltk
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ClockType {
    Square = 0,
    Round = 1,
}

/// Creates a spinner widget
#[derive(WidgetExt, Debug)]
pub struct Spinner {
    _inner: *mut Fl_Spinner,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Spinner {
    /// Returns the minimum value of the spinner widget
    pub fn minimum(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_minimum(self._inner) }
    }

    /// Sets the minimu value of the spinner widget
    pub fn set_minimum(&mut self, a: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_minimum(self._inner, a) }
    }

    /// Returns the maximum value of the spinner widget
    pub fn maximum(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_maximum(self._inner) }
    }

    /// Sets the minimum value of the spinner widget
    pub fn set_maximum(&mut self, a: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_maximum(self._inner, a) }
    }

    /// Sets the range of the spinner widget
    pub fn set_range(&mut self, a: f64, b: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_range(self._inner, a, b) }
    }

    /// Sets the step of the spinner widget
    pub fn set_step(&mut self, a: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_step(self._inner, a) }
    }

    /// Gets the range of the spinner widget
    pub fn step(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_step(self._inner) }
    }

    /// Returns the maximum size supported by the spinner widget
    pub fn maximum_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_maxsize(self._inner) as u32 }
    }

    /// Sets the maximum size supported by the spinner widget
    pub fn set_maximum_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_maxsize(self._inner, s as i32) }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Spinner_text_font(self._inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_text_font(self._inner, f as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_text_size(self._inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Spinner_text_color(self._inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Spinner_set_text_color(self._inner, color as u32) }
    }
}

/// Creates a clock widget
#[derive(WidgetExt, Debug)]
pub struct Clock {
    _inner: *mut Fl_Clock,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a chart widget
#[derive(WidgetExt, Debug)]
pub struct Chart {
    _inner: *mut Fl_Chart,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Chart {
    /// Clears the chart
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_clear(self._inner) }
    }

    /// Adds an entry
    pub fn add(&mut self, val: f64, txt: &str, col: Color) {
        assert!(!self.was_deleted());
        let txt = match CString::new(txt) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        };
        unsafe { Fl_Chart_add(self._inner, val, txt.as_ptr(), col as u32) }
    }

    /// Inserts an entry at an index
    pub fn insert(&mut self, idx: u32, val: f64, txt: &str, col: Color) {
        debug_assert!(
            idx <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        let txt = match CString::new(txt) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        };
        unsafe { Fl_Chart_insert(self._inner, idx as i32, val, txt.as_ptr(), col as u32) }
    }

    /// Replaces an entry at an index
    pub fn replace(&mut self, idx: u32, val: f64, txt: &str, col: Color) {
        debug_assert!(
            idx <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        let txt = match CString::new(txt) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        };
        unsafe { Fl_Chart_replace(self._inner, idx as i32, val, txt.as_ptr(), col as u32) }
    }

    /// Sets the bounds of the chart
    pub fn set_bounds(&mut self, a: f64, b: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_bounds(self._inner, a, b) }
    }

    /// Returns the size of the chart
    pub fn size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_size(self._inner) as u32 }
    }

    /// Sets the size of the chart
    pub fn set_size(&mut self, w: u32, h: u32) {
        debug_assert!(
            w <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        debug_assert!(
            h <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_size(self._inner, w as i32, h as i32) }
    }

    /// Gets the maximum supported size of the chart
    pub fn maximum_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_maxsize(self._inner) as u32 }
    }

    /// Sets the maximum supported size of the chart
    pub fn set_maximum_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_maxsize(self._inner, s as i32) }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Chart_text_font(self._inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_text_font(self._inner, f as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_text_size(self._inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Chart_text_color(self._inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_set_text_color(self._inner, color as u32) }
    }

    /// Returns wheter the chart is autosizable
    pub fn is_autosize(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Chart_is_autosize(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets the ability of the chart to be autosizable
    pub fn make_autosize(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_make_autosize(self._inner, val as i32) }
    }
}

/// Creates a progress bar
#[derive(WidgetExt, Debug)]
pub struct Progress {
    _inner: *mut Fl_Progress,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Progress {
    /// Returns the minimum value of the progress bar
    pub fn minimum(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Progress_minimum(self._inner) }
    }

    /// Sets the minimu value of the progress bar
    pub fn set_minimum(&mut self, a: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Progress_set_minimum(self._inner, a) }
    }

    /// Returns the maximum value of the progress bar
    pub fn maximum(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Progress_maximum(self._inner) }
    }

    /// Sets the minimum value of the progress bar
    pub fn set_maximum(&mut self, a: f64) {
        assert!(!self.was_deleted());
        unsafe { Fl_Progress_set_maximum(self._inner, a) }
    }

    /// Returns the value of the progress bar
    pub fn value(&self) -> f64 {
        assert!(!self.was_deleted());
        unsafe { Fl_Progress_value(self._inner) }
    }

    /// Sets the value of the progress bar
    pub fn set_value(&mut self, arg2: f64) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Progress_set_value(self._inner, arg2);
        }
    }
}

/// Shows a standalone tooltip
#[derive(Clone, Debug)]
pub struct Tooltip {}

impl Tooltip {
    /// Gets the tooltip's delay
    pub fn delay() -> f32 {
        unsafe { Fl_Tooltip_delay() }
    }

    /// Sets the tooltip's delay
    pub fn set_delay(f: f32) {
        unsafe { Fl_Tooltip_set_delay(f) }
    }

    /// Gets the tooltip's hide delay
    pub fn hidedelay() -> f32 {
        unsafe { Fl_Tooltip_hidedelay() }
    }

    /// Sets the tooltip's hide delay
    pub fn set_hidedelay(f: f32) {
        unsafe { Fl_Tooltip_set_hidedelay(f) }
    }

    /// Gets the tooltip's hover delay
    pub fn hoverdelay() -> f32 {
        unsafe { Fl_Tooltip_hoverdelay() }
    }

    /// Sets the tooltip's hover delay
    pub fn set_hoverdelay(f: f32) {
        unsafe { Fl_Tooltip_set_hoverdelay(f) }
    }

    /// Returns whether the tooltip is enabled
    pub fn enabled() -> bool {
        unsafe {
            match Fl_Tooltip_enabled() {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the tooltip is enabled
    pub fn enable(b: bool) {
        unsafe { Fl_Tooltip_enable(b as i32) }
    }

    /// Disables the tooltip
    pub fn disable() {
        unsafe { Fl_Tooltip_disable() }
    }

    /// Defines the area of the tooltip
    pub fn enter_area<W: WidgetExt>(widget: &W, x: i32, y: i32, w: i32, h: i32, tip: &str) {
        assert!(!widget.was_deleted());
        let tip = match CString::new(tip) {
            Ok(v) => v,
            Err(r) => {
                let i = r.nul_position();
                CString::new(&r.into_vec()[0..i]).unwrap()
            }
        };
        unsafe {
            Fl_Tooltip_enter_area(
                widget.as_widget_ptr() as *mut Fl_Widget,
                x,
                y,
                w,
                h,
                tip.as_ptr(),
            )
        }
    }

    /// Returns the current widget under the tooltip
    pub fn current_widget() -> Widget {
        unsafe {
            let widget_ptr = Fl_Tooltip_current_widget();
            assert!(!widget_ptr.is_null());
            Widget::from_raw(widget_ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Sets the current widget associated with the tooltip
    pub fn current<W: WidgetExt>(w: &W) {
        assert!(!w.was_deleted());
        unsafe { Fl_Tooltip_current(w.as_widget_ptr() as *mut Fl_Widget) }
    }

    /// Gets the tooltip's font
    pub fn font() -> Font {
        unsafe { mem::transmute(Fl_Tooltip_font()) }
    }

    /// Sets the tooltip's font
    pub fn set_font(font: Font) {
        unsafe { Fl_Tooltip_set_font(font as i32) }
    }

    /// Gets the tooltip's font size
    pub fn font_size() -> u32 {
        unsafe { Fl_Tooltip_font_size() as u32 }
    }

    /// Sets the tooltip's font size
    pub fn set_font_size(s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        unsafe { Fl_Tooltip_set_font_size(s as i32) }
    }

    /// Gets the tooltip's color
    pub fn color() -> Color {
        unsafe { mem::transmute(Fl_Tooltip_color()) }
    }

    /// Sets the tooltip's color
    pub fn set_color(c: Color) {
        unsafe { Fl_Tooltip_set_color(c as u32) }
    }

    /// Gets the tooltip's text color
    pub fn text_color() -> Color {
        unsafe { mem::transmute(Fl_Tooltip_text_color()) }
    }

    /// Sets the tooltip's text color
    pub fn set_text_color(c: Color) {
        unsafe { Fl_Tooltip_set_text_color(c as u32) }
    }

    /// Gets the margin width
    pub fn margin_width() -> u32 {
        unsafe { Fl_Tooltip_margin_width() as u32 }
    }

    /// Sets the margin width
    pub fn set_margin_width(v: u32) {
        debug_assert!(
            v <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        unsafe { Fl_Tooltip_set_margin_width(v as i32) }
    }

    /// Gets the margin height
    pub fn margin_height() -> u32 {
        unsafe { Fl_Tooltip_margin_height() as u32 }
    }

    /// Sets the margin height
    pub fn set_margin_height(v: u32) {
        debug_assert!(
            v <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        unsafe { Fl_Tooltip_set_margin_height(v as i32) }
    }

    /// Gets the wrap width
    pub fn wrap_width() -> u32 {
        unsafe { Fl_Tooltip_wrap_width() as u32 }
    }

    /// Sets the wrap width
    pub fn set_wrap_width(v: u32) {
        debug_assert!(
            v <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        unsafe { Fl_Tooltip_set_wrap_width(v as i32) }
    }

    /// Returns the current window
    pub fn current_window<W: WindowExt>() -> Window {
        unsafe {
            let wind = Fl_Tooltip_current_window();
            assert!(!wind.is_null());
            Window::from_widget_ptr(wind as *mut fltk_sys::widget::Fl_Widget)
        }
    }
}
