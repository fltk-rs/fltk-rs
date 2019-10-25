use crate::widget;
use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Button {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Button_new(
    x: raw::c_int,
    y: raw::c_int,
    width: raw::c_int,
    height: raw::c_int,
    title: *const raw::c_char,
    ) -> *mut Fl_Button;

    pub fn Fl_Button_add_callback(
        arg1: *mut Fl_Button,
        cb: widget::Fl_Callback,
        data: *mut raw::c_void,
    );

}