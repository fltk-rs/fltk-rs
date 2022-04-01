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
///         if let Some(label) = w.label() {
///             draw::draw_text2(&label, w.x(), w.y(), w.w(), w.h(), w.align());
///         }
///     });
///     a.run().unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct Widget {
    inner: *mut Fl_Widget,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Widget, Fl_Widget);
crate::macros::widget::impl_widget_base!(Widget, Fl_Widget);

/// An alias exposing the Widget tracker
pub type WidgetTrackerPtr = *mut fltk_sys::fl::Fl_Widget_Tracker;
