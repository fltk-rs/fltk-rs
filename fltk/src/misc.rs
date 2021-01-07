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
#[derive(WidgetBase, WidgetExt, Debug)]
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
        unsafe { Fl_Spinner_set_text_font(self._inner, f.bits() as i32) }
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
        unsafe { Fl_Spinner_set_text_color(self._inner, color.bits() as u32) }
    }
}

/// Creates a clock widget
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct Clock {
    _inner: *mut Fl_Clock,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a chart widget
#[derive(WidgetBase, WidgetExt, Debug)]
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
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_add(self._inner, val, txt.as_ptr(), col.bits() as u32) }
    }

    /// Inserts an entry at an index
    pub fn insert(&mut self, idx: u32, val: f64, txt: &str, col: Color) {
        debug_assert!(
            idx <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_insert(self._inner, idx as i32, val, txt.as_ptr(), col.bits() as u32) }
    }

    /// Replaces an entry at an index
    pub fn replace(&mut self, idx: u32, val: f64, txt: &str, col: Color) {
        debug_assert!(
            idx <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_replace(self._inner, idx as i32, val, txt.as_ptr(), col.bits() as u32) }
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
        unsafe { Fl_Chart_set_text_font(self._inner, f.bits() as i32) }
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
        unsafe { Fl_Chart_set_text_color(self._inner, color.bits() as u32) }
    }

    /// Returns wheter the chart is autosizable
    pub fn is_autosize(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Chart_is_autosize(self._inner) != 0
        }
    }

    /// Sets the ability of the chart to be autosizable
    pub fn make_autosize(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Chart_make_autosize(self._inner, val as i32) }
    }
}

/// Creates a progress bar
#[derive(WidgetBase, WidgetExt, Debug)]
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
            Fl_Tooltip_enabled() != 0
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
        let tip = CString::safe_new(tip);
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
    pub fn current_widget() -> Box<dyn WidgetExt> {
        unsafe {
            let widget_ptr = Fl_Tooltip_current_widget();
            assert!(!widget_ptr.is_null());
            Box::new(Widget::from_widget_ptr(
                widget_ptr as *mut fltk_sys::widget::Fl_Widget,
            ))
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
        unsafe { Fl_Tooltip_set_font(font.bits() as i32) }
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
        unsafe { Fl_Tooltip_set_color(c.bits() as u32) }
    }

    /// Gets the tooltip's text color
    pub fn text_color() -> Color {
        unsafe { mem::transmute(Fl_Tooltip_text_color()) }
    }

    /// Sets the tooltip's text color
    pub fn set_text_color(c: Color) {
        unsafe { Fl_Tooltip_set_text_color(c.bits() as u32) }
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
    pub fn current_window() -> impl WindowExt {
        unsafe {
            let wind = Fl_Tooltip_current_window();
            assert!(!wind.is_null());
            Window::from_widget_ptr(wind as *mut fltk_sys::widget::Fl_Widget)
        }
    }
}

/// Creates an InputChoice widget
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct InputChoice {
    _inner: *mut Fl_Input_Choice,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl InputChoice {
    /// Set the down_box of the widget
    pub fn set_down_frame(&mut self, f: FrameType) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Input_Choice_set_down_box(self._inner, f as i32)
        }
    }
    
    /// Get the down frame type of the widget
    pub fn down_frame(&self) -> FrameType {
        assert!(!self.was_deleted());
        unsafe {
            mem::transmute(Fl_Input_Choice_down_box(self._inner))
        }
    }

    /// Add an element to the input choice
    pub fn add(&mut self, s: &str) {
        assert!(!self.was_deleted());
        let s = CString::safe_new(s);
        unsafe {
            Fl_Input_Choice_add(self._inner, s.as_ptr())
        }
    }

    /// Clear the input choice widget
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Input_Choice_clear(self._inner)
        }
    }

    /// Get the value of the current choice
    pub fn value(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Input_Choice_value(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            }
        }
    }

    /// Set the value to a string
    pub fn set_value(&mut self, val: &str) {
        assert!(!self.was_deleted());
        let val = CString::safe_new(val);
        unsafe {
            Fl_Input_Choice_set_value(self._inner, val.as_ptr())
        }
    }

    /// Set the value of the input choice to a current element
    pub fn set_value2(&mut self, val: u32) {
        debug_assert!(
            val <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe {
            Fl_Input_Choice_set_value2(self._inner, val as i32)
        }
    }

    /// Get the associated menu button
    pub fn menu_button(&self) -> Box<dyn MenuExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Input_Choice_menubutton(self._inner);
            assert!(!ptr.is_null());
            Box::new(crate::menu::MenuButton::from_widget_ptr(ptr as _))
        }
    }
    
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Input_Choice_textfont(self._inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Input_Choice_set_textfont(self._inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Input_Choice_textsize(self._inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Input_Choice_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Input_Choice_textcolor(self._inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Input_Choice_set_textcolor(self._inner, color.bits() as u32) }
    }
}

/// Creates a HelpView widget
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct HelpView {
    _inner: *mut Fl_Help_View,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl HelpView {
    /// Return the directory
    pub fn directory(&self) -> std::path::PathBuf {
        assert!(!self.was_deleted());
        unsafe {
            let x = Fl_Help_View_directory(self._inner);
            if !x.is_null() {
                std::path::PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                std::path::PathBuf::from("")
            }
        }
    }

    /// Return the filename
    pub fn filename(&self) -> std::path::PathBuf {
        assert!(!self.was_deleted());
        unsafe {
            let x = Fl_Help_View_directory(self._inner);
            if !x.is_null() {
                std::path::PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                std::path::PathBuf::from("")
            }
        }
    }

    /// Find a string, returns the index
    pub fn find(&self, s: &str, start_from: usize) -> Option<usize> {
        assert!(!self.was_deleted());
        if let Some(v) = self.value() {
            v[start_from..].find(s).map(|idx| start_from + idx)
        } else {
            None
        }
    }

    /// Get the value of the widget
    pub fn value(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let val = Fl_Help_View_value(self._inner);
            if val.is_null() {
                None
            } else {
                Some(CStr::from_ptr(val).to_string_lossy().to_string())
            }
        }
    }

    /// Set value of the widget
    pub fn set_value(&mut self, val: &str) {
        assert!(!self.was_deleted());
        let val = CString::safe_new(val);
        unsafe { Fl_Help_View_set_value(self._inner, val.as_ptr()) }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_clear_selection(self._inner) }
    }

    /// Select all
    pub fn select_all(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_select_all(self._inner) }
    }

    /// Set the topline string
    pub fn set_topline(&mut self, n: &str) {
        assert!(!self.was_deleted());
        let n = CString::safe_new(n);
        unsafe { Fl_Help_View_set_topline(self._inner, n.as_ptr()) }
    }

    /// Set the leftline position
    pub fn set_topline2(&mut self, arg1: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_topline2(self._inner, arg1) }
    }

    /// Get the topline position
    pub fn topline(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_topline(self._inner) }
    }

    /// Set the leftline position
    pub fn set_leftline(&mut self, arg1: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_leftline(self._inner, arg1) }
    }

    /// Gets the current leftline in pixels
    pub fn leftline(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_leftline(self._inner) }
    }


    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Help_View_textfont(self._inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_textfont(self._inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_textsize(self._inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Help_View_textcolor(self._inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_textcolor(self._inner, color.bits() as u32) }
    }


    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_scrollbar_size(self._inner) as u32 }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: u32) {
        debug_assert!(
            new_size <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Help_View_set_scrollbar_size(self._inner, new_size as i32) }
    }

    /// Load a view from a file or URI
    pub fn load(&mut self, f: &str) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        let f = CString::safe_new(f);
        unsafe { 
            match Fl_Help_View_load(self._inner, f.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }
}
