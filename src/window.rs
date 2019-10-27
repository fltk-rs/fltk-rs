pub use crate::widget;
use std::ffi;

pub struct Window {
    _window: *mut fltk_sys::window::Fl_Window,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl Window {
    pub fn begin(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_begin(self._window) }
    }
    pub fn end(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_end(self._window) }
    }
    pub fn show(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_show(self._window) }
    }

    pub fn as_ptr(&self) -> *mut fltk_sys::window::Fl_Window {
        self._window
    }
}

impl widget::WidgetTrait for Window {
    fn new() -> Window {
        Window {
            _window: std::ptr::null_mut(),
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
        self._window = unsafe {
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
                self._window,
                self._title.as_ptr() as *const libc::c_char,
            )
        }
    }

    fn redraw(&mut self) {
        unsafe {
            fltk_sys::window::Fl_Window_redraw(self._window);
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
            let widget: *mut fltk_sys::widget::Fl_Widget = std::mem::transmute(self._window);
            let callback: unsafe extern "C" fn(*mut fltk_sys::widget::Fl_Widget) =
                std::mem::transmute(cb);
            fltk_sys::widget::Fl_Widget_callback(widget, Option::from(callback));
        }
    }

    fn add_callback(&mut self, cb: fn()) {
        unsafe {
            let widget: *mut fltk_sys::widget::Fl_Widget = std::mem::transmute(self._window);
            let callback: unsafe extern "C" fn(*mut fltk_sys::widget::Fl_Widget) =
                std::mem::transmute(cb);
            fltk_sys::widget::Fl_Widget_callback(widget, Option::from(callback));
        }
    }
}
