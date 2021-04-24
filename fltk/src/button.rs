use crate::enums::{
    Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType, Shortcut,
};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::button::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a normal button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct Button {
    inner: *mut Fl_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the button type, which can be changed dynamically using the `set_type()`.
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ButtonType {
    /// Normal button
    Normal = 0,
    /// Toggle button
    Toggle = 1,
    /// Radio button
    Radio = 102,
    /// Hidden button
    Hidden = 3,
}

/// Creates a radio button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct RadioButton {
    inner: *mut Fl_Radio_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioButton {
    /// Check whether a `RadioButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Radio_Button_is_toggled(self.inner) != 0
        }
    }

    /// Sets whether the `RadioButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Button_toggle(self.inner, val as i32) }
    }
}

/// Creates a radio round button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct RadioRoundButton {
    inner: *mut Fl_Radio_Round_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioRoundButton {
    /// Check whether a `RadioRoundButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Radio_Round_Button_is_toggled(self.inner) != 0
        }
    }

    /// Sets whether the `RadioRoundButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Round_Button_toggle(self.inner, val as i32) }
    }
}

/// Creates a radio light button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct RadioLightButton {
    inner: *mut Fl_Radio_Light_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioLightButton {
    /// Check whether a `RadioLightButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Radio_Light_Button_is_toggled(self.inner) != 0
        }
    }

    /// Sets whether the `RadioLightButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Light_Button_toggle(self.inner, val as i32) }
    }
}

/// Creates a round button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct RoundButton {
    inner: *mut Fl_Round_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RoundButton {
    /// Check whether a `RoundButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Round_Button_is_toggled(self.inner) != 0
        }
    }

    /// Sets whether the `RoundButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Round_Button_toggle(self.inner, val as i32) }
    }
}

/// Creates a check button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct CheckButton {
    inner: *mut Fl_Check_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl CheckButton {
    /// Check whether a `CheckButton` is checked
    pub fn is_checked(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Check_Button_is_checked(self.inner) != 0
        }
    }

    /// Set whether `CheckButton` is checked or not
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Check_Button_set_checked(self.inner, checked as i32);
        }
    }
}

/// Creates a toggle button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct ToggleButton {
    inner: *mut Fl_Toggle_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ToggleButton {
    /// Check whether a `ToggleButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Toggle_Button_is_toggled(self.inner) != 0
        }
    }

    /// Sets whether the `ToggleButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Toggle_Button_toggle(self.inner, val as i32) }
    }
}

/// Creates a light button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct LightButton {
    inner: *mut Fl_Light_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl LightButton {
    /// Check whether a `LightButton` is on
    pub fn is_on(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Light_Button_is_on(self.inner) != 0
        }
    }

    /// Sets whether the `LightButton` is on or not
    pub fn turn_on(&mut self, on: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Light_Button_turn_on(self.inner, on as i32) }
    }
}

/// Creates a repeat button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct RepeatButton {
    inner: *mut Fl_Repeat_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a return button
#[derive(WidgetBase, WidgetExt, ButtonExt, Debug)]
pub struct ReturnButton {
    inner: *mut Fl_Return_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}
