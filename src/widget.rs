pub use crate::button;
pub use crate::prelude::*;
pub use crate::window;
use std::ffi;

#[derive(Debug, Clone)]
pub struct Widget {
    _inner: *mut fltk_sys::widget::Fl_Widget,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl Widget {
    pub fn as_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
        self._inner
    }
}

impl<W: WidgetTrait> From<W> for Widget {
    fn from(s: W) -> Self {
        let widg: *mut fltk_sys::widget::Fl_Widget = s.as_widget_ptr();
        Widget {
            _inner: widg,
            _x: s.x(),
            _y: s.y(),
            _width: s.width(),
            _height: s.height(),
            _title: ffi::CString::new(s.label().as_str()).unwrap(),
        }
    }
}
