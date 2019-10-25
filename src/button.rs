use fltk_sys::{button, widget};

pub struct Button {
    _button: *mut button::Fl_Button,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &'static str) -> Button {
        unsafe {
            Button {
                _button: button::Fl_Button_new(x, y, width, height, title.as_ptr() as *const i8),
            }
        }
    }

    pub fn add_callback<F: FnMut(Button, widget::Data)>(&mut self, cb: F, data: widget::Data) {
        let mut buf = data as *mut _ as *mut std::os::raw::c_void;
        let opt = std::option::Option::from(cb as *mut unsafe extern "C" fn(*mut widget::Fl_Widget, *mut std::os::raw::c_void));
        unsafe {
            button::Fl_Button_add_callback(self._button, opt, buf)
        }
    }
}