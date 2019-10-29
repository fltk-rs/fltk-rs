use crate::group::GroupTrait;
use crate::widget::WidgetTrait;
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
    pub fn as_ptr(&self) -> *mut fltk_sys::window::Fl_Window {
        self._inner
    }

    pub fn make_modal(&self, val: bool) {
        unsafe { fltk_sys::window::Fl_Window_make_modal(self._inner, val as u32) }
    }

    pub fn fullscreen(&self, val: bool) {
        unsafe { fltk_sys::window::Fl_Window_fullscreen(self._inner, val as u32) }
    }

    pub fn make_current(&self) {
        unsafe { fltk_sys::window::Fl_Window_make_current(self._inner) }
    }
}

impl GroupTrait for Window {
    fn begin(&self) {
        unsafe { fltk_sys::window::Fl_Window_begin(self._inner) }
    }

    fn end(&self) {
        unsafe { fltk_sys::window::Fl_Window_end(self._inner) }
    }
}

impl WidgetTrait for Window {
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

    fn show(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_show(self._inner) }
    }

    fn hide(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_hide(self._inner) }
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
        unsafe { fltk_sys::window::Fl_Window_activate(self._inner) }
    }

    fn deactivate(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_deactivate(self._inner) }
    }

    fn redraw_label(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_redraw_label(self._inner) }
    }

    fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { fltk_sys::window::Fl_Window_resize(self._inner, x, y, width, height) }
    }

    fn set_tooltip(&mut self, txt: &str) {
        let txt = ffi::CString::new(txt).unwrap();
        unsafe {
            fltk_sys::window::Fl_Window_set_tooltip(
                self._inner,
                txt.as_ptr() as *const libc::c_char,
            )
        }
    }
}
