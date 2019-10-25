use std::os::raw;

extern "C" {
    pub fn Fl_run() -> raw::c_int;
}
