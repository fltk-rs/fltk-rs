pub use crate::prelude::*;
use std::{ffi, mem, ptr};

#[derive(Debug, Clone)]
pub struct Frame {
    _inner: *mut fltk_sys::frame::Fl_Box,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum FrameType {
    NoBox = 0,
    FlatBox,
    UpBox,
    DownBox,
    UpFrame,
    DownFrame,
    ThinUpBox,
    ThinDownBox,
    ThinUpFrame,
    ThinDownFrame,
    EngraveBox,
    EmbossedBox,
    EngravedFrame,
    EmbossedFrame,
    BorderBox,
    ShadowBox,
    BorderFrame,
    ShadowFrame,
    RoundedBox,
    RShadowBox,
    RoundedFrame,
    RFlatBox,
    RoundUpBox,
    RoundDownBox,
    DiamondUpBox,
    DiamondDownBox,
    OvalBox,
    OShadowBox,
    OvalFrame,
    OFlatFrame,
    PlasticUpBox,
    PlasticDownBox,
    PlasticUpFrame,
    PlasticDownFrame,
    PlasticThinUpBox,
    PlasticThinDownBox,
    PlasticRoundUpBox,
    PlasticRoundDownBox,
    GtkUpBox,
    GtkDownBox,
    GtkUpFrame,
    GtkDownFrame,
    GtkThinUpBox,
    GtkThinDownBox,
    GtkThinUpFrame,
    GtkThinDownFrame,
    GtkRoundUpFrame,
    GtkRoundDownFrame,
    GleamUpBox,
    GleamDownBox,
    GleamUpFrame,
    GleamDownFrame,
    GleamThinUpBox,
    GleamThinDownBox,
    GleamRoundUpBox,
    GleamRoundDownBox,
    FreeBoxType,
}

impl WidgetType for FrameType {
    fn to_int(self) -> i32 {
        self as i32
    }

    fn from_i32(val: i32) -> FrameType {
        unsafe { mem::transmute(val) }
    }
}

impl Frame {
    pub fn as_ptr(&self) -> *mut fltk_sys::frame::Fl_Box {
        self._inner
    }
}

impl WidgetTrait for Frame {
    fn new() -> Frame {
        Frame {
            _inner: ptr::null_mut(),
            _x: 0,
            _y: 0,
            _width: 0,
            _height: 0,
            _title: ffi::CString::new("").unwrap(),
        }
    }

    fn set(mut self, x: i32, y: i32, width: i32, height: i32, title: &str) -> Frame {
        self._x = x;
        self._y = y;
        self._width = width;
        self._height = height;
        self._title = ffi::CString::new(title).unwrap();
        self._inner = unsafe {
            fltk_sys::frame::Fl_Box_new(
                self._x,
                self._y,
                self._width,
                self._height,
                self._title.as_ptr() as *const libc::c_char,
            )
        };
        self
    }

    fn set_label(&mut self, title: &str) {
        self._title = ffi::CString::new(title).unwrap();
        unsafe {
            fltk_sys::frame::Fl_Box_set_label(
                self._inner,
                self._title.as_ptr() as *const libc::c_char,
            )
        }
    }

    fn redraw(&mut self) {
        unsafe {
            fltk_sys::frame::Fl_Box_redraw(self._inner);
        }
    }

    fn show(&mut self) {
        unsafe { fltk_sys::frame::Fl_Box_show(self._inner) }
    }

    fn hide(&mut self) {
        unsafe { fltk_sys::frame::Fl_Box_hide(self._inner) }
    }

    fn x(&self) -> i32 {
        self._x
    }

    fn y(&self) -> i32 {
        self._y
    }

    fn width(&self) -> i32 {
        self._width
    }

    fn height(&self) -> i32 {
        self._height
    }

    fn label(&self) -> ffi::CString {
        self._title.clone()
    }

    fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
        unsafe { mem::transmute(self._inner) }
    }

    fn activate(&mut self) {
        unsafe { fltk_sys::frame::Fl_Box_activate(self._inner) }
    }

    fn deactivate(&mut self) {
        unsafe { fltk_sys::frame::Fl_Box_deactivate(self._inner) }
    }

    fn redraw_label(&mut self) {
        unsafe { fltk_sys::frame::Fl_Box_redraw_label(self._inner) }
    }

    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { fltk_sys::frame::Fl_Box_resize(self._inner, x, y, width, height) }
    }

    fn tooltip(&self) -> ffi::CString {
        unsafe {
            ffi::CString::from_raw(fltk_sys::frame::Fl_Box_tooltip(self._inner) as *mut libc::c_char)
        }
    }

    fn set_tooltip(&mut self, txt: &str) {
        let txt = ffi::CString::new(txt).unwrap();
        unsafe {
            fltk_sys::frame::Fl_Box_set_tooltip(self._inner, txt.as_ptr() as *const libc::c_char)
        }
    }

    fn get_type<T: WidgetType>(&self) -> T {
        unsafe { T::from_i32(fltk_sys::frame::Fl_Box_get_type(self._inner)) }
    }

    fn set_type<T: WidgetType>(&mut self, typ: T) {
        unsafe {
            fltk_sys::frame::Fl_Box_set_type(self._inner, typ.to_int());
        }
    }

    fn color(&self) -> Color {
        unsafe { mem::transmute(fltk_sys::frame::Fl_Box_color(self._inner)) }
    }

    fn set_color(&mut self, color: Color) {
        unsafe { fltk_sys::frame::Fl_Box_set_color(self._inner, color as i32) }
    }

    fn label_color(&self) -> Color {
        unsafe { mem::transmute(fltk_sys::frame::Fl_Box_label_color(self._inner)) }
    }

    fn set_label_color(&mut self, color: Color) {
        unsafe { fltk_sys::frame::Fl_Box_set_label_color(self._inner, color as i32) }
    }

    fn label_font(&self) -> Font {
        unsafe { mem::transmute(fltk_sys::frame::Fl_Box_label_font(self._inner)) }
    }

    fn set_label_font(&mut self, font: Font) {
        unsafe { fltk_sys::frame::Fl_Box_set_label_color(self._inner, font as i32) }
    }

    fn label_size(&self) -> usize {
        unsafe { fltk_sys::frame::Fl_Box_label_size(self._inner) as usize }
    }

    fn set_label_size(&mut self, sz: usize) {
        unsafe { fltk_sys::frame::Fl_Box_set_label_size(self._inner, sz as i32) }
    }

    fn label_type<T: WidgetType>(&self) -> T {
        unsafe { T::from_i32(fltk_sys::frame::Fl_Box_label_type(self._inner)) }
    }

    fn set_label_type<T: WidgetType>(&mut self, typ: T) {
        unsafe {
            fltk_sys::frame::Fl_Box_set_label_type(self._inner, typ.to_int());
        }
    }

    fn get_box<T: WidgetType>(&self) -> T {
        unsafe { T::from_i32(fltk_sys::frame::Fl_Box_box(self._inner)) }
    }

    fn set_box<T: WidgetType>(&mut self, typ: T) {
        unsafe {
            fltk_sys::frame::Fl_Box_set_box(self._inner, typ.to_int());
        }
    }
}
