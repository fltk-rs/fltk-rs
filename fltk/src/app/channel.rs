use fltk_sys::fl;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{any, marker, mem, os::raw};

/// Sends a custom message
/// # Safety
/// The type must be Send and Sync safe
pub unsafe fn awake_msg<T>(msg: T) {
    fl::Fl_awake_msg(Box::into_raw(Box::from(msg)) as *mut raw::c_void);
}

#[allow(clippy::missing_safety_doc)]
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
#[derive(Debug, Clone, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

impl<T: Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        let msg = Message {
            hash: self.hash,
            sz: self.sz,
            msg: val,
        };
        unsafe { awake_msg(msg) }
    }
}

/// Creates a receiver struct
#[derive(Debug, Clone, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

impl<T: Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        let data: Option<Message<T>> = unsafe { thread_msg() };
        data.and_then(|data| {
            if data.sz == self.sz && data.hash == self.hash {
                Some(data.msg)
            } else {
                None
            }
        })
    }
}

/// Creates a channel returning a Sender and Receiver structs (mpsc).
/// FLTK doesn't give access to the message queue, only to the last message in the queue, 
/// so a messages sent in WidgetBase::handle() can push other messages down the queue. 
/// If you need access to the queue, you might be better served by another channel implementation with a try_iter() method.
// The implementation could really use generic statics
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
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
