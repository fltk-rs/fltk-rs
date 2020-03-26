pub use crate::prelude::*;
use std::{ffi, mem, os::raw};

/// Runs the event loop
fn run() -> Result<(), FltkError> {
    unsafe {
        match fltk_sys::fl::Fl_run() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Locks the main UI thread
fn lock() -> Result<(), FltkError> {
    unsafe {
        match fltk_sys::fl::Fl_lock() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToLock)),
        }
    }
}

/// sets the scheme of the application
fn set_scheme(scheme: AppScheme) {
    let name_str = match scheme {
        AppScheme::Base => "base",
        AppScheme::Gtk => "gtk+",
        AppScheme::Gleam => "gleam",
        AppScheme::Plastic => "plastic",
    };
    let name_str= std::ffi::CString::new(name_str).unwrap();
    unsafe {
        fltk_sys::fl::Fl_set_scheme(name_str.into_raw() as *const raw::c_char)
    }
}

/// Unlocks the main UI thread
// #[allow(dead_code)]
// fn unlock() {
//     unsafe {
//         fltk_sys::fl::Fl_unlock();
//     }
// }

// pub fn awake<'a>(cb: Box<dyn FnMut() + 'a>) {
//     unsafe {
//         unsafe extern "C" fn shim<'a>(data: *mut raw::c_void) {
//             let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
//             let f: &mut (dyn FnMut() + 'a) = &mut **a;
//             f();
//         }
//         let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
//         let data: *mut raw::c_void = mem::transmute(a);
//         let callback: fltk_sys::fl::Fl_Awake_Handler = Some(shim);
//         fltk_sys::fl::Fl_awake(callback, data);
//     }
// }

/// Basic Application struct, used to instatiate, set the scheme and run the event loop
#[derive(Debug, Copy, Clone)]
pub struct App {}

impl App {
    /// Instantiates an App type
    pub fn default() -> App {
        App {}
    }
    
    /// Sets the scheme of the application
    pub fn set_scheme(self, scheme: AppScheme) -> App {
        fl::set_scheme(scheme);
        self
    }
    
    /// Runs the event loop
    pub fn run(&self) -> Result<(), FltkError> {
        fl::lock()?;
        return fl::run();
    }
    // pub fn awake<'a>(&'a self, cb: Box<dyn FnMut() + 'a>) {
    //     unsafe {
    //         unsafe extern "C" fn shim<'a>(data: *mut raw::c_void) {
    //             let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
    //             let f: &mut (dyn FnMut() + 'a) = &mut **a;
    //             f();
    //         }
    //         let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
    //         let data: *mut raw::c_void = mem::transmute(a);
    //         let callback: fltk_sys::fl::Fl_Awake_Handler = Some(shim);
    //         fltk_sys::fl::Fl_awake(callback, data);
    //     }
    // }
}

/// Returns the latest captured event
pub fn event() -> Event {
    unsafe {
        let x = fltk_sys::fl::Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}

/// Returns the presed key
pub fn event_key() -> i32 {
    unsafe {
        let x = fltk_sys::fl::Fl_event_key();
        x
    }
}

/// Returns a textual representation of the latest event
pub fn event_text() -> String {
    unsafe {
        ffi::CString::from_raw(fltk_sys::fl::Fl_event_text() as *mut raw::c_char)
            .to_string_lossy().to_string()
    }
}

/// Gets the character representation of the keyboard event
pub fn event_char() -> char {
    event_key() as u8 as char
}

/// Returns the captured button event
pub fn event_button() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_button() }
}

/// Returns the number of clicks
pub fn event_clicks() -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_clicks() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the x and y coordinates of the captured event
pub fn event_coords() -> (i32, i32) {
    unsafe { (fltk_sys::fl::Fl_event_dx(), fltk_sys::fl::Fl_event_dy()) }
}

/// Determines whether an event was a click
pub fn event_is_click() -> bool {
    unsafe {
        match fltk_sys::fl::Fl_event_is_click() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the duration of an event
pub fn event_length() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_length() }
}

/// Returns the state of the event
pub fn event_state() -> i32 {
    unsafe { fltk_sys::fl::Fl_event_state() }
}

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe {
        (
            (fltk_sys::fl::Fl_screen_w() as f64 / 0.96).into(),
            (fltk_sys::fl::Fl_screen_h() as f64 / 0.96).into(),
        )
    }
}

/// Used for widgets implementing the InputTrait, pastes content from the clipboard
pub fn paste<T>(widget: T)
where
    T: WidgetTrait + InputTrait,
{
    unsafe {
        fltk_sys::fl::Fl_paste(widget.as_widget_ptr() as *mut raw::c_void, 1);
    }
}

/// Sets the callback of a widget
pub fn set_callback<'a, W>(widget: &'a W, cb: Box<dyn FnMut() + 'a>)
where
    W: WidgetTrait,
{
    unsafe {
        unsafe extern "C" fn shim<'a>(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
            let f: &mut (dyn FnMut() + 'a) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}