pub use crate::prelude::*;
use fltk_sys::widget::*;

/// An abstract type, shouldn't be instantiated in user code
#[derive(Debug, Clone)]
pub struct Widget {
    _inner: *mut Fl_Widget,
}

/// A conversion function for internal use
impl<W: WidgetTrait> From<W> for Widget {
    fn from(s: W) -> Self {
        let widg: *mut Fl_Widget = s.as_widget_ptr();
        Widget {
            _inner: widg,
        }
    }
}

impl Widget {
    pub fn from_raw(ptr: *mut Fl_Widget) -> Self {
        Widget {
            _inner: ptr,
        }
    }
}
