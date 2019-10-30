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

    pub fn Fl_Button_show(arg1: *mut Fl_Button);

    pub fn Fl_Button_hide(arg1: *mut Fl_Button);

    pub fn Fl_Button_set_label(
        arg1: *mut Fl_Button,
        title: *const libc::c_char,
    );

    pub fn Fl_Button_redraw(
        arg1: *mut Fl_Button,
    );


    pub fn Fl_Button_activate(arg1: *mut Fl_Button);


    pub fn Fl_Button_deactivate(arg1: *mut Fl_Button);


    pub fn Fl_Button_redraw_label(arg1: *mut Fl_Button);


    pub fn Fl_Button_resize(
        arg1: *mut Fl_Button,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );

    pub fn Fl_Button_tooltip(arg1: *mut Fl_Button) -> *const libc::c_char;

    pub fn Fl_Button_set_tooltip(arg1: *mut Fl_Button, txt: *const libc::c_char);

    pub fn Fl_Button_get_type(arg1: *mut Fl_Button) -> libc::c_int;

    pub fn Fl_Button_set_type(arg1: *mut Fl_Button, typ: libc::c_int); 

    pub fn Fl_Button_color(arg1: *mut Fl_Button) -> libc::c_int;

    pub fn Fl_Button_set_color(arg1: *mut Fl_Button, color: libc::c_int);

    pub fn Fl_Button_label_color(arg1: *mut Fl_Button) -> libc::c_int;

    pub fn Fl_Button_set_label_color(arg1: *mut Fl_Button, color: libc::c_int);

    pub fn Fl_Button_label_font(arg1: *mut Fl_Button) -> libc::c_int;

    pub fn Fl_Button_set_label_font(arg1: *mut Fl_Button, font: libc::c_int);

    pub fn Fl_Button_label_size(arg1: *mut Fl_Button) -> libc::c_int;

    pub fn Fl_Button_set_label_size(arg1: *mut Fl_Button, sz: libc::c_int);

    pub fn Fl_Button_label_type(arg1: *mut Fl_Button) -> libc::c_int;
    
    pub fn Fl_Button_set_label_type(arg1: *mut Fl_Button, typ: libc::c_int);

}