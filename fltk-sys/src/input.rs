#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Input {
    _unused: [u8; 0],
}

extern "C" {
    pub fn Fl_Input_new(
    x: libc::c_int,
    y: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
    title: *const libc::c_char,
    ) -> *mut Fl_Input;

    pub fn Fl_Input_show(arg1: *mut Fl_Input);

    pub fn Fl_Input_hide(arg1: *mut Fl_Input);

    pub fn Fl_Input_set_label(
        arg1: *mut Fl_Input,
        title: *const libc::c_char,
    );

    pub fn Fl_Input_redraw(
        arg1: *mut Fl_Input,
    );


    pub fn Fl_Input_activate(arg1: *mut Fl_Input);


    pub fn Fl_Input_deactivate(arg1: *mut Fl_Input);


    pub fn Fl_Input_redraw_label(arg1: *mut Fl_Input);


    pub fn Fl_Input_resize(
        arg1: *mut Fl_Input,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    );

    pub fn Fl_Input_tooltip(arg1: *mut Fl_Input) -> *const libc::c_char;

    pub fn Fl_Input_set_tooltip(arg1: *mut Fl_Input, txt: *const libc::c_char);

    pub fn Fl_Input_get_type(arg1: *mut Fl_Input) -> libc::c_int;

    pub fn Fl_Input_set_type(arg1: *mut Fl_Input, typ: libc::c_int); 

    pub fn Fl_Input_color(arg1: *mut Fl_Input) -> libc::c_int;

    pub fn Fl_Input_set_color(arg1: *mut Fl_Input, color: libc::c_int);

    pub fn Fl_Input_label_color(arg1: *mut Fl_Input) -> libc::c_int;

    pub fn Fl_Input_set_label_color(arg1: *mut Fl_Input, color: libc::c_int);

    pub fn Fl_Input_label_font(arg1: *mut Fl_Input) -> libc::c_int;

    pub fn Fl_Input_set_label_font(arg1: *mut Fl_Input, font: libc::c_int);

    pub fn Fl_Input_label_size(arg1: *mut Fl_Input) -> libc::c_int;

    pub fn Fl_Input_set_label_size(arg1: *mut Fl_Input, sz: libc::c_int);

    pub fn Fl_Input_label_type(arg1: *mut Fl_Input) -> libc::c_int;
    
    pub fn Fl_Input_set_label_type(arg1: *mut Fl_Input, typ: libc::c_int);

    pub fn Fl_Input_box(arg1: *mut Fl_Input) -> libc::c_int;
    
    pub fn Fl_Input_set_box(arg1: *mut Fl_Input, typ: libc::c_int);

}