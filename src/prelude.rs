pub use crate::enums::*;
pub use crate::fl;
pub use crate::menu::*;
use fltk_sys::widget::*;
use std::os::raw;

pub trait WidgetTrait {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
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
    fn changed(&self) -> bool;
    fn set_changed(&mut self);
    fn clear_changed(&mut self);
    fn align(&self) -> Align;
    fn set_align(&mut self, align: Align);
    fn set_image<Image: ImageTrait>(&mut self, image: Image);
    fn set_callback<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>);
    fn handle(&mut self, ev: Event) -> bool;
}

pub trait GroupTrait: WidgetTrait {
    fn begin(&self);
    fn end(&self);
    fn find<Widget: WidgetTrait>(&self, widget: &Widget) -> usize;
    fn add<Widget: WidgetTrait>(&mut self, widget: &Widget);
    fn insert<Widget: WidgetTrait>(&mut self, widget: &Widget, index: usize);
    fn remove(&mut self, index: usize);
    fn clear(&mut self);
    fn children(&self) -> usize;
    fn make_resizable<Widget: WidgetTrait>(&self, widget: &Widget);
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

pub trait WindowTrait: GroupTrait {
    fn make_modal(&mut self, val: bool);
    fn fullscreen(&mut self, val: bool);
    fn make_current(&mut self);
    fn set_icon<Image: ImageTrait>(&mut self, image: Image);
}

pub trait InputTrait: WidgetTrait  {
    fn value(&self) -> String;
    fn set_value(&self, val: &str);
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

pub trait MenuTrait: WidgetTrait  {
    fn get_item(&self, name: &str) -> crate::menu::MenuItem;
    fn text_font(&self) -> Font;
    fn set_text_font(&mut self, c: Font);
    fn text_size(&self) -> usize;
    fn set_text_size(&mut self, c: usize);
    fn text_color(&self) -> Color;
    fn set_text_color(&mut self, c: Color);
    fn add<'a>(&'a mut self, name: &str, shortcut: i32, flag: MenuFlag, cb: Box<dyn FnMut() + 'a>);
}

pub trait ValuatorTrait: WidgetTrait  {
    fn set_bounds(&mut self, a: f64, b: f64);
    fn minimum(&self) -> f64;
    fn set_minimum(&mut self, a: f64);
    fn maximum(&self) -> f64;
    fn set_maximum(&mut self, a: f64);
    fn set_range(&mut self, a: f64, b: f64);
    fn set_step(&mut self, a: f64, b: i32);
    fn step(&self) -> f64;
    fn set_precision(&mut self, digits: i32);
    fn value(&self) -> f64;
    fn set_value(&mut self, arg2: f64);
    fn format(&mut self, arg2: &str);
    fn round(&self, arg2: f64) -> f64;
    fn clamp(&self, arg2: f64) -> f64;
    fn increment(&mut self, arg2: f64, arg3: i32) -> f64;
}

pub trait ImageTrait {
    fn new(path: std::path::PathBuf) -> Self;
    fn draw(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn as_ptr(&self) -> *mut raw::c_void;
}