use crate::widget::Fl_Widget;
use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Button {
    _unused: [u8; 0],
}

pub type Fl_Button_cb = extern "C" fn(widget: *mut Fl_Widget);

extern "C" {
    pub fn Fl_Button_new(
    x: raw::c_int,
    y: raw::c_int,
    width: raw::c_int,
    height: raw::c_int,
    title: *const raw::c_char,
    ) -> *mut Fl_Button;

}