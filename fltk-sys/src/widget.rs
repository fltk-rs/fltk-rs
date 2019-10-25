use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}

pub type Fl_Callback = Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut raw::c_void),
>;

extern "C" {
        pub fn Fl_Widget_callback(
        widget: *mut Fl_Widget,
        cb: Fl_Callback,
        data: *mut ::std::os::raw::c_void,
    );
}