pub use crate::widget;
use std::{ffi, mem, ptr};

#[derive(Debug, Clone)]
pub struct Window {
    _inner: *mut fltk_sys::window::Fl_Window,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl Window {
    pub fn begin(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_begin(self._inner) }
    }
    pub fn end(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_end(self._inner) }
    }
    pub fn show(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_show(self._inner) }
    }

    pub fn as_ptr(&self) -> *mut fltk_sys::window::Fl_Window {
        self._inner
    }
}

impl widget::WidgetTrait for Window {
    fn new() -> Window {
        Window {
            _inner: ptr::null_mut(),
            _x: 0,
            _y: 0,
            _width: 0,
            _height: 0,
            _title: ffi::CString::new("").unwrap(),
        }
    }

    fn set(mut self, x: i32, y: i32, width: i32, height: i32, title: &str) -> Window {
        // let title = ffi::CString::new(title).unwrap();
        self._x = x;
        self._y = y;
        self._width = width;
        self._height = height;
        self._title = ffi::CString::new(title).unwrap();
        self._inner = unsafe {
            fltk_sys::window::Fl_Window_new(
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
            fltk_sys::window::Fl_Window_set_label(
                self._inner,
                self._title.as_ptr() as *const libc::c_char,
            )
        }
    }

    fn redraw(&mut self) {
        unsafe {
            fltk_sys::window::Fl_Window_redraw(self._inner);
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

    fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
        unsafe { mem::transmute(self._inner) }
    }

    fn add_callback(&self, cb: fn()) {
        unsafe {
            let widget: *mut fltk_sys::widget::Fl_Widget = mem::transmute(self._inner);
            let callback: unsafe extern "C" fn(
                *mut fltk_sys::widget::Fl_Widget,
                *mut libc::c_void,
            ) = mem::transmute(cb);
            fltk_sys::widget::Fl_Widget_callback(widget, Option::from(callback));
        }
    }
}
