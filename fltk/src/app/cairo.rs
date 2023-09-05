use crate::prelude::WidgetExt;
use fltk_sys::fl::*;
use std::os::raw::c_void;

/// Makes the cairo context current
pub fn make_current(w: &impl WidgetExt) -> *mut c_void {
    unsafe { Fl_cairo_make_current(w.window().unwrap().as_widget_ptr() as _) }
}

/// Sets autolink
pub fn set_autolink_context(alink: bool) {
    unsafe { Fl_set_cairo_autolink_context(alink as i32) }
}

/// Gets autolink
pub fn autolink_context() -> bool {
    unsafe { Fl_cairo_autolink_context() != 0 }
}

/// Gets the cairo context
pub fn cc() -> *mut c_void {
    unsafe { Fl_cairo_cc() }
}

/// Sets the cairo context
/// # Safety
/// Doesn't check the passed context for validity
pub unsafe fn set_cc(c: *mut c_void, own: bool) {
    Fl_set_cairo_cc(c, own as i32)
}

/// Flushes the cairo context
/// # Safety
/// Doesn't check the passed context for validity
pub unsafe fn flush(c: *mut c_void) {
    Fl_cairo_flush(c)
}
