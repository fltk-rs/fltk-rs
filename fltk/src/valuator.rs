use crate::enums::{Color, Font, FrameType};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::valuator::*;
use std::{
    ffi::{CStr, CString},
    os::raw,
};

/// Creates a slider widget
#[derive(Debug)]
pub struct Slider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Slider, Fl_Slider);
crate::macros::widget::impl_widget_base!(Slider, Fl_Slider);
crate::macros::widget::impl_widget_default!(Slider);
crate::macros::valuator::impl_valuator_ext!(Slider, Fl_Slider);

/// Slider implementation
impl Slider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Creates a nice slider widget
#[derive(Debug)]
pub struct NiceSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(NiceSlider, Fl_Nice_Slider);
crate::macros::widget::impl_widget_base!(NiceSlider, Fl_Nice_Slider);
crate::macros::widget::impl_widget_default!(NiceSlider);
crate::macros::valuator::impl_valuator_ext!(NiceSlider, Fl_Nice_Slider);

/// Slider implementation
impl NiceSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Defines slider types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

crate::macros::widget::impl_widget_type!(SliderType);

/// Creates a dial widget
#[derive(Debug)]
pub struct Dial {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Dial, Fl_Dial);
crate::macros::widget::impl_widget_base!(Dial, Fl_Dial);
crate::macros::widget::impl_widget_default!(Dial);
crate::macros::valuator::impl_valuator_ext!(Dial, Fl_Dial);

impl Dial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (i32, i32) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner.widget() as _) };
        let angle2 = unsafe { Fl_Dial_angle2(self.inner.widget() as _) };
        (angle1 as i32, angle2 as i32)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: i32, angle2: i32) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner.widget() as _, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner.widget() as _, angle2 as _);
            }
        }
    }
}

/// Creates a line dial widget
#[derive(Debug)]
pub struct LineDial {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(LineDial, Fl_Line_Dial);
crate::macros::widget::impl_widget_base!(LineDial, Fl_Line_Dial);
crate::macros::widget::impl_widget_default!(LineDial);
crate::macros::valuator::impl_valuator_ext!(LineDial, Fl_Line_Dial);

impl LineDial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (i32, i32) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner.widget() as _) };
        let angle2 = unsafe { Fl_Dial_angle2(self.inner.widget() as _) };
        (angle1 as i32, angle2 as i32)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: i32, angle2: i32) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner.widget() as _, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner.widget() as _, angle2 as _);
            }
        }
    }
}

/// Defines dial types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DialType {
    /// Normal dial
    Normal = 0,
    /// Line dial
    Line = 1,
    /// Filled dial
    Fill = 2,
}

crate::macros::widget::impl_widget_type!(DialType);

/// Creates a counter widget
#[derive(Debug)]
pub struct Counter {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Counter, Fl_Counter);
crate::macros::widget::impl_widget_base!(Counter, Fl_Counter);
crate::macros::widget::impl_widget_default!(Counter);
crate::macros::valuator::impl_valuator_ext!(Counter, Fl_Counter);

/// Defines counter types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CounterType {
    /// Normal counter
    Normal = 0,
    /// Simple counter
    Simple = 1,
}

crate::macros::widget::impl_widget_type!(CounterType);

/// Creates a scrollbar widget
#[derive(Debug)]
pub struct Scrollbar {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Scrollbar, Fl_Scrollbar);
crate::macros::widget::impl_widget_base!(Scrollbar, Fl_Scrollbar);
crate::macros::widget::impl_widget_default!(Scrollbar);
crate::macros::valuator::impl_valuator_ext!(Scrollbar, Fl_Scrollbar);

impl Scrollbar {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Defines scrollbar types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

crate::macros::widget::impl_widget_type!(ScrollbarType);

/// Creates a roller widget
#[derive(Debug)]
pub struct Roller {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Roller, Fl_Roller);
crate::macros::widget::impl_widget_base!(Roller, Fl_Roller);
crate::macros::widget::impl_widget_default!(Roller);
crate::macros::valuator::impl_valuator_ext!(Roller, Fl_Roller);

/// Creates a value slider widget
#[derive(Debug)]
pub struct ValueSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ValueSlider, Fl_Value_Slider);
crate::macros::widget::impl_widget_base!(ValueSlider, Fl_Value_Slider);
crate::macros::widget::impl_widget_default!(ValueSlider);
crate::macros::valuator::impl_valuator_ext!(ValueSlider, Fl_Value_Slider);

impl ValueSlider {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Value_Slider_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Value_Slider_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Value_Slider_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Value_Slider_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Value_Slider_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Value_Slider_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Creates an adjuster widget
#[derive(Debug)]
pub struct Adjuster {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Adjuster, Fl_Adjuster);
crate::macros::widget::impl_widget_base!(Adjuster, Fl_Adjuster);
crate::macros::widget::impl_widget_default!(Adjuster);
crate::macros::valuator::impl_valuator_ext!(Adjuster, Fl_Adjuster);

/// Creates an value input widget, which takes a numeric value.
/// If a step is set, the value can be also dragged
#[derive(Debug)]
pub struct ValueInput {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ValueInput, Fl_Value_Input);
crate::macros::widget::impl_widget_base!(ValueInput, Fl_Value_Input);
crate::macros::widget::impl_widget_default!(ValueInput);
crate::macros::valuator::impl_valuator_ext!(ValueInput, Fl_Value_Input);

impl ValueInput {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Value_Input_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Value_Input_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Value_Input_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Value_Input_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Value_Input_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Value_Input_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Returns whether the user is allowed to drag the value outside the range.
    /// Default is true
    pub fn soft(&self) -> bool {
        unsafe { Fl_Value_Input_soft(self.inner.widget() as _) != 0 }
    }

    /// Set whether the user is allowed to drag the value outside the range
    pub fn set_soft(&mut self, val: bool) {
        unsafe { Fl_Value_Input_set_soft(self.inner.widget() as _, val as raw::c_char) }
    }
}

/// Creates an value output widget
#[derive(Debug)]
pub struct ValueOutput {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ValueOutput, Fl_Value_Output);
crate::macros::widget::impl_widget_base!(ValueOutput, Fl_Value_Output);
crate::macros::widget::impl_widget_default!(ValueOutput);
crate::macros::valuator::impl_valuator_ext!(ValueOutput, Fl_Value_Output);

impl ValueOutput {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Value_Output_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Value_Output_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Value_Output_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Value_Output_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Value_Output_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Value_Output_set_text_color(self.inner.widget() as _, color.bits()) }
    }
}

/// Creates a fill slider
#[derive(Debug)]
pub struct FillSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(FillSlider, Fl_Fill_Slider);
crate::macros::widget::impl_widget_base!(FillSlider, Fl_Fill_Slider);
crate::macros::widget::impl_widget_default!(FillSlider);
crate::macros::valuator::impl_valuator_ext!(FillSlider, Fl_Fill_Slider);

/// Creates a fill dial
#[derive(Debug)]
pub struct FillDial {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(FillDial, Fl_Fill_Dial);
crate::macros::widget::impl_widget_base!(FillDial, Fl_Fill_Dial);
crate::macros::widget::impl_widget_default!(FillDial);
crate::macros::valuator::impl_valuator_ext!(FillDial, Fl_Fill_Dial);

impl FillDial {
    /// Get the angles of the dial
    pub fn angles(&self) -> (i32, i32) {
        let angle1 = unsafe { Fl_Dial_angle1(self.inner.widget() as _) };
        let angle2 = unsafe { Fl_Dial_angle2(self.inner.widget() as _) };
        (angle1 as i32, angle2 as i32)
    }

    /// Set the angles of the dial
    pub fn set_angles(&mut self, angle1: i32, angle2: i32) {
        if angle1 <= 360 {
            unsafe {
                Fl_Dial_set_angle1(self.inner.widget() as _, angle1 as _);
            }
        }
        if angle2 <= 360 {
            unsafe {
                Fl_Dial_set_angle2(self.inner.widget() as _, angle2 as _);
            }
        }
    }
}

/// Creates a horizontal slider
#[derive(Debug)]
pub struct HorSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HorSlider, Fl_Hor_Slider);
crate::macros::widget::impl_widget_base!(HorSlider, Fl_Hor_Slider);
crate::macros::widget::impl_widget_default!(HorSlider);
crate::macros::valuator::impl_valuator_ext!(HorSlider, Fl_Hor_Slider);

/// Slider implementation
impl HorSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Creates a horizontal fill slider
#[derive(Debug)]
pub struct HorFillSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HorFillSlider, Fl_Hor_Fill_Slider);
crate::macros::widget::impl_widget_base!(HorFillSlider, Fl_Hor_Fill_Slider);
crate::macros::widget::impl_widget_default!(HorFillSlider);
crate::macros::valuator::impl_valuator_ext!(HorFillSlider, Fl_Hor_Fill_Slider);

/// Slider implementation
impl HorFillSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Creates a horizontal nice slider
#[derive(Debug)]
pub struct HorNiceSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HorNiceSlider, Fl_Hor_Nice_Slider);
crate::macros::widget::impl_widget_base!(HorNiceSlider, Fl_Hor_Nice_Slider);
crate::macros::widget::impl_widget_default!(HorNiceSlider);
crate::macros::valuator::impl_valuator_ext!(HorNiceSlider, Fl_Hor_Nice_Slider);

/// Slider implementation
impl HorNiceSlider {
    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}

/// Creates a horizontal value slider
#[derive(Debug)]
pub struct HorValueSlider {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HorValueSlider, Fl_Hor_Value_Slider);
crate::macros::widget::impl_widget_base!(HorValueSlider, Fl_Hor_Value_Slider);
crate::macros::widget::impl_widget_default!(HorValueSlider);
crate::macros::valuator::impl_valuator_ext!(HorValueSlider, Fl_Hor_Value_Slider);

impl HorValueSlider {
    /// Gets the text font
    pub fn text_font(&self) -> Font {
        unsafe { std::mem::transmute(Fl_Hor_Value_Slider_text_font(self.inner.widget() as _)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        unsafe { Fl_Hor_Value_Slider_set_text_font(self.inner.widget() as _, f.bits()) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Hor_Value_Slider_text_size(self.inner.widget() as _) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Hor_Value_Slider_set_text_size(self.inner.widget() as _, s) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        unsafe { std::mem::transmute(Fl_Hor_Value_Slider_text_color(self.inner.widget() as _)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Hor_Value_Slider_set_text_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the slider size as a fraction of the long axis
    pub fn slider_size(&self) -> f32 {
        unsafe { Fl_Slider_slider_size(self.inner.widget() as _) }
    }

    /// Set the slider size as a fraction of the long axis
    pub fn set_slider_size(&mut self, v: f32) {
        unsafe { Fl_Slider_set_slider_size(self.inner.widget() as _, v) }
    }

    /// Get the frame type of the slider box
    pub fn slider_frame(&self) -> FrameType {
        unsafe { FrameType::from_i32(Fl_Slider_slider_box(self.inner.widget() as _)) }
    }

    /// Set the frame type of the slider box
    pub fn set_slider_frame(&mut self, c: FrameType) {
        unsafe { Fl_Slider_set_slider_box(self.inner.widget() as _, c.as_i32()) }
    }
}
