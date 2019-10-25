use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Window {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Window_new(
        x: raw::c_int,
        y: raw::c_int,
        width: raw::c_int,
        height: raw::c_int,
        title: *const raw::c_char,
    ) -> *mut Fl_Window;

    pub fn Fl_Window_begin(arg1: *mut Fl_Window);

    pub fn Fl_Window_end(arg1: *mut Fl_Window);

    pub fn Fl_Window_show(arg1: *mut Fl_Window);
}
