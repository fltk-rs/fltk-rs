use fltk_sys::{button, widget};
use std::os::raw;
use std::mem::transmute;

type VoidPtr = *mut raw::c_void;

pub struct Button {
    _button: *mut button::Fl_Button,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Button {
        unsafe {
            Button {
                _button: button::Fl_Button_new(x, y, width, height, title.as_ptr() as *const i8),
            }
        }
    }

    pub fn add_callback<F: FnMut(Button)>(&mut self, cb: &mut F) {
        type WidgetPtr = *mut widget::Fl_Widget;
        let data: VoidPtr = std::ptr::null_mut();
        let callback: *mut unsafe extern "C" fn(WidgetPtr, VoidPtr) = unsafe {transmute(cb)};
        let callback = unsafe {*callback};
        let opt = std::option::Option::from(callback);
        unsafe {
            button::Fl_Button_callback(self._button, opt, data)
        }
    }

    pub fn set_label(&mut self, title: & str) {
        unsafe {
            button::Fl_Button_set_label(self._button, title.as_ptr() as *const i8)
        }
    }

    pub fn redraw(&mut self) {
        unsafe {
            button::Fl_Button_redraw(self._button);
        }
    }
}