use crate::enums::{Color, Font, FrameType};
use crate::prelude::*;
use crate::utils::FlString;
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChartType {
    /// Bar chart
    Bar = 0,
    /// Horizontal bar chart
    HorizontalBar = 1,
    /// Line chart
    Line = 2,
    /// Fill chart
    Fill = 3,
    /// Spike chart
    Spike = 4,
    /// Pie chart
    Pie = 5,
    /// Special pie chart
    SpecialPie = 6,
}

crate::macros::widget::impl_widget_type!(ChartType);

/// Defines the clock types supported by fltk
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClockType {
    /// Square clock
    Square = 0,
    /// Round clock
    Round = 1,
}

crate::macros::widget::impl_widget_type!(ClockType);

/// Creates a spinner widget
#[derive(Debug)]
pub struct Spinner {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Spinner, Fl_Spinner);
crate::macros::widget::impl_widget_base!(Spinner, Fl_Spinner);
crate::macros::widget::impl_widget_default!(Spinner);

impl Spinner {
    /// Returns the minimum value of the spinner widget
    pub fn minimum(&self) -> f64 {
        unsafe { Fl_Spinner_minimum(self.inner.widget() as _) }
    }

    /// Sets the minimum value of the spinner widget
    pub fn set_minimum(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_minimum(self.inner.widget() as _, a) }
    }

    /// Returns the maximum value of the spinner widget
    pub fn maximum(&self) -> f64 {
        unsafe { Fl_Spinner_maximum(self.inner.widget() as _) }
    }

    /// Sets the minimum value of the spinner widget
    pub fn set_maximum(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_maximum(self.inner.widget() as _, a) }
    }

    /// Sets the range of the spinner widget
    pub fn set_range(&mut self, a: f64, b: f64) {
        unsafe { Fl_Spinner_set_range(self.inner.widget() as _, a, b) }
    }

    /// Sets the step of the spinner widget
    pub fn set_step(&mut self, a: f64) {
        unsafe { Fl_Spinner_set_step(self.inner.widget() as _, a) }
    }

    /// Gets the range of the spinner widget
    pub fn step(&self) -> f64 {
        unsafe { Fl_Spinner_step(self.inner.widget() as _) }
    }

    /// Returns the maximum size supported by the spinner widget
    pub fn maximum_size(&self) -> i32 {
        unsafe { Fl_Spinner_maxsize(self.inner.widget() as _) }
    }

    /// Sets the maximum size supported by the spinner widget
    pub fn set_maximum_size(&mut self, s: i32) {
        unsafe { Fl_Spinner_set_maxsize(self.inner.widget() as _, s) }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Spinner_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Spinner_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Spinner_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Spinner_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Spinner_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Spinner_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Returns the value of the spinner
    pub fn value(&self) -> f64 {
        unsafe { Fl_Spinner_value(self.inner.widget() as _) }
    }

    /// Sets the value of the spinner
    pub fn set_value(&mut self, arg2: f64) {
        unsafe {
            Fl_Spinner_set_value(self.inner.widget() as _, arg2);
        }
    }

    /// Returns whether wrap is set
    pub fn wrap(&self) -> bool {
        unsafe { Fl_Spinner_wrap(self.inner.widget() as _) != 0 }
    }

    /// Sets wrap for the spinner
    pub fn set_wrap(&mut self, flag: bool) {
        unsafe {
            Fl_Spinner_set_wrap(self.inner.widget() as _, flag as _);
        }
    }
}

/// Creates a clock widget
#[derive(Debug)]
pub struct Clock {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Clock, Fl_Clock);
crate::macros::widget::impl_widget_base!(Clock, Fl_Clock);
crate::macros::widget::impl_widget_default!(Clock);

/// Creates a chart widget
#[derive(Debug)]
pub struct Chart {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Chart, Fl_Chart);
crate::macros::widget::impl_widget_base!(Chart, Fl_Chart);
crate::macros::widget::impl_widget_default!(Chart);

impl Chart {
    /// Clears the chart
    pub fn clear(&mut self) {
        unsafe { Fl_Chart_clear(self.inner.widget() as _) }
    }

    /// Adds an entry
    pub fn add(&mut self, val: f64, txt: &str, col: Color) {
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_add(self.inner.widget() as _, val, txt.as_ptr(), col.bits()) }
    }

    /// Inserts an entry at an index
    pub fn insert(&mut self, idx: i32, val: f64, txt: &str, col: Color) {
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_insert(self.inner.widget() as _, idx, val, txt.as_ptr(), col.bits()) }
    }

    /// Replaces an entry at an index
    pub fn replace(&mut self, idx: i32, val: f64, txt: &str, col: Color) {
        let txt = CString::safe_new(txt);
        unsafe { Fl_Chart_replace(self.inner.widget() as _, idx, val, txt.as_ptr(), col.bits()) }
    }

    /// Sets the bounds of the chart
    pub fn set_bounds(&mut self, a: f64, b: f64) {
        unsafe { Fl_Chart_set_bounds(self.inner.widget() as _, a, b) }
    }

    /// Returns the size of the chart
    pub fn size(&self) -> i32 {
        unsafe { Fl_Chart_size(self.inner.widget() as _) }
    }

    /// Gets the maximum supported size of the chart
    pub fn maximum_size(&self) -> i32 {
        unsafe { Fl_Chart_maxsize(self.inner.widget() as _) }
    }

    /// Sets the maximum supported size of the chart
    pub fn set_maximum_size(&mut self, s: i32) {
        unsafe { Fl_Chart_set_maxsize(self.inner.widget() as _, s) }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Chart_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Chart_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Chart_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Chart_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Chart_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Chart_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Returns whether the chart is autosizable
    pub fn is_autosize(&self) -> bool {
        unsafe { Fl_Chart_is_autosize(self.inner.widget() as _) != 0 }
    }

    /// Sets the ability of the chart to be autosizable
    pub fn make_autosize(&mut self, val: bool) {
        unsafe { Fl_Chart_make_autosize(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a progress bar
#[derive(Debug)]
pub struct Progress {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Progress, Fl_Progress);
crate::macros::widget::impl_widget_base!(Progress, Fl_Progress);
crate::macros::widget::impl_widget_default!(Progress);

impl Progress {
    /// Returns the minimum value of the progress bar
    pub fn minimum(&self) -> f64 {
        unsafe { Fl_Progress_minimum(self.inner.widget() as _) }
    }

    /// Sets the minimum value of the progress bar
    pub fn set_minimum(&mut self, a: f64) {
        unsafe { Fl_Progress_set_minimum(self.inner.widget() as _, a) }
    }

    /// Returns the maximum value of the progress bar
    pub fn maximum(&self) -> f64 {
        unsafe { Fl_Progress_maximum(self.inner.widget() as _) }
    }

    /// Sets the minimum value of the progress bar
    pub fn set_maximum(&mut self, a: f64) {
        unsafe { Fl_Progress_set_maximum(self.inner.widget() as _, a) }
    }

    /// Returns the value of the progress bar
    pub fn value(&self) -> f64 {
        unsafe { Fl_Progress_value(self.inner.widget() as _) }
    }

    /// Sets the value of the progress bar
    pub fn set_value(&mut self, arg2: f64) {
        unsafe {
            Fl_Progress_set_value(self.inner.widget() as _, arg2);
        }
    }
}

/// Controls tooltips on an application-wide basis; use .set_tooltip() to add a tooltip to a particular widget
#[derive(Clone, Debug)]
pub struct Tooltip {}

impl Tooltip {
    /// Gets the tooltips delay
    pub fn delay() -> f32 {
        unsafe { Fl_Tooltip_delay() }
    }

    /// Sets the tooltips delay
    pub fn set_delay(f: f32) {
        unsafe { Fl_Tooltip_set_delay(f) }
    }

    /// Gets the tooltips hide delay
    pub fn hidedelay() -> f32 {
        unsafe { Fl_Tooltip_hidedelay() }
    }

    /// Sets the tooltips hide delay
    pub fn set_hidedelay(f: f32) {
        unsafe { Fl_Tooltip_set_hidedelay(f) }
    }

    /// Gets the tooltips hover delay
    pub fn hoverdelay() -> f32 {
        unsafe { Fl_Tooltip_hoverdelay() }
    }

    /// Sets the tooltips hover delay
    pub fn set_hoverdelay(f: f32) {
        unsafe { Fl_Tooltip_set_hoverdelay(f) }
    }

    /// Returns whether the tooltips are enabled
    pub fn enabled() -> bool {
        unsafe { Fl_Tooltip_enabled() != 0 }
    }

    /// Sets tooltips to be displayed if b is true; otherwise not to be displayed
    pub fn enable(b: bool) {
        unsafe { Fl_Tooltip_enable(b as i32) }
    }

    /// Disables the display of all tooltips
    pub fn disable() {
        unsafe { Fl_Tooltip_disable() }
    }

    /// Used to provide tooltips for internal pieces of your widget.
    /// Check FLTK's [documentation](https://www.fltk.org/doc-1.3/classFl__Tooltip.html#a55b4d5a9a98e69eef5716ca02a16d59e).
    /// The text of the tooltip must be a static CStr since the data is not copied by FLTK. This also avoid memory leaks in user code.
    pub fn enter_area<W: WidgetExt>(
        widget: &W,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        tip: &'static CStr,
    ) {
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

    /// Returns the current widget associated with the tooltip
    pub fn current_widget() -> impl WidgetExt {
        unsafe {
            let widget_ptr = Fl_Tooltip_current_widget();
            assert!(!widget_ptr.is_null());
            Widget::from_widget_ptr(widget_ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Sets the current widget associated with the tooltip
    pub fn current<W: WidgetExt>(w: &W) {
        unsafe { Fl_Tooltip_current(w.as_widget_ptr() as *mut Fl_Widget) }
    }

    /// Gets the tooltips font
    pub fn font() -> Font {
        unsafe { mem::transmute(Fl_Tooltip_font()) }
    }

    /// Sets the tooltips font
    pub fn set_font(font: Font) {
        unsafe { Fl_Tooltip_set_font(font.bits()) }
    }

    /// Gets the tooltips font size
    pub fn font_size() -> i32 {
        unsafe { Fl_Tooltip_font_size() }
    }

    /// Sets the tooltips font size
    pub fn set_font_size(s: i32) {
        unsafe { Fl_Tooltip_set_font_size(s) }
    }

    /// Gets the tooltips color
    pub fn color() -> Color {
        unsafe { mem::transmute(Fl_Tooltip_color()) }
    }

    /// Sets the tooltips color
    pub fn set_color(c: Color) {
        unsafe { Fl_Tooltip_set_color(c.bits()) }
    }

    /// Gets the tooltips text color
    pub fn text_color() -> Color {
        unsafe { mem::transmute(Fl_Tooltip_text_color()) }
    }

    /// Sets the tooltips text color
    pub fn set_text_color(c: Color) {
        unsafe { Fl_Tooltip_set_text_color(c.bits()) }
    }

    /// Gets the tooltips margin width
    pub fn margin_width() -> i32 {
        unsafe { Fl_Tooltip_margin_width() }
    }

    /// Sets the tooltips margin width
    pub fn set_margin_width(v: i32) {
        unsafe { Fl_Tooltip_set_margin_width(v) }
    }

    /// Gets the tooltips margin height
    pub fn margin_height() -> i32 {
        unsafe { Fl_Tooltip_margin_height() }
    }

    /// Sets the tooltips margin height
    pub fn set_margin_height(v: i32) {
        unsafe { Fl_Tooltip_set_margin_height(v) }
    }

    /// Gets the tooltips wrap width
    pub fn wrap_width() -> i32 {
        unsafe { Fl_Tooltip_wrap_width() }
    }

    /// Sets the tooltips wrap width
    pub fn set_wrap_width(v: i32) {
        unsafe { Fl_Tooltip_set_wrap_width(v) }
    }

    /// Returns the window used for tooltips
    pub fn current_window() -> impl WindowExt {
        unsafe {
            let wind = Fl_Tooltip_current_window();
            assert!(!wind.is_null());
            Window::from_widget_ptr(wind as *mut fltk_sys::widget::Fl_Widget)
        }
    }
}

/// Creates an `InputChoice` widget
#[derive(Debug)]
pub struct InputChoice {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(InputChoice, Fl_Input_Choice);
crate::macros::widget::impl_widget_base!(InputChoice, Fl_Input_Choice);
crate::macros::widget::impl_widget_default!(InputChoice);

impl InputChoice {
    /// Set the `down_box` of the widget
    pub fn set_down_frame(&mut self, f: FrameType) {
        unsafe { Fl_Input_Choice_set_down_box(self.inner.widget() as _, f.as_i32()) }
    }

    /// Get the down frame type of the widget
    pub fn down_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Input_Choice_down_box(self.inner.widget() as _)) }
    }

    /// Add an element to the input choice
    pub fn add(&mut self, s: &str) {
        let s = CString::safe_new(s);
        unsafe { Fl_Input_Choice_add(self.inner.widget() as _, s.as_ptr()) }
    }

    /// Clear the input choice widget
    pub fn clear(&mut self) {
        unsafe { Fl_Input_Choice_clear(self.inner.widget() as _) }
    }

    /// Get the value of the current choice
    pub fn value(&self) -> Option<String> {
        unsafe {
            let ptr = Fl_Input_Choice_value(self.inner.widget() as _);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            }
        }
    }

    /// Set the value to a string
    pub fn set_value(&mut self, val: &str) {
        let val = CString::safe_new(val);
        unsafe { Fl_Input_Choice_set_value(self.inner.widget() as _, val.as_ptr()) }
    }

    /// Set the value of the input choice to the element at `idx`
    pub fn set_value_index(&mut self, idx: i32) {
        unsafe { Fl_Input_Choice_set_value2(self.inner.widget() as _, idx) }
    }

    /// Get the associated input widget
    pub fn input(&self) -> crate::input::Input {
        unsafe {
            let ptr = Fl_Input_Choice_input(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::input::Input::from_widget_ptr(ptr as _)
        }
    }

    /// Get the associated menu button
    pub fn menu_button(&self) -> crate::menu::MenuButton {
        unsafe {
            let ptr = Fl_Input_Choice_menu_button(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::menu::MenuButton::from_widget_ptr(ptr as _)
        }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Input_Choice_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Input_Choice_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Input_Choice_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Input_Choice_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Input_Choice_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Input_Choice_set_text_color(self.inner.widget() as _, color.bits()) }
    }
}

/**
    Creates a `HelpView` widget which supports HTML 2 formatting
    ```rust,no_run
    use fltk::{prelude::*, *};
    let mut h = misc::HelpView::new(10, 10, 380, 280, "");
    h.set_value("Hello <b><font color=red>again</font></b>");

    ```
*/
#[derive(Debug)]
pub struct HelpView {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HelpView, Fl_Help_View);
crate::macros::widget::impl_widget_base!(HelpView, Fl_Help_View);
crate::macros::widget::impl_widget_default!(HelpView);

impl HelpView {
    /// Return the directory
    pub fn directory(&self) -> std::path::PathBuf {
        unsafe {
            let x = Fl_Help_View_directory(self.inner.widget() as _);
            if x.is_null() {
                std::path::PathBuf::from("")
            } else {
                std::path::PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Return the filename
    pub fn filename(&self) -> std::path::PathBuf {
        unsafe {
            let x = Fl_Help_View_directory(self.inner.widget() as _);
            if x.is_null() {
                std::path::PathBuf::from("")
            } else {
                std::path::PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Find a string, returns the index
    pub fn find(&self, s: &str, start_from: i32) -> Option<usize> {
        unsafe {
            let s = CString::safe_new(s);
            let ret = Fl_Help_View_find(self.inner.widget() as _, s.as_ptr(), start_from);
            match ret {
                -1 => None,
                _ => Some(ret as usize),
            }
        }
    }

    /// Get the value of the widget
    pub fn value(&self) -> Option<String> {
        unsafe {
            let val = Fl_Help_View_value(self.inner.widget() as _);
            if val.is_null() {
                None
            } else {
                Some(CStr::from_ptr(val).to_string_lossy().to_string())
            }
        }
    }

    /// Set value of the widget
    pub fn set_value(&mut self, val: &str) {
        let val = CString::safe_new(val);
        unsafe { Fl_Help_View_set_value(self.inner.widget() as _, val.as_ptr()) }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        unsafe { Fl_Help_View_clear_selection(self.inner.widget() as _) }
    }

    /// Select all
    pub fn select_all(&mut self) {
        unsafe { Fl_Help_View_select_all(self.inner.widget() as _) }
    }

    /// Set the top line string
    pub fn set_top_line_string(&mut self, n: &str) {
        let n = CString::safe_new(n);
        unsafe { Fl_Help_View_set_topline(self.inner.widget() as _, n.as_ptr()) }
    }

    /// Set the top line position
    pub fn set_top_line(&mut self, line: i32) {
        unsafe { Fl_Help_View_set_topline2(self.inner.widget() as _, line) }
    }

    /// Get the top line position
    pub fn top_line(&self) -> i32 {
        unsafe { Fl_Help_View_topline(self.inner.widget() as _) }
    }

    /// Set the left line position
    pub fn set_left_line(&mut self, arg1: i32) {
        unsafe { Fl_Help_View_set_leftline(self.inner.widget() as _, arg1) }
    }

    /// Gets the current left line in pixels
    pub fn left_line(&self) -> i32 {
        unsafe { Fl_Help_View_leftline(self.inner.widget() as _) }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Help_View_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Help_View_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Help_View_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Help_View_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Help_View_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Help_View_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        unsafe { Fl_Help_View_scrollbar_size(self.inner.widget() as _) }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: i32) {
        unsafe { Fl_Help_View_set_scrollbar_size(self.inner.widget() as _, new_size) }
    }

    /// Load a view from a file or URI
    /// # Errors
    /// Errors on non-existent path
    pub fn load(&mut self, f: &str) -> Result<(), FltkError> {
        let f = CString::safe_new(f);
        unsafe {
            match Fl_Help_View_load(self.inner.widget() as _, f.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }
}
