use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::group::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a widget group
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Group {
    _inner: *mut Fl_Group,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a widget pack
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Pack {
    _inner: *mut Fl_Pack,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines pack types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum PackType {
    Vertical = 0,
    Horizontal = 1,
}

/// Creates a scroll group
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Scroll {
    _inner: *mut Fl_Scroll,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines Scroll types
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum ScrollType {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
    AlwaysOn = 4,
    HorizontalAlways = 5,
    VerticalAlways = 6,
    BothAlways = 7,
}

impl Scroll {
    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> impl ValuatorExt {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_scrollbar(self._inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> impl ValuatorExt {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Scroll_hscrollbar(self._inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the x position
    pub fn xposition(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_xposition(self._inner) as u32 }
    }

    /// Returns the y position
    pub fn yposition(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_yposition(self._inner) as u32 }
    }

    /// Scrolls from ```from``` to ```to```
    pub fn scroll_to(&mut self, from: u32, to: u32) {
        debug_assert!(
            from <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        debug_assert!(
            to <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scroll_to(self._inner, from as i32, to as i32) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_scrollbar_size(self._inner) as u32 }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: u32) {
        debug_assert!(
            new_size <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Scroll_set_scrollbar_size(self._inner, new_size as i32) }
    }
}

/// Creates a tab which can contain widgets
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Tabs {
    _inner: *mut Fl_Tabs,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Tabs {
    /// Gets the currently visible group
    pub fn value(&mut self) -> Option<impl GroupBase> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_value(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Sets the currently visible group
    pub fn set_value<Grp: GroupBase>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_value(
                self._inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the tab group for the tab the user has currently down-clicked
    pub fn push(&self) -> Option<impl GroupBase> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Tabs_push(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(Group::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// This is called by the tab widget's handle() method to set the tab group widget the user last pushed
    pub fn set_push<Grp: GroupBase>(&mut self, w: &Grp) -> Result<(), FltkError> {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Tabs_set_push(
                self._inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            ) {
                0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                _ => Ok(()),
            }
        }
    }

    /// Returns the position and size available to be used by its children
    pub fn client_area(&mut self) -> (i32, i32, i32, i32) {
        assert!(!self.was_deleted());
        unsafe {
            let mut i1 = 0;
            let mut i2 = 0;
            let mut i3 = 0;
            let mut i4 = 0;
            Fl_Tabs_client_area(self._inner, &mut i1, &mut i2, &mut i3, &mut i4);
            (i1, i2, i3, i4)
        }
    }

    /// Sets the tab label alignment
    pub fn set_tab_align(&mut self, a: Align) {
        assert!(!self.was_deleted());
        unsafe { Fl_Tabs_set_tab_align(self._inner, a as i32) }
    }

    /// Gets the tab label alignment.
    pub fn tab_align(&self) -> Align {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Tabs_tab_align(self._inner)) }
    }
}

/// Creates a tile which can contain widgets
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Tile {
    _inner: *mut Fl_Tile,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a wizard widget
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct Wizard {
    _inner: *mut Fl_Wizard,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Wizard {
    /// Gets the next view of the wizard
    pub fn next(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_next(self._inner) }
    }

    /// Gets the previous view of the wizard
    pub fn prev(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Wizard_prev(self._inner) }
    }

    /// Gets the underlying widget of the current view
    pub fn current_widget(&mut self) -> Box<dyn WidgetBase> {
        unsafe {
            assert!(!self.was_deleted());
            Box::new(Widget::from_raw(
                Fl_Wizard_value(self._inner) as *mut fltk_sys::widget::Fl_Widget
            ))
        }
    }

    /// Sets the underlying widget of the current view
    pub fn set_current_widget<W: WidgetBase>(&mut self, w: &W) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Wizard_set_value(
                self._inner,
                w.as_widget_ptr() as *mut fltk_sys::group::Fl_Widget,
            )
        }
    }
}

/// Creates a color chooser widget
#[derive(WidgetBase, WidgetExt, GroupBase, GroupExt, Debug)]
pub struct ColorChooser {
    _inner: *mut Fl_Color_Chooser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl ColorChooser {
    pub fn rgb_color(&self) -> (u8, u8, u8) {
        unsafe {
            assert!(!self.was_deleted());
            let r = (Fl_Color_Chooser_r(self._inner) * 255.0) as u8;
            let g = (Fl_Color_Chooser_g(self._inner) * 255.0) as u8;
            let b = (Fl_Color_Chooser_b(self._inner) * 255.0) as u8;
            (r, g, b)
        }
    }
    pub fn hex_color(&self) -> u32 {
        assert!(!self.was_deleted());
        let c = self.rgb_color();
        let x = Color::from_rgb(c.0, c.1, c.2);
        x.to_u32()
    }
}

impl Pack {
    pub fn spacing(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Pack_spacing(self._inner) }
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        unsafe {
            assert!(!self.was_deleted());
            Fl_Pack_set_spacing(self._inner, spacing);
        }
    }
}
