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
    inner: *mut Fl_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Slider implementation
impl Slider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner, c as i32) }
    }
}

/// Creates a nice slider widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct NiceSlider {
    inner: *mut Fl_Nice_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Slider implementation
impl NiceSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Defines slider types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum SliderType {
    /// Vertical slider
    Vertical = 0,
    /// Horizontal slider
    Horizontal = 1,
    /// Vertical fill slider
    VerticalFill = 2,
    /// Horizontal fill slider
    HorizontalFill = 3,
    /// Vertical nice slider
    VerticalNice = 4,
    /// Horizontal nice slider
    HorizontalNice = 5,
}

/// Creates a dial widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Dial {
    inner: *mut Fl_Dial,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Dial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (u16, u16) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner) };
        let angle2 = unsafe { Fl_Dial_angle1(self.inner) };
        (angle1 as u16, angle2 as u16)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: u16, angle2: u16) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner, angle2 as _);
            }
        }
    }
}

/// Creates a line dial widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct LineDial {
    inner: *mut Fl_Line_Dial,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl LineDial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (u16, u16) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner as _) };
        let angle2 = unsafe { Fl_Dial_angle1(self.inner as _) };
        (angle1 as u16, angle2 as u16)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: u16, angle2: u16) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner as _, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner as _, angle2 as _);
            }
        }
    }
}

/// Defines dial types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum DialType {
    /// Normal dial
    Normal = 0,
    /// Line dial
    Line = 1,
    /// Filled dial
    Fill = 2,
}

/// Creates a counter widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Counter {
    inner: *mut Fl_Counter,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines counter types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum CounterType {
    /// Normal counter
    Normal = 0,
    /// Simple counter
    Simple = 1,
}

/// Creates a scrollbar widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Scrollbar {
    inner: *mut Fl_Scrollbar,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Scrollbar {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Defines scrollbar types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ScrollbarType {
    /// Vertical scrollbar
    Vertical = 0,
    /// Horizontal scrollbar
    Horizontal = 1,
    /// Vertical fill scrollbar
    VerticalFill = 2,
    /// Horizontal fill scrollbar
    HorizontalFill = 3,
    /// Vertical nice scrollbar
    VerticalNice = 4,
    /// Horizontal nice scrollbar
    HorizontalNice = 5,
}

/// Creates a roller widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Roller {
    inner: *mut Fl_Roller,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a value slider widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueSlider {
    inner: *mut Fl_Value_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ValueSlider {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Slider_text_font(self.inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Slider_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Slider_text_size(self.inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Slider_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Slider_text_color(self.inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Slider_set_text_color(self.inner, color.bits() as u32) }
    }

    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Creates an adjuster widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct Adjuster {
    inner: *mut Fl_Adjuster,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an value input widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueInput {
    inner: *mut Fl_Value_Input,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ValueInput {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Input_text_font(self.inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Input_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Input_text_size(self.inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Input_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Input_text_color(self.inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Input_set_text_color(self.inner, color.bits() as u32) }
    }
}

/// Creates an value output widget
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct ValueOutput {
    inner: *mut Fl_Value_Output,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ValueOutput {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Output_text_font(self.inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Output_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Output_text_size(self.inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Output_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Value_Output_text_color(self.inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Value_Output_set_text_color(self.inner, color.bits() as u32) }
    }
}

/// Creates a fill slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct FillSlider {
    inner: *mut Fl_Fill_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a fill dial
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct FillDial {
    inner: *mut Fl_Fill_Dial,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl FillDial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (u16, u16) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner as _) };
        let angle2 = unsafe { Fl_Dial_angle1(self.inner as _) };
        (angle1 as u16, angle2 as u16)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: u16, angle2: u16) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner as _, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner as _, angle2 as _);
            }
        }
    }
}

/// Creates a horizontal slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorSlider {
    inner: *mut Fl_Hor_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Slider implementation
impl HorSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Creates a horizontal fill slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorFillSlider {
    inner: *mut Fl_Hor_Fill_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Slider implementation
impl HorFillSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Creates a horizontal nice slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorNiceSlider {
    inner: *mut Fl_Hor_Nice_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Slider implementation
impl HorNiceSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}

/// Creates a horizontal value slider
#[derive(WidgetBase, WidgetExt, ValuatorExt, Debug)]
pub struct HorValueSlider {
    inner: *mut Fl_Hor_Value_Slider,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl HorValueSlider {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Hor_Value_Slider_text_font(self.inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Hor_Value_Slider_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Hor_Value_Slider_text_size(self.inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Hor_Value_Slider_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Hor_Value_Slider_text_color(self.inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Hor_Value_Slider_set_text_color(self.inner, color.bits() as u32) }
    }

    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { mem::transmute(Fl_Slider_slider_box(self.inner as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner as _, c as i32) }
    }
}
