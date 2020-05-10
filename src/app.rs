pub use crate::enums::*;
use crate::prelude::*;
use fltk_sys::fl::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Runs the event loop
fn run() -> Result<(), FltkError> {
    unsafe {
        match Fl_run() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Locks the main UI thread
pub fn lock() -> Result<(), FltkError> {
    unsafe {
        match Fl_lock() {
            0 => Ok(()),
            _ => return Err(FltkError::Internal(FltkErrorKind::FailedToLock)),
        }
    }
}

/// Set the app scheme
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AppScheme {
    /// Base fltk scheming
    Base,
    /// inspired by the Aqua user interface on Mac OS X
    Plastic,
    /// inspired by the GTK+ theme
    Gtk,
    /// inspired by the Clearlooks Glossy scheme
    Gleam,
}

/// sets the scheme of the application
fn set_scheme(scheme: AppScheme) {
    let name_str = match scheme {
        AppScheme::Base => "base",
        AppScheme::Gtk => "gtk+",
        AppScheme::Gleam => "gleam",
        AppScheme::Plastic => "plastic",
    };
    let name_str = CString::new(name_str).unwrap();
    unsafe { Fl_set_scheme(name_str.into_raw() as *const raw::c_char) }
}

/// Unlocks the main UI thread
#[allow(dead_code)]
pub fn unlock() {
    unsafe {
        Fl_unlock();
    }
}

/// Awakens the main UI thread with a callback
pub fn awake(cb: Box<dyn FnMut()>) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = mem::transmute(data);
            let f: &mut (dyn FnMut()) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: Fl_Awake_Handler = Some(shim);
        Fl_awake(callback, data);
    }
}

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
        set_scheme(scheme);
        self
    }

    /// Runs the event loop
    pub fn run(&self) -> Result<(), FltkError> {
        lock()?;
        return run();
    }

    /// Wait for incoming messages
    pub fn wait(&self) -> bool {
        let ret = lock();
        if ret.is_err() {
            return false;
        }
        wait()
    }

    /// Awakens the main UI thread with a callback
    pub fn awake(&self, cb: Box<dyn FnMut()>) {
        unsafe {
            unsafe extern "C" fn shim(data: *mut raw::c_void) {
                let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                let f: &mut (dyn FnMut()) = &mut **a;
                f();
            }
            let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = mem::transmute(a);
            let callback: Fl_Awake_Handler = Some(shim);
            Fl_awake(callback, data);
        }
    }
}

/// Returns the latest captured event
pub fn event() -> Event {
    unsafe {
        let x = Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}

/// Returns the presed key
pub fn event_key() -> Key {
    unsafe {
        let x = Fl_event_key();
        mem::transmute(x)
    }
}

/// Returns a textual representation of the latest event
pub fn event_text() -> String {
    unsafe {
        let text = Fl_event_text();
        if text.is_null() {
            String::from("")
        } else {
            CString::from_raw(text as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }
}

/// Returns the captured button event
pub fn event_button() -> i32 {
    unsafe { Fl_event_button() }
}

/// Returns the number of clicks
pub fn event_clicks() -> bool {
    unsafe {
        match Fl_event_clicks() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the x and y coordinates of the captured event
pub fn event_coords() -> (i32, i32) {
    unsafe { (Fl_event_x(), Fl_event_y()) }
}

/// Determines whether an event was a click
pub fn event_is_click() -> bool {
    unsafe {
        match Fl_event_is_click() {
            0 => false,
            _ => true,
        }
    }
}

/// Returns the duration of an event
pub fn event_length() -> u32 {
    unsafe { Fl_event_length() as u32 }
}

/// Returns the state of the event
pub fn event_state() -> Shortcut {
    unsafe { mem::transmute(Fl_event_state()) }
}

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe {
        (
            (Fl_screen_w() as f64 / 0.96).into(),
            (Fl_screen_h() as f64 / 0.96).into(),
        )
    }
}

/// Used for widgets implementing the InputExt, pastes content from the clipboard
pub fn paste<T>(widget: &T)
where
    T: WidgetExt + InputExt,
{
    unsafe {
        Fl_paste(widget.as_widget_ptr() as *mut raw::c_void, 1);
    }
}

/// Sets the callback of a widget
pub fn set_callback<W>(widget: &mut W, cb: Box<dyn FnMut()>)
where
    W: WidgetExt,
{
    debug_assert!(
        widget.top_window().unwrap().takes_events() && widget.takes_events(),
        "Handling events requires that the window and widget be active!"
    );
    unsafe {
        unsafe extern "C" fn shim(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = mem::transmute(data);
            let f: &mut (dyn FnMut()) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_callback_with_captures(widget.as_widget_ptr(), callback, data);
    }
}

/// Initializes loaded fonts of a certain patter ```name```
fn set_fonts(name: &str) -> u8 {
    let name = CString::new(name).unwrap();
    unsafe { Fl_set_fonts(name.into_raw() as *mut raw::c_char) as u8 }
}

/// Returns the number of fonts available to the application
pub fn get_font_count() -> u8 {
    set_fonts("*")
}

/// Gets the name of a font through its index
pub fn get_font_name(idx: u8) -> Option<String> {
    unsafe {
        let font = Fl_get_font(idx as i32);
        if font.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(font as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }
}

/// Returns a list of available fonts to the application
pub fn get_font_names() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let cnt = get_font_count();
    for i in 0..cnt {
        vec.push(get_font_name(i).unwrap());
    }
    vec
}

/// Finds the index of a font through its name
pub fn get_font_index(name: &str) -> Option<u8> {
    let cnt = set_fonts("*");
    let mut ret: Option<u8> = None;
    for i in 0..cnt {
        if name == get_font_name(i).unwrap() {
            ret = Some(i);
            break;
        }
    }
    ret
}

/// Adds a custom handler for unhandled events
pub fn add_handler(cb: fn(Event) -> bool) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(ev: raw::c_int) -> raw::c_int> =
            Some(mem::transmute(move |ev| {
                cb(ev) as i32;
            }));
        Fl_add_handler(callback);
    }
}

fn wait() -> bool {
    unsafe {
        match Fl_wait() {
            0 => false,
            _ => true,
        }
    }
}

/// Sends a custom message
fn awake_msg<T>(msg: T) {
    unsafe {
        Fl_awake_msg(Box::into_raw(Box::from(msg)) as *mut raw::c_void)
    }
}

/// Receives a custom message
fn thread_msg<T>() -> Option<T> {
    unsafe {
        let msg = Fl_thread_msg();
        if msg.is_null() {
            None
        } else {
            let msg: *const T = msg as *const T;
            Some(std::ptr::read(msg))
        }
    }
}

#[repr(C)]
struct Message<T: Copy> {
    id: usize,
    msg: T,
}

/// Creates a sender struct
#[derive(Debug, Clone, Copy)]
pub struct Sender<T: Copy> {
    data: std::marker::PhantomData<T>,
    id: usize,
}

impl<T: Copy> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        let msg = Message { id: self.id, msg: val };
        awake_msg(msg)
    }
}

/// Creates a receiver struct
#[derive(Debug, Clone, Copy)]
pub struct Receiver<T: Copy> {
    data: std::marker::PhantomData<T>,
    id: usize,
}

impl<T: Copy> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        let data: Option<Message<T>> = thread_msg();
        if data.is_some() {
            let data = data.unwrap();
            if data.id == self.id {
                Some(data.msg)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Creates a channel returning a Sender and Receiver structs
pub fn channel<T: Copy>() -> (Sender<T>, Receiver<T>) {
    let mut s = Sender {
        data: std::marker::PhantomData,
        id: 0,
    };
    let mut r = Receiver {
        data: std::marker::PhantomData,
        id: 0,
    };
    let sz = std::mem::size_of::<T>();
    s.id = sz;
    r.id = sz;
    (s, r)
}

fn first_window() -> Option<crate::window::Window> {
    unsafe {
        let x = Fl_first_window();
        if x.is_null() {
            None
        } else {
            let x = crate::window::Window::from_widget_ptr(x as *mut fltk_sys::widget::Fl_Widget);
            Some(x)
        }
    }
}

/// Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
pub fn add_timeout(tm: f64, cb: Box<dyn FnMut()>) {
    let main_win = first_window();
    debug_assert!(main_win.is_some() && main_win.unwrap().takes_events(), "Main Window is unable to take events!");
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = mem::transmute(data);
            let f: &mut (dyn FnMut()) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_add_timeout(tm,  callback, data);
    }
}

/// Repeats a timeout callback from the expiration of the previous timeout
/// You may only call this method inside a timeout callback.
/// The timeout duration `tm` is indicated in seconds
pub fn repeat_timeout(tm: f64, cb: Box<dyn FnMut()>) {
    let main_win = first_window();
    debug_assert!(main_win.is_some() && main_win.unwrap().takes_events(), "Main Window is unable to take events!");
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = mem::transmute(data);
            let f: &mut (dyn FnMut()) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_repeat_timeout(tm,  callback, data);
    }
}

/// Removes a timeout callback
pub fn remove_timeout(cb: Box<dyn FnMut()>) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = mem::transmute(data);
            let f: &mut (dyn FnMut()) = &mut **a;
            f();
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
        let data: *mut raw::c_void = mem::transmute(a);
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_remove_timeout(callback, data);
    }
}