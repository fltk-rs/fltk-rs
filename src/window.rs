pub struct Window {
    _window: *mut fltk_sys::cfl_window,
}

impl Window {
    pub fn new(width: i32, height: i32, title: &'static str) -> Window {
        unsafe {
            Window {
                _window: fltk_sys::cfl_window_new(width, height, title.as_ptr() as *const i8),
            }
        }
    }
    pub fn end(&mut self) {
        unsafe { fltk_sys::cfl_window_end(self._window) }
    }
    pub fn show(&mut self) {
        unsafe { fltk_sys::cfl_window_show(self._window) }   
    }
}