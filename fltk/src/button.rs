use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::button::*;
use std::ffi::{CStr, CString};

/// Creates a normal button
#[derive(Debug)]
pub struct Button {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Button, Fl_Button);
crate::macros::widget::impl_widget_base!(Button, Fl_Button);
crate::macros::widget::impl_widget_default!(Button);
crate::macros::button::impl_button_ext!(Button, Fl_Button);

impl Button {
    /// Set whether a button is compact
    pub fn set_compact(&mut self, flag: bool) {
        unsafe {
            Fl_Button_set_compact(self.inner.widget() as _, flag as _)
        }
    }

    /// Get whether a button is compact
    pub fn compact(&self) -> bool {
        unsafe {
            Fl_Button_compact(self.inner.widget() as _) != 0
        }
    }
}

/// Defines the button type, which can be changed dynamically using the `set_type()`.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

crate::macros::widget::impl_widget_type!(ButtonType);

/// Creates a radio button.
/// Radio meaning only one can be toggled in the same group
#[derive(Debug)]
pub struct RadioButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(RadioButton, Fl_Radio_Button);
crate::macros::widget::impl_widget_base!(RadioButton, Fl_Radio_Button);
crate::macros::widget::impl_widget_default!(RadioButton);
crate::macros::button::impl_button_ext!(RadioButton, Fl_Radio_Button);

impl RadioButton {
    /// Check whether a `RadioButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe { Fl_Radio_Button_is_toggled(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `RadioButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        unsafe { Fl_Radio_Button_toggle(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a radio round button.
/// Radio meaning only one can be toggled in the same group
#[derive(Debug)]
pub struct RadioRoundButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(RadioRoundButton, Fl_Radio_Round_Button);
crate::macros::widget::impl_widget_base!(RadioRoundButton, Fl_Radio_Round_Button);
crate::macros::widget::impl_widget_default!(RadioRoundButton);
crate::macros::button::impl_button_ext!(RadioRoundButton, Fl_Radio_Round_Button);

impl RadioRoundButton {
    /// Check whether a `RadioRoundButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe { Fl_Radio_Round_Button_is_toggled(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `RadioRoundButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        unsafe { Fl_Radio_Round_Button_toggle(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a radio light button.
/// Radio meaning only one can be toggled in the same group
#[derive(Debug)]
pub struct RadioLightButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(RadioLightButton, Fl_Radio_Light_Button);
crate::macros::widget::impl_widget_base!(RadioLightButton, Fl_Radio_Light_Button);
crate::macros::widget::impl_widget_default!(RadioLightButton);
crate::macros::button::impl_button_ext!(RadioLightButton, Fl_Radio_Light_Button);

impl RadioLightButton {
    /// Check whether a `RadioLightButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe { Fl_Radio_Light_Button_is_toggled(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `RadioLightButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        unsafe { Fl_Radio_Light_Button_toggle(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a round button
#[derive(Debug)]
pub struct RoundButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(RoundButton, Fl_Round_Button);
crate::macros::widget::impl_widget_base!(RoundButton, Fl_Round_Button);
crate::macros::widget::impl_widget_default!(RoundButton);
crate::macros::button::impl_button_ext!(RoundButton, Fl_Round_Button);

impl RoundButton {
    /// Check whether a `RoundButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe { Fl_Round_Button_is_toggled(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `RoundButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        unsafe { Fl_Round_Button_toggle(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a check button
#[derive(Debug)]
pub struct CheckButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(CheckButton, Fl_Check_Button);
crate::macros::widget::impl_widget_base!(CheckButton, Fl_Check_Button);
crate::macros::widget::impl_widget_default!(CheckButton);
crate::macros::button::impl_button_ext!(CheckButton, Fl_Check_Button);

impl CheckButton {
    /// Check whether a `CheckButton` is checked
    pub fn is_checked(&self) -> bool {
        unsafe { Fl_Check_Button_is_checked(self.inner.widget() as _) != 0 }
    }

    /// Set whether `CheckButton` is checked or not
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            Fl_Check_Button_set_checked(self.inner.widget() as _, checked as i32);
        }
    }
}

/// Creates a toggle button
#[derive(Debug)]
pub struct ToggleButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ToggleButton, Fl_Toggle_Button);
crate::macros::widget::impl_widget_base!(ToggleButton, Fl_Toggle_Button);
crate::macros::widget::impl_widget_default!(ToggleButton);
crate::macros::button::impl_button_ext!(ToggleButton, Fl_Toggle_Button);

impl ToggleButton {
    /// Check whether a `ToggleButton` is toggled
    pub fn is_toggled(&self) -> bool {
        unsafe { Fl_Toggle_Button_is_toggled(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `ToggleButton` is toggled or not
    pub fn toggle(&mut self, val: bool) {
        unsafe { Fl_Toggle_Button_toggle(self.inner.widget() as _, val as i32) }
    }
}

/// Creates a light button
#[derive(Debug)]
pub struct LightButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(LightButton, Fl_Light_Button);
crate::macros::widget::impl_widget_base!(LightButton, Fl_Light_Button);
crate::macros::widget::impl_widget_default!(LightButton);
crate::macros::button::impl_button_ext!(LightButton, Fl_Light_Button);

impl LightButton {
    /// Check whether a `LightButton` is on
    pub fn is_on(&self) -> bool {
        unsafe { Fl_Light_Button_is_on(self.inner.widget() as _) != 0 }
    }

    /// Sets whether the `LightButton` is on or not
    pub fn turn_on(&mut self, on: bool) {
        unsafe { Fl_Light_Button_turn_on(self.inner.widget() as _, on as i32) }
    }
}

/// Creates a repeat button
#[derive(Debug)]
pub struct RepeatButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(RepeatButton, Fl_Repeat_Button);
crate::macros::widget::impl_widget_base!(RepeatButton, Fl_Repeat_Button);
crate::macros::widget::impl_widget_default!(RepeatButton);
crate::macros::button::impl_button_ext!(RepeatButton, Fl_Repeat_Button);

/// Creates a return button
#[derive(Debug)]
pub struct ReturnButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ReturnButton, Fl_Return_Button);
crate::macros::widget::impl_widget_base!(ReturnButton, Fl_Return_Button);
crate::macros::widget::impl_widget_default!(ReturnButton);
crate::macros::button::impl_button_ext!(ReturnButton, Fl_Return_Button);

/// Creates a Shortcut button
#[derive(Debug)]
pub struct ShortcutButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(ShortcutButton, Fl_Shortcut_Button);
crate::macros::widget::impl_widget_base!(ShortcutButton, Fl_Shortcut_Button);
crate::macros::widget::impl_widget_default!(ShortcutButton);
crate::macros::button::impl_button_ext!(ShortcutButton, Fl_Shortcut_Button);

impl ShortcutButton {
    /// Gets the Shortcut button value
    pub fn value(&self) -> crate::enums::Shortcut {
        unsafe { std::mem::transmute(Fl_Shortcut_Button_value(self.inner.widget() as _)) }
    }

    /// Sets the Shortcut button value
    pub fn set_value(&mut self, val: crate::enums::Shortcut) {
        unsafe { Fl_Shortcut_Button_set_value(self.inner.widget() as _, val.bits()) }
    }

    /// Gets the Shortcut button default value
    pub fn default_value(&self) -> crate::enums::Shortcut {
        unsafe { std::mem::transmute(Fl_Shortcut_Button_default_value(self.inner.widget() as _)) }
    }

    /// Sets the Shortcut button default value
    pub fn set_default_value(&mut self, val: crate::enums::Shortcut) {
        unsafe { Fl_Shortcut_Button_set_default_value(self.inner.widget() as _, val.bits()) }
    }

    /// Clears the default shortcut
    pub fn default_clear(&mut self) {
        unsafe { Fl_Shortcut_Button_default_clear(self.inner.widget() as _) }
    }
}
