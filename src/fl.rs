pub use crate::prelude::*;
use std::mem;

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
            use std::panic::{catch_unwind, AssertUnwindSafe};
            use std::process::abort;
            let a: *mut F = mem::transmute(data);
            let f = &mut *a;
            catch_unwind(AssertUnwindSafe(|| {
                f();
            }))
            .unwrap_or_else(|_| abort())
        }
        let a: *mut F = Box::into_raw(Box::new(cb));
        let data: *mut libc::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim::<F>);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}
