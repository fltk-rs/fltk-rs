pub use crate::fl;
use std::ffi;

pub trait WidgetTrait {
    fn new() -> Self;
    fn set(self, x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
    fn set_label(&mut self, title: &str);
    fn redraw(&mut self);
    fn show(&mut self);
    fn hide(&mut self);
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn label(&self) -> ffi::CString;
    fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget;
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn redraw_label(&mut self);
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn set_tooltip(&mut self, txt: &str);
}

pub trait GroupTrait {
    fn begin(&self);
    fn end(&self);
}