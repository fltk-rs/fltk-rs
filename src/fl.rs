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
    unsafe {
        ffi::CString::from_raw(fltk_sys::fl::Fl_event_text() as *mut raw::c_char)
            .into_string()
            .unwrap()
    }
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
        fltk_sys::fl::Fl_paste(widget.as_widget_ptr() as *mut raw::c_void, 1);
    }
}

pub fn set_callback<'a, W>(widget: &'a W, cb: Box<dyn FnMut() + 'a>)
where
    W: WidgetTrait,
{
    unsafe {
        unsafe extern "C" fn shim<'a>(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
            let f: &mut dyn FnMut() = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}