#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}

pub type cfl_callback = Option<unsafe extern "C" fn(arg1: *mut Fl_Widget)>;

extern "C" {
    pub fn Fl_Widget_callback(arg1: *mut Fl_Widget, cb: cfl_callback);
}