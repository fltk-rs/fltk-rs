use fltk_sys::fl;
use std::marker;
use std::any::Any;

static mut SENDER: Option<crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>> = None;
static mut RECEIVER: Option<crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>> = None;

#[doc(hidden)]
/// Sends a custom message
/// # Safety
/// The type must be Send and Sync safe
pub unsafe fn awake_msg<T>(msg: T) {
    fl::Fl_awake_msg(Box::into_raw(Box::from(msg)) as *mut std::os::raw::c_void);
}

#[allow(clippy::missing_safety_doc)]
#[doc(hidden)]
/**
    Receives a custom message
    ```rust,no_run
    use fltk::{prelude::*, *};
    if let Some(msg) = unsafe { app::thread_msg::<i32>() } { /* do something */ }
    ```
    # Safety
    The type must correspond to the received message
*/
pub unsafe fn thread_msg<T>() -> Option<T> {
    let msg = fl::Fl_thread_msg();
    if msg.is_null() {
        None
    } else {
        let msg = Box::from_raw(msg as *const _ as *mut T);
        Some(*msg)
    }
}

/// Creates a sender struct
#[derive(Debug, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync + Clone> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        unsafe {
            if let Some(s) = &SENDER {
                s.try_send(Box::new(val))
                    .ok();
            }
        }
    }
}

/// Creates a receiver struct
#[derive(Debug, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync + Clone> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        if let Some(r) = unsafe { &RECEIVER } {
            if let Ok(msg) = r.try_recv() {
                if let Some(message) = msg.downcast_ref::<T>() {
                    Some((*message).clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}


/// Creates a channel returning a Sender and Receiver structs (mpsc: multiple producer single consumer).
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
    unsafe {
        if SENDER.is_none() || RECEIVER.is_none() {
            let (s, r) = crossbeam_channel::unbounded();
            SENDER = Some(s);
            RECEIVER = Some(r);
        }
    }

    let s = Sender {
        data: marker::PhantomData,
    };
    let r = Receiver {
        data: marker::PhantomData,
    };
    (s, r)
}
