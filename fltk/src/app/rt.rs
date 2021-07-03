use crate::app::init_all;
use crate::prelude::*;
use fltk_sys::fl;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread, time,
};

lazy_static! {
    /// Basically a check for global locking
    pub(crate) static ref IS_INIT: AtomicBool = AtomicBool::new(false);
}

/// Runs the event loop
/// # Errors
/// Returns `FailedToRun`, this is fatal to the app
pub fn run() -> Result<(), FltkError> {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        match fl::Fl_run() {
            0 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Locks the main UI thread
/// # Errors
/// Returns `FailedToLock` if locking is unsopported. This is fatal to the app
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

/// Starts waiting for events.
/// Calls to redraw within wait require an explicit sleep
pub fn wait() -> bool {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
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
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        match fl::Fl_wait_for(dur) as i32 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(FltkError::Unknown(String::from(
                "An unknown error occured!",
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
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        fl::Fl_check() != 0
    }
}

/// This is similar to app::check() except this does not call app::flush() or any callbacks,
/// which is useful if your program is in a state where such callbacks are illegal.
pub fn ready() -> bool {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        fl::Fl_ready() != 0
    }
}
