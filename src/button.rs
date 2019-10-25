pub struct Button {
    _button: *mut fltk_sys::button::Fl_Button,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &'static str) -> Button {
        unsafe {
            Button {
                _button: fltk_sys::button::Fl_Button_new(x, y, width, height, title.as_ptr() as *const i8),
            }
        }
    }
}