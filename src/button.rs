use crate::widget;
use fltk_sys;
use std::ffi;

pub struct Button {
    _button: *mut fltk_sys::button::Fl_Button,
}

impl Button {
    pub fn set_label(&mut self, title: &str) {
        let title = ffi::CString::new(title).unwrap();
        unsafe {
            fltk_sys::button::Fl_Button_set_label(
                self._button,
                title.as_ptr() as *const libc::c_char,
            )
        }
    }

    pub fn redraw(&mut self) {
        unsafe {
            fltk_sys::button::Fl_Button_redraw(self._button);
        }
    }
}

impl widget::Widget for Button {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Button {
        let title = ffi::CString::new(title).unwrap();
        unsafe {
            Button {
                _button: fltk_sys::button::Fl_Button_new(
                    x,
                    y,
                    width,
                    height,
                    title.as_ptr() as *const libc::c_char,
                ),
            }
        }
    }
}
