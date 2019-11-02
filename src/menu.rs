pub use crate::prelude::*;
use fltk_sys::menu::*;
use std::{ffi, mem, os::raw, ptr};

#[derive(WidgetTrait, Debug, Clone)]
pub struct MenuBar {
    _inner: *mut Fl_Menu_Bar,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct MenuButton {
    _inner: *mut Fl_Menu_Button,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

#[derive(WidgetTrait, Debug, Clone)]
pub struct Choice {
    _inner: *mut Fl_Choice,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _title: ffi::CString,
}

impl MenuTrait for MenuBar {
    fn add<F>(&mut self, name: &str, shortcut: i32, cb: F, flag: i32) where F: FnMut() {
        let temp = ffi::CString::new(name).unwrap();
        unsafe {
            unsafe extern "C" fn shim<F>(_wid: *mut fltk_sys::menu::Fl_Widget, data: *mut raw::c_void)
            where
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
            let data: *mut raw::c_void = mem::transmute(a);
            let callback: fltk_sys::menu::Fl_Callback = Some(shim::<F>);
            fltk_sys::menu::Fl_Menu_Bar_add(self._inner, temp.as_ptr() as *const raw::c_char, shortcut, callback, data, flag)
        }
    }
}
