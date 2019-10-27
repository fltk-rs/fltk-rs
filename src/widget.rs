pub use crate::button;
pub use crate::window;
use std::ffi;
use std::mem;

pub struct Widget {
    _widget: *mut fltk_sys::widget::Fl_Widget,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl Widget {
    pub fn as_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
        self._widget
    }
}

pub trait WidgetTrait {
    fn new() -> Self;
    fn set(self, x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
    fn set_label(&mut self, title: &str);
    fn redraw(&mut self);
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn label(&self) -> ffi::CString;
    fn callback<W>(&mut self, cb: fn(&mut W))
    where
        W: WidgetTrait;
    fn add_callback(&mut self, cb: fn());
}

impl From<button::Button> for Widget {
    fn from(but: button::Button) -> Self {
        let widg: *mut fltk_sys::widget::Fl_Widget = unsafe {mem::transmute(but.as_ptr())};
        Widget {
            _widget: widg,
            _x: but.x(),
            _y: but.y(),
            _width: but.width(),
            _height: but.height(),
            _title: but.label(),
        }
    }
}