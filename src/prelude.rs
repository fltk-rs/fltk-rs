pub use crate::color::Color;
pub use crate::fl;
pub use crate::font::Font;
use fltk_sys::widget::*;

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
    fn label(&self) -> String;
    fn as_widget_ptr(&self) -> *mut Fl_Widget;
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn redraw_label(&mut self);
    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn tooltip(&self) -> String;
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
    fn frame<T: WidgetType>(&self) -> T;
    fn set_frame<T: WidgetType>(&mut self, typ: T);
}

pub trait GroupTrait: WidgetTrait {
    fn begin(&self);
    fn end(&self);
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

pub trait WindowTrait {
    fn make_modal(&mut self, val: bool);
    fn fullscreen(&mut self, val: bool);
    fn make_current(&mut self);
}

pub trait InputTrait {
    fn value(&self) -> String;
    fn set_value(&mut self, val: &str);
    fn maximum_size(&self) -> usize;
    fn set_maximum_size(&mut self, val: usize);
    fn position(&self) -> i32;
    fn set_position(&mut self, val: i32);
    fn mark(&self) -> i32;
    fn set_mark(&mut self, val: i32);
    fn replace(&mut self, beg: usize, end: usize, val: &str);
    fn insert(&mut self, txt: &str);
    fn append(&mut self, txt: &str);
    fn copy(&mut self);
    fn undo(&mut self);
    fn cut(&mut self);
    fn text_font(&self) -> Font;
    fn set_text_font(&mut self, font: Font);
    fn text_color(&self) -> Color;
    fn set_text_color(&mut self, color: Color);
    fn text_size(&self) -> usize;
    fn set_text_size(&mut self, sz: usize);
    fn readonly(&self) -> bool;
    fn set_readonly(&mut self, val: bool);
    fn wrap(&self) -> bool;
    fn set_wrap(&mut self, val: bool);
}
