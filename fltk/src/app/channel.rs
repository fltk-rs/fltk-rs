use fltk_sys::fl;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{any, marker, mem, os::raw};

static mut SENDER: Option<crossbeam_channel::Sender<*mut raw::c_void>> = None;
static mut RECEIVER: Option<crossbeam_channel::Receiver<*mut raw::c_void>> = None;

#[doc(hidden)]
/// Sends a custom message
/// # Safety
/// The type must be Send and Sync safe
pub unsafe fn awake_msg<T>(msg: T) {
    fl::Fl_awake_msg(Box::into_raw(Box::from(msg)) as *mut raw::c_void);
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

#[repr(C)]
struct Message<T: Send + Sync> {
    hash: u64,
    sz: usize,
    msg: T,
}

/// Creates a sender struct
#[derive(Debug, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Sender {
      data: marker::PhantomData,
      hash: self.hash,
      sz: self.sz,
    }
  }
}

impl<T: Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        let msg = Message {
            hash: self.hash,
            sz: self.sz,
            msg: val,
        };
        unsafe {
            if let Some(s) = &SENDER {
                s.try_send(Box::into_raw(Box::from(msg)) as *mut raw::c_void)
                    .ok();
                crate::app::awake();
            }
        }
    }
}

/// Creates a receiver struct
#[derive(Debug, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}
//
// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
  fn clone(&self) -> Self {
    Receiver {
      data: marker::PhantomData,
      hash: self.hash,
      sz: self.sz,
    }
  }
}

impl<T: Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        if let Some(r) = unsafe { &RECEIVER } {
            if let Ok(msg) = r.try_recv() {
                if !msg.is_null() {
                    let data = unsafe { Box::from_raw(msg as *const _ as *mut Message<T>) };
                    if data.sz == self.sz && data.hash == self.hash {
                        Some(data.msg)
                    } else {
                        None
                    }
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
/// FLTK doesn't give access to the message queue, only to the last message in the queue,
/// so a messages sent in WidgetBase::handle() can push other messages down the queue.
/// If you need access to the queue, you might be better served by another channel implementation with a try_iter() method.
// The implementation could really use generic statics
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
    unsafe {
        if SENDER.is_none() || RECEIVER.is_none() {
            let (s, r) = crossbeam_channel::unbounded();
            SENDER = Some(s);
            RECEIVER = Some(r);
        }
    }
    let msg_sz = mem::size_of::<T>();
    let type_name = any::type_name::<T>();
    let mut hasher = DefaultHasher::new();
    type_name.hash(&mut hasher);
    let type_hash = hasher.finish();

    let s = Sender {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    let r = Receiver {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    (s, r)
}
