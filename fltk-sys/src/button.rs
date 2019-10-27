#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Button {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Button_new(
    x: libc::c_int,
    y: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
    title: *const libc::c_char,
    ) -> *mut Fl_Button;

    pub fn Fl_Button_set_label(
        arg1: *mut Fl_Button,
        title: *const libc::c_char,
    );

    pub fn Fl_Button_redraw(
        arg1: *mut Fl_Button,
    );

    pub fn Fl_Button_handle (
        arg1: *mut Fl_Button,
        event: libc::c_int,
    ) -> libc::c_int;

}