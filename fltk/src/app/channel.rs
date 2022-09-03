use fltk_sys::fl;
use std::marker;
use std::any::Any;
use once_cell::sync::Lazy;

type Chan = (crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>, crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>);

static CHANNEL: Lazy<Chan> = Lazy::new(|| crossbeam_channel::unbounded());
static SENDER: Lazy<crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>> = Lazy::new(|| CHANNEL.clone().0);
static RECEIVER: Lazy<crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>> = Lazy::new(|| CHANNEL.clone().1);

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

unsafe impl<T: Send + Sync> Send for Sender<T> {}
unsafe impl<T: Send + Sync> Sync for Sender<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        SENDER.try_send(Box::new(val)).ok();
        crate::app::awake();
    }
    /// Get the global sender
    pub fn get() -> Self {
        Sender {
            data: marker::PhantomData,
        }
    }
}

/// Creates a receiver struct
#[derive(Debug, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Receiver<T> {}
unsafe impl<T: Send + Sync> Sync for Receiver<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        if let Ok(msg) = RECEIVER.try_recv() {
            if let Ok(t) = (msg as Box<dyn Any + 'static>).downcast::<T>() {
                Some(*t)
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Get the global receiver
    pub fn get() -> Self {
        Receiver {
            data: marker::PhantomData,
        }
    }
}

/// Creates a channel returning a Sender and Receiver structs (mpsc: multiple producer single consumer).
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
    let s = Sender {
        data: marker::PhantomData,
    };
    let r = Receiver {
        data: marker::PhantomData,
    };
    (s, r)
}
