pub use crate::button;
pub use crate::window;
use std::{ffi, mem};

#[derive(Debug, Clone)]
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
    fn add_callback(&self, cb: fn());
    fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget;
}

impl From<button::Button> for Widget {
    fn from(but: button::Button) -> Self {
        let widg: *mut fltk_sys::widget::Fl_Widget = unsafe { mem::transmute(but.as_ptr()) };
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

pub fn register_callback<W, F>(widget: &W, cb: F)
where
    W: WidgetTrait,
    F: FnMut(),
{
    unsafe {
        unsafe extern "C" fn shim<F>(
            _wid: *mut fltk_sys::widget::Fl_Widget,
            data: *mut libc::c_void,
        ) where
            F: FnMut(),
        {
            use std::panic::{catch_unwind, AssertUnwindSafe};
            use std::process::abort;
            // let a: *mut &mut dyn FnMut() = mem::transmute(data);
            let a: *mut F = mem::transmute(data);
            let f = &mut *a;
            catch_unwind(AssertUnwindSafe(|| {
                f();
            }))
            .unwrap_or_else(|_| abort())
        }
        let a: *mut F = Box::into_raw(Box::new(cb));
        let data: *mut libc::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim::<F>);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}
