#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Box {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Box_new(
    x: libc::c_int,
    y: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
    title: *const libc::c_char,
    ) -> *mut Fl_Box;

    pub fn Fl_Box_show(arg1: *mut Fl_Box);

    pub fn Fl_Box_hide(arg1: *mut Fl_Box);

    pub fn Fl_Box_set_label(
        arg1: *mut Fl_Box,
        title: *const libc::c_char,
    );

    pub fn Fl_Box_redraw(
        arg1: *mut Fl_Box,
    );


    pub fn Fl_Box_activate(arg1: *mut Fl_Box);


    pub fn Fl_Box_deactivate(arg1: *mut Fl_Box);


    pub fn Fl_Box_redraw_label(arg1: *mut Fl_Box);


    pub fn Fl_Box_resize(
        arg1: *mut Fl_Box,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );

    pub fn Fl_Box_tooltip(arg1: *mut Fl_Box) -> *const libc::c_char;

    pub fn Fl_Box_set_tooltip(arg1: *mut Fl_Box, txt: *const libc::c_char);

    pub fn Fl_Box_get_type(arg1: *mut Fl_Box) -> libc::c_int;

    pub fn Fl_Box_set_type(arg1: *mut Fl_Box, typ: libc::c_int); 

    pub fn Fl_Box_color(arg1: *mut Fl_Box) -> libc::c_int;

    pub fn Fl_Box_set_color(arg1: *mut Fl_Box, color: libc::c_int);

    pub fn Fl_Box_label_color(arg1: *mut Fl_Box) -> libc::c_int;

    pub fn Fl_Box_set_label_color(arg1: *mut Fl_Box, color: libc::c_int);

    pub fn Fl_Box_label_font(arg1: *mut Fl_Box) -> libc::c_int;

    pub fn Fl_Box_set_label_font(arg1: *mut Fl_Box, font: libc::c_int);

    pub fn Fl_Box_label_size(arg1: *mut Fl_Box) -> libc::c_int;

    pub fn Fl_Box_set_label_size(arg1: *mut Fl_Box, sz: libc::c_int);

    pub fn Fl_Box_label_type(arg1: *mut Fl_Box) -> libc::c_int;
    
    pub fn Fl_Box_set_label_type(arg1: *mut Fl_Box, typ: libc::c_int);

    pub fn Fl_Box_box(arg1: *mut Fl_Box) -> libc::c_int;
    
    pub fn Fl_Box_set_box(arg1: *mut Fl_Box, typ: libc::c_int);

}