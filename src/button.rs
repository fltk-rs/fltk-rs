pub use crate::widget;
use fltk_sys;
use std::ffi;

pub struct Button {
    _button: *mut fltk_sys::button::Fl_Button,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl Button {
    pub fn as_ptr(&self) -> *mut fltk_sys::button::Fl_Button {
        self._button
    }

    pub fn handle(&self, event: i32) {
        unsafe {
            fltk_sys::button::Fl_Button_handle(self._button, event);
        }
    }
}

impl widget::WidgetTrait for Button {
    fn new() -> Button {
        Button {
            _button: std::ptr::null_mut(),
            _x: 0,
            _y: 0,
            _width: 0,
            _height: 0,
            _title: ffi::CString::new("").unwrap(),
        }
    }

    fn set(mut self, x: i32, y: i32, width: i32, height: i32, title: &str) -> Button {
        // let title = ffi::CString::new(title).unwrap();
        self._x = x;
        self._y = y;
        self._width = width;
        self._height = height;
        self._title = ffi::CString::new(title).unwrap();
        self._button = unsafe {
            fltk_sys::button::Fl_Button_new(
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
            fltk_sys::button::Fl_Button_set_label(
                self._button,
                self._title.as_ptr() as *const libc::c_char,
            )
        }
    }

    fn redraw(&mut self) {
        unsafe {
            fltk_sys::button::Fl_Button_redraw(self._button);
        }
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

    fn callback<W>(&mut self, cb: fn(&mut W))
    where
        W: widget::WidgetTrait,
    {
        unsafe {
            let widget: *mut fltk_sys::widget::Fl_Widget = std::mem::transmute(self._button);
            let callback: unsafe extern "C" fn(*mut fltk_sys::widget::Fl_Widget) =
                std::mem::transmute(cb);
            fltk_sys::widget::Fl_Widget_callback(widget, Option::from(callback));
        }
    }

    fn add_callback(&mut self, cb: fn()) {
        unsafe {
            let widget: *mut fltk_sys::widget::Fl_Widget = std::mem::transmute(self._button);
            let callback: unsafe extern "C" fn(*mut fltk_sys::widget::Fl_Widget) =
                std::mem::transmute(cb);
            fltk_sys::widget::Fl_Widget_callback(widget, Option::from(callback));
        }
    }
}
