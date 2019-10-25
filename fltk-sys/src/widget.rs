use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    _unused: [u8; 0],
}

pub type Fl_Callback = Option<
    unsafe extern "C" fn(widget: *mut Fl_Widget, data: *mut raw::c_void)
>;