use crate::app::{
    font::unload_font, init::init_all, init::is_initialized, init::LOADED_FONT, widget::windows,
};
use crate::prelude::*;
use fltk_sys::fl;
use std::{mem, os::raw, panic, thread, time};

/// Runs the event loop
/// # Errors
/// Returns `FailedToRun`, this is fatal to the app
pub fn run() -> Result<(), FltkError> {
    unsafe {
        if !is_initialized() {
            init_all();
        }
        if !crate::app::is_ui_thread() {
            return Err(FltkError::Internal(FltkErrorKind::FailedToRun));
        }
        match fl::Fl_run() {
            0 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Enable locks. This is called automatically in the beginning of the app initialization
pub fn enable_locks() -> Result<(), FltkError> {
    lock()?;
    Ok(())
}

/// Locks the main UI thread
/// # Errors
/// Returns `FailedToLock` if locking is unsupported. This is fatal to the app
pub fn lock() -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_lock() {
            0 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedToLock)),
        }
    }
}

/// Unlocks the main UI thread
pub fn unlock() {
    unsafe {
        fl::Fl_unlock();
    }
}

/// Trigger event loop handling in the main thread
pub fn awake() {
    unsafe { fl::Fl_awake() }
}

/// Registers a function that will be called by the main thread during the next message handling cycle
pub fn awake_callback<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let mut a: Box<Box<dyn FnMut()>> = Box::from_raw(data as *mut Box<dyn FnMut()>);
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: fl::Fl_Awake_Handler = Some(shim);
        fl::Fl_awake_callback(callback, data);
    }
}

/// Starts waiting for events.
/// Calls to redraw within wait require an explicit sleep
pub fn wait() -> bool {
    unsafe {
        if !is_initialized() {
            init_all();
        }
        assert!(crate::app::is_ui_thread());
        fl::Fl_wait() != 0
    }
}

/// Put the thread to sleep for `dur` seconds
pub fn sleep(dur: f64) {
    let dur = dur * 1000.;
    thread::sleep(time::Duration::from_millis(dur as u64));
}

/// Waits a maximum of `dur` seconds or until "something happens".
/// Returns true if an event happened (always true on windows).
/// Returns false if nothing happened.
/// # Errors
/// Can error out on X11 system if interrupted by a signal
pub fn wait_for(dur: f64) -> Result<bool, FltkError> {
    unsafe {
        if !is_initialized() {
            init_all();
        }
        if !crate::app::is_ui_thread() {
            return Err(FltkError::Internal(FltkErrorKind::FailedToRun));
        }
        match fl::Fl_wait_for(dur) as i32 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(FltkError::Unknown(String::from(
                "The event loop was probably interrupted by an OS signal!",
            ))),
        }
    }
}

/// Returns whether a quit signal was sent
pub fn should_program_quit() -> bool {
    unsafe { fl::Fl_should_program_quit() != 0 }
}

/// Determines whether a program should quit
pub fn program_should_quit(flag: bool) {
    unsafe { fl::Fl_program_should_quit(flag as i32) }
}

/// Calling this during a big calculation will keep the screen up to date and the interface responsive.
pub fn check() -> bool {
    unsafe {
        if !is_initialized() {
            init_all();
        }
        assert!(crate::app::is_ui_thread());
        fl::Fl_check() != 0
    }
}

/// This is similar to app::check() except this does not call app::flush() or any callbacks,
/// which is useful if your program is in a state where such callbacks are illegal.
pub fn ready() -> bool {
    unsafe {
        if !is_initialized() {
            init_all();
        }
        assert!(crate::app::is_ui_thread());
        fl::Fl_ready() != 0
    }
}

/// Quit the app
pub fn quit() {
    if let Some(loaded_font) = LOADED_FONT {
        // Shouldn't fail
        unload_font(loaded_font).unwrap_or(());
    }
    if let Some(wins) = windows() {
        for mut i in wins {
            if i.shown() {
                i.hide();
            }
        }
    }
}

#[deprecated(since = "1.2.26", note = "please use `add_idle3` instead")]
/// Add an idle callback to run within the event loop.
/// Calls to [`WidgetExt::redraw`](`crate::prelude::WidgetExt::redraw`) within the callback require an explicit sleep
pub fn add_idle<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_add_idle(callback, data);
    }
}

#[deprecated(since = "1.2.26", note = "please use `remove_idle3` instead")]
/// Remove an idle function
pub fn remove_idle<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_remove_idle(callback, data);
    }
}

#[deprecated(since = "1.2.26", note = "please use `has_idle3` instead")]
/// Checks whether an idle function is installed
pub fn has_idle<F: FnMut() + 'static>(cb: F) -> bool {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_has_idle(callback, data) != 0
    }
}

/// Add an idle callback to run within the event loop.
/// Calls to [`WidgetExt::redraw`](`crate::prelude::WidgetExt::redraw`) within the callback require an explicit sleep
pub fn add_idle2(cb: fn()) {
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_add_idle(callback, data);
    }
}

/// Remove an idle function
pub fn remove_idle2(cb: fn()) {
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_remove_idle(callback, data);
    }
}

/// Checks whether an idle function is installed
pub fn has_idle2(cb: fn()) -> bool {
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_has_idle(callback, data) != 0
    }
}

/// Handle object for interacting with idle callbacks
pub type IdleHandle = *mut ();

unsafe extern "C" fn idle_shim(data: *mut raw::c_void) {
    let a: *mut Box<dyn FnMut(IdleHandle)> = data as *mut Box<dyn FnMut(IdleHandle)>;
    let f: &mut (dyn FnMut(IdleHandle)) = &mut **a;
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| (*f)(data as _)));
}

/// Add an idle callback to run within the event loop.
/// This function returns a handle that can be used for future interaction with the callback.
/// Calls to [`WidgetExt::redraw`](`crate::prelude::WidgetExt::redraw`) within the callback require an explicit sleep
pub fn add_idle3<F: FnMut(IdleHandle) + 'static>(cb: F) -> IdleHandle {
    unsafe {
        let a: *mut Box<dyn FnMut(IdleHandle)> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(idle_shim);
        fl::Fl_add_idle(callback, data);

        data as _
    }
}

/// Remove the idle function associated with the handle
pub fn remove_idle3(handle: IdleHandle) {
    unsafe {
        let data: *mut raw::c_void = handle as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(idle_shim);
        fl::Fl_remove_idle(callback, data);
    }
}

/// Checks whether the idle function, associated with the handle, is installed
pub fn has_idle3(handle: IdleHandle) -> bool {
    unsafe {
        let data: *mut raw::c_void = handle as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(idle_shim);
        fl::Fl_has_idle(callback, data) != 0
    }
}

/// Register a callback whenever there is a change to the selection buffer or the clipboard.
/// The clipboard is source 1 and the selection buffer is source 0
pub fn add_clipboard_notify(cb: fn(source: i32)) {
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(source: i32, arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_add_clipboard_notify(callback, data);
    }
}

/// Stop calling the specified callback when there are changes to the selection
/// buffer or the clipboard.
/// The clipboard is source 1 and the selection buffer is source 0
pub fn remove_clipboard_notify(cb: fn(source: i32)) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(source: i32, arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_remove_clipboard_notify(callback);
    }
}

#[deprecated(since = "1.2.26", note = "please use `add_clipboard_notify3` instead")]
/// Register a callback whenever there is a change to the selection buffer or the clipboard.
/// The clipboard is source 1 and the selection buffer is source 0.
/// A callback via closure cannot be removed!
pub fn add_clipboard_notify2<F: FnMut(i32) + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(source: i32, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut(i32)> = data as *mut Box<dyn FnMut(i32)>;
            let f: &mut (dyn FnMut(i32)) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f(source)));
        }
        let a: *mut Box<dyn FnMut(i32)> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(source: i32, arg1: *mut raw::c_void)> =
            Some(shim);
        fl::Fl_add_clipboard_notify(callback, data);
    }
}

unsafe extern "C" fn clipboard_notify_shim(source: i32, data: *mut raw::c_void) {
    let a: *mut Box<dyn FnMut(i32)> = data as *mut Box<dyn FnMut(i32)>;
    let f: &mut (dyn FnMut(i32)) = &mut **a;
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| (*f)(source)));
}

/// Register a callback whenever there is a change to the selection buffer or the clipboard.
/// The clipboard is source 1 and the selection buffer is source 0.
/// A callback via closure cannot be removed!
pub fn add_clipboard_notify3<F: FnMut(i32) + 'static>(cb: F) {
    unsafe {
        let a: *mut Box<dyn FnMut(i32)> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(source: i32, arg1: *mut raw::c_void)> =
            Some(clipboard_notify_shim);
        fl::Fl_add_clipboard_notify(callback, data);
    }
}

/// Stop calling the specified callback when there are changes to the selection
/// buffer or the clipboard.
/// The clipboard is source 1 and the selection buffer is source 0
pub fn remove_clipboard_notify3() {
    unsafe {
        let callback: Option<unsafe extern "C" fn(source: i32, arg1: *mut raw::c_void)> =
            Some(clipboard_notify_shim);
        fl::Fl_remove_clipboard_notify(callback);
    }
}

#[deprecated(since = "1.2.26", note = "please use `add_timeout3` instead")]
/**
    Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn add_timeout<F: FnMut() + 'static>(tm: f64, cb: F) {
    unsafe {
        assert!(crate::app::is_ui_thread());
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let mut a: Box<Box<dyn FnMut()>> = Box::from_raw(data as *mut Box<dyn FnMut()>);
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_add_timeout(tm, callback, data);
    }
}

#[deprecated(since = "1.2.26", note = "please use `repeat_timeout3` instead")]
/**
    Repeats a timeout callback from the expiration of the previous timeout.
    You may only call this method inside a timeout callback.
    The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn repeat_timeout<F: FnMut() + 'static>(tm: f64, cb: F) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_repeat_timeout(tm, callback, data);
    }
}

#[deprecated(since = "1.2.26", note = "please use `remove_timeout3` instead")]
/// Removes a timeout callback
pub fn remove_timeout<F: FnMut() + 'static>(cb: F) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_remove_timeout(callback, data);
    }
}

#[deprecated(since = "1.2.26", note = "please use `has_timeout3` instead")]
/// Check whether a timeout is installed
pub fn has_timeout<F: FnMut() + 'static>(cb: F) -> bool {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_has_timeout(callback, data) != 0
    }
}

/**
    Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout2(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout2(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn add_timeout2(tm: f64, cb: fn()) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_add_timeout(tm, callback, data);
    }
}

/**
    Repeats a timeout callback from the expiration of the previous timeout.
    You may only call this method inside a timeout callback.
    The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout2(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout2(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn repeat_timeout2(tm: f64, cb: fn()) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_repeat_timeout(tm, callback, data);
    }
}

/// Removes a timeout callback
pub fn remove_timeout2(cb: fn()) {
    assert!(crate::app::is_ui_thread());
    if has_timeout2(cb) {
        unsafe {
            let data: *mut raw::c_void = std::ptr::null_mut();
            let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
                Some(mem::transmute(cb));
            fl::Fl_remove_timeout(callback, data);
        }
    }
}

/// Check whether a timeout is installed
pub fn has_timeout2(cb: fn()) -> bool {
    unsafe {
        let data: *mut raw::c_void = std::ptr::null_mut();
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        fl::Fl_has_timeout(callback, data) != 0
    }
}

/// Handle object for interacting with timeouts
pub type TimeoutHandle = *mut ();

unsafe extern "C" fn timeout_shim(data: *mut raw::c_void) {
    let a: *mut Box<dyn FnMut(TimeoutHandle)> = data as *mut Box<dyn FnMut(TimeoutHandle)>;
    let f: &mut (dyn FnMut(TimeoutHandle)) = &mut **a;
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| (*f)(data as _)));
}

/**
    Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
    This function returns a handle that can be use for future interaction with the timeout
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let callback = |_handle| {
            println!("FIRED");
        };

        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        let _handle = app::add_timeout3(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn add_timeout3<F: FnMut(TimeoutHandle) + 'static>(tm: f64, cb: F) -> TimeoutHandle {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let a: *mut Box<dyn FnMut(TimeoutHandle)> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(timeout_shim);
        fl::Fl_add_timeout(tm, callback, data);

        data as _
    }
}

/**
    Repeats the timeout callback, associated with the hadle, from the expiration of the previous timeout.
    You may only call this method inside a timeout callback.
    The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let callback = |handle| {
            println!("TICK");
            app::repeat_timeout3(1.0, handle);
        };

        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout3(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn repeat_timeout3(tm: f64, handle: TimeoutHandle) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let data: *mut raw::c_void = handle as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(timeout_shim);
        fl::Fl_repeat_timeout(tm, callback, data);
    }
}

/**
    Removes the timeout callback associated with the handle
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let callback = |handle| {
            println!("FIRED");
        };

        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        let handle = app::add_timeout3(1.0, callback);
        app::remove_timeout3(handle);
        app.run().unwrap();
    }
    ```
*/
pub fn remove_timeout3(handle: TimeoutHandle) {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let data: *mut raw::c_void = handle as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(timeout_shim);
        fl::Fl_remove_timeout(callback, data);
    }
}

/// Check whether the timeout, associated with the handle, is installed
pub fn has_timeout3(handle: TimeoutHandle) -> bool {
    assert!(crate::app::is_ui_thread());
    unsafe {
        let data: *mut raw::c_void = handle as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(timeout_shim);
        fl::Fl_has_timeout(callback, data) != 0
    }
}

#[doc(hidden)]
pub fn add_raw_timeout<T>(tm: f64, cb: fn(*mut T), data: *mut T) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        let data: *mut raw::c_void = data as *mut raw::c_void;
        fl::Fl_add_timeout(tm, callback, data);
    }
}

#[doc(hidden)]
pub fn repeat_raw_timeout<T>(tm: f64, cb: fn(*mut T), data: *mut T) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        let data: *mut raw::c_void = data as *mut raw::c_void;
        fl::Fl_repeat_timeout(tm, callback, data);
    }
}

#[doc(hidden)]
pub fn remove_raw_timeout<T>(cb: fn(*mut T), data: *mut T) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        let data: *mut raw::c_void = data as *mut raw::c_void;
        fl::Fl_remove_timeout(callback, data);
    }
}

#[doc(hidden)]
pub fn has_raw_timeout<T>(cb: fn(*mut T), data: *mut T) -> bool {
    unsafe {
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> =
            Some(mem::transmute(cb));
        let data: *mut raw::c_void = data as *mut raw::c_void;
        fl::Fl_has_timeout(callback, data) != 0
    }
}

/// Add a system handler
/// # Safety
/// FLTK makes no assurances regarding handling by the system handler
pub unsafe fn add_system_handler(
    cb: Option<unsafe extern "C" fn(*mut raw::c_void, *mut raw::c_void) -> i32>,
    data: *mut raw::c_void,
) {
    assert!(crate::app::is_ui_thread());
    fl::Fl_add_system_handler(cb, data);
}

/// Add a system handler
/// # Safety
/// FLTK makes no assurances regarding handling by the system handler
pub unsafe fn remove_system_handler(
    cb: Option<unsafe extern "C" fn(*mut raw::c_void, *mut raw::c_void) -> i32>,
) {
    assert!(crate::app::is_ui_thread());
    fl::Fl_remove_system_handler(cb);
}
