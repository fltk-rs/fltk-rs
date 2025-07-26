use std::any::Any;
use std::marker;
use std::sync::LazyLock;

type Chan = (
    crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>,
    crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>,
);

static CHANNEL: LazyLock<Chan> = LazyLock::new(crossbeam_channel::unbounded);
static SENDER: LazyLock<crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>> =
    LazyLock::new(|| CHANNEL.clone().0);
static RECEIVER: LazyLock<crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>> =
    LazyLock::new(|| CHANNEL.clone().1);

/// Creates a sender struct
#[derive(Debug)]
pub struct Sender<T> {
    data: marker::PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Sender<T> {}
unsafe impl<T: Send + Sync> Sync for Sender<T> {}
impl<T: Send + Sync> Copy for Sender<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
    fn clone(&self) -> Self {
        *self
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
#[derive(Debug)]
pub struct Receiver<T> {
    data: marker::PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Receiver<T> {}
unsafe impl<T: Send + Sync> Sync for Receiver<T> {}
impl<T: Send + Sync> Copy for Receiver<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        *self
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
