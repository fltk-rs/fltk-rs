pub use crate::prelude::*;
use std::{ffi, mem};

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    NoEvent = 0,
    Push,
    Released,
    Enter,
    Leave,
    Drag,
    Focus,
    Unfocus,
    KeyDown,
    KeyUp,
    Close,
    Move,
    Shortcut,
    Deactivate,
    Activate,
    Hide,
    Show,
    Paste,
    SelectionClear,
    MouseWheel,
}

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
        ffi::CString::from_raw(fltk_sys::fl::Fl_event_text() as *mut libc::c_char)
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

pub fn event_inside(arg1: *const fltk_sys::widget::Fl_Widget) -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_inside(arg1 as *mut libc::c_void) {
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

pub fn register_callback<W, F>(widget: &W, cb: F)
where
    W: WidgetTrait,
    F: FnMut(),
{
    unsafe {
        unsafe extern "C" fn shim<F>(
            _wid: *mut fltk_sys::widget::Fl_Widget,
            data: *mut libc::c_void,
        ) where
            F: FnMut(),
        {
            // use std::panic::{catch_unwind, AssertUnwindSafe};
            // use std::process::abort;
            let a: *mut F = mem::transmute(data);
            let f = &mut *a;
            // catch_unwind(AssertUnwindSafe(|| {
            //     f();
            // }))
            // .unwrap_or_else(|_| abort())
            f();
        }
        let a: *mut F = Box::into_raw(Box::new(cb));
        let data: *mut libc::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim::<F>);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}
