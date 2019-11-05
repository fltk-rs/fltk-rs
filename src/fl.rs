pub use crate::prelude::*;
use std::{ffi, mem, os::raw};

pub fn run() {
    unsafe {
        fltk_sys::fl::Fl_run();
    }
}

pub fn event() -> Event {
    unsafe {
        let x = fltk_sys::fl::Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}

pub fn event_key() -> i32 {
    unsafe {
        let x = fltk_sys::fl::Fl_event_key();
        x
    }
}

pub fn event_text() -> String {
    unsafe { String::from(ffi::CStr::from_ptr(fltk_sys::fl::Fl_event_text()).to_string_lossy()) }
}

pub fn event_button() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_button() }
}

pub fn event_clicks() -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_clicks() {
            0 => false,
            _ => true,
        }
    }
}

pub fn event_coords() -> (i32, i32) {
    unsafe { (fltk_sys::fl::Fl_event_dx(), fltk_sys::fl::Fl_event_dy()) }
}

pub fn event_inside(arg1: *const fltk_sys::widget::Fl_Widget) -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_inside(arg1 as *mut raw::c_void) {
            0 => false,
            _ => true,
        }
    }
}

pub fn event_is_click() -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_is_click() {
            0 => false,
            _ => true,
        }
    }
}

pub fn event_length() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_length() }
}

pub fn event_state() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_state() }
}

pub fn belowmouse() -> *mut fltk_sys::widget::Fl_Widget {
    unsafe { mem::transmute(fltk_sys::fl::Fl_belowmouse()) }
}

pub fn screen_size() -> (f64, f64) {
    unsafe {
        (
            (fltk_sys::fl::Fl_screen_w() as f64 / 0.96).into(),
            (fltk_sys::fl::Fl_screen_h() as f64 / 0.96).into(),
        )
    }
}

pub fn paste<T>(widget: T)
where
    T: WidgetTrait + InputTrait,
{
    unsafe {
        fltk_sys::fl::Fl_paste(widget.as_widget_ptr() as *mut raw::c_void);
    }
}

pub fn set_callback<W>(widget: &W, cb: &mut dyn FnMut())
where
    W: WidgetTrait,
{
    unsafe {
        unsafe extern "C" fn shim(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            use std::panic::{catch_unwind, AssertUnwindSafe};
            use std::process::abort;
            let a: *mut &mut dyn FnMut() = mem::transmute(data);
            let f = AssertUnwindSafe(a.read());
            catch_unwind(f).unwrap_or_else(|_| abort());
        }
        let a: *mut &mut dyn FnMut() = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}

// unsafe {
//     unsafe extern "C" fn shim<F>(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void)
//     where
//         F: FnMut(),
//     {
//         // use std::panic::{catch_unwind, AssertUnwindSafe};
//         // use std::process::abort;
//         let a: *mut F = mem::transmute(data);
//         let f = &mut *a;
//         // catch_unwind(AssertUnwindSafe(|| {
//         //     f();
//         // }))
//         // .unwrap_or_else(|_| abort())
//         f();
//     }
//     let a: *mut F = Box::into_raw(Box::new(cb));
//     let data: *mut raw::c_void = mem::transmute(a);
//     let callback: fltk_sys::widget::Fl_Callback = Some(shim::<F>);
//     fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
// }

// unsafe {
//     unsafe extern "C" fn shim(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
//         use std::panic::{catch_unwind, AssertUnwindSafe};
//         use std::process::abort;
//         let a: *mut &mut dyn FnMut() = mem::transmute(data);
//         let f = AssertUnwindSafe(a.read());
//         catch_unwind(f).unwrap_or_else(|_| abort());
//     }
//     let a: *mut &mut dyn FnMut() = Box::into_raw(Box::new(cb));
//     let data: *mut raw::c_void = mem::transmute(a);
//     let callback: fltk_sys::widget::Fl_Callback = Some(shim);
//     fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
// }
