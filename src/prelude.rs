pub use crate::fl;
pub use crate::color::Color;
pub use crate::font::Font;
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
    fn tooltip(&self) -> ffi::CString;
    fn set_tooltip(&mut self, txt: &str);
    fn get_type<T: WidgetType>(&self) -> T;
    fn set_type<T: WidgetType>(&mut self, typ: T);
    fn color(&self) -> Color;
    fn set_color(&mut self, color: Color);
    fn label_color(&self) -> Color;
    fn set_label_color(&mut self, color: Color);
    fn label_font(&self) -> Font;
    fn set_label_font(&mut self, font: Font);
    fn label_size(&self) -> usize;
    fn set_label_size(&mut self, sz: usize);
    fn label_type<T: WidgetType>(&self) -> T;
    fn set_label_type<T: WidgetType>(&mut self, typ: T);
}

pub trait GroupTrait {
    fn begin(&self);
    fn end(&self);
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}