#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Window {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Window_new(
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        title: *const libc::c_char,
    ) -> *mut Fl_Window;

    pub fn Fl_Window_begin(arg1: *mut Fl_Window);

    pub fn Fl_Window_end(arg1: *mut Fl_Window);

    pub fn Fl_Window_show(arg1: *mut Fl_Window);
}
