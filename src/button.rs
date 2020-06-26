use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::button::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a normal button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct Button {
    _inner: *mut Fl_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the button type, which can be changed dynamically using the set_type function().
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ButtonType {
    Normal = 0,
    Toggle = 1,
    Radio = 102,
    Hidden = 3,
}

/// Creates a radio button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct RadioButton {
    _inner: *mut Fl_Radio_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioButton {
    /// Check whether a RadioButton is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Radio_Button_is_toggled(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the RadioButton is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Button_toggle(self._inner, val as i32) }
    }
}

/// Creates a radio round button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct RadioRoundButton {
    _inner: *mut Fl_Radio_Round_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioRoundButton {
    /// Check whether a RadioRoundButton is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Radio_Round_Button_is_toggled(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the RadioRoundButton is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Round_Button_toggle(self._inner, val as i32) }
    }
}

/// Creates a radio light button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct RadioLightButton {
    _inner: *mut Fl_Radio_Light_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RadioLightButton {
    /// Check whether a RadioLightButton is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Radio_Light_Button_is_toggled(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the RadioLightButton is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Radio_Light_Button_toggle(self._inner, val as i32) }
    }
}

/// Creates a round button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct RoundButton {
    _inner: *mut Fl_Round_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl RoundButton {
    /// Check whether a RoundButton is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Round_Button_is_toggled(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the RoundButton is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Round_Button_toggle(self._inner, val as i32) }
    }
}

/// Creates a check button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct CheckButton {
    _inner: *mut Fl_Check_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl CheckButton {
    /// Check whether a CheckButton is checked
    pub fn is_checked(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Check_Button_is_checked(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Set whether CheckButton is checked or not
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Check_Button_set_checked(self._inner, checked as i32);
        }
    }
}

/// Creates a toggle button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct ToggleButton {
    _inner: *mut Fl_Toggle_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ToggleButton {
    /// Check whether a ToggleButton is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Toggle_Button_is_toggled(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the ToggleButton is toggled or not
    pub fn toggle(&mut self, val: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Toggle_Button_toggle(self._inner, val as i32) }
    }
}

/// Creates a light button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct LightButton {
    _inner: *mut Fl_Light_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl LightButton {
    /// Check whether a LightButton is on
    pub fn is_on(&self) -> bool {
        unsafe {
            assert!(!self.was_deleted());
            match Fl_Light_Button_is_on(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Sets whether the LightButton is on or not
    pub fn turn_on(&mut self, on: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Light_Button_turn_on(self._inner, on as i32) }
    }
}

/// Creates a repeat button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct RepeatButton {
    _inner: *mut Fl_Repeat_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a return button
#[derive(WidgetExt, ButtonExt, Debug)]
pub struct ReturnButton {
    _inner: *mut Fl_Return_Button,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

#[cfg(test)]
mod button {
    use super::*;
    #[test]
    fn tooltip() {
        let mut but = Button::new(0, 0, 0, 0, "hello");
        but.set_tooltip("tooltip");
        assert!(but.tooltip().unwrap() == "tooltip");
    }
}
