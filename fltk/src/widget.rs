//! Basic empty widget
//!
//! **Multithreaded** applications can call widget methods from non-ui
//! threads, but will need to call [`app::awake()`](`crate::app::awake`) to awaken
//! the ui thread's event loop.

use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::widget::*;
use std::ffi::{CStr, CString};

/// An empty widget. Instantiating a Widget requires that you at least add a draw callback for it to show anything
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// fn main() {
///     let a = app::App::default();
///     let mut win = window::Window::default();
///     let mut wid = widget::Widget::default();
///     win.end();
///     win.show();
///     wid.draw(|w| {
///         draw::draw_box(w.frame(), w.x(), w.y(), w.w(), w.h(), w.color());
///         draw::set_draw_color(enums::Color::Black); // for the text
///         draw::set_font(enums::Font::Helvetica, app::font_size());
///         draw::draw_text2(&w.label(), w.x(), w.y(), w.w(), w.h(), w.align());
///     });
///     a.run().unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct Widget {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Widget, Fl_Widget);
crate::macros::widget::impl_widget_base!(Widget, Fl_Widget);
crate::macros::widget::impl_widget_default!(Widget);

/// Widget Tracker
#[derive(Debug, Clone)]
pub struct WidgetTracker {
    inner: *mut Fl_Widget,
}

#[cfg(not(feature = "single-threaded"))]
unsafe impl Send for WidgetTracker {}

#[cfg(not(feature = "single-threaded"))]
unsafe impl Sync for WidgetTracker {}

impl Drop for WidgetTracker {
    fn drop(&mut self) {
        unsafe { fltk_sys::fl::Fl_watch_widget_pointer(self.inner as _) };
    }
}

impl WidgetTracker {
    /// Creates a new widget tracker
    pub fn new(w: *mut Fl_Widget) -> Self {
        assert!(!w.is_null());
        unsafe { fltk_sys::fl::Fl_watch_widget_pointer(w as _) };
        Self {
            inner: w,
        }
    }

    /// Checks if the wrapped widget was deleted
    pub fn deleted(&self) -> bool {
        self.inner.is_null()
    }

    /// Gets the wrapped widget
    pub fn widget(&self) -> *mut Fl_Widget {
        assert!(!self.inner.is_null());
        self.inner
    }
}
