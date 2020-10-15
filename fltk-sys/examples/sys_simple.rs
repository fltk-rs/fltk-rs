use fltk_sys::*;
use std::os::raw::*;

unsafe extern "C" fn cb(_wid: *mut button::Fl_Widget, data: *mut c_void) {
    let frame = data as *mut frame::Fl_Box;
    frame::Fl_Box_set_label(frame, "Hello World\0".as_ptr() as *const _);
}

fn main() {
    unsafe {
        fl::Fl_init_all();
        image::Fl_register_images();
        fl::Fl_lock();
        let win = window::Fl_Window_new(100, 100, 400, 300, "Window\0".as_ptr() as *const _);
        let frame = frame::Fl_Box_new(0, 0, 400, 200, std::ptr::null());
        let but = button::Fl_Button_new(160, 220, 80, 40, "Click\0".as_ptr() as *const _);
        window::Fl_Window_end(win);
        window::Fl_Window_show(win);

        button::Fl_Button_set_callback(but, Some(cb), frame as *mut _);

        fl::Fl_run();
    }
}