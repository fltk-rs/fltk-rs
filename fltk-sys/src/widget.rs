#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}

pub type Fl_Callback = Option<unsafe extern "C" fn(arg1: *mut Fl_Widget, data: *mut libc::c_void)>;

extern "C" {
    pub fn Fl_Widget_callback(arg1: *mut Fl_Widget, cb: Fl_Callback);
    pub fn Fl_Widget_callback_with_captures(arg1: *mut Fl_Widget, cb: Fl_Callback, data: *mut libc::c_void);
}