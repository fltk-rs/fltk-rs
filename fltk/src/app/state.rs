use std::sync::Mutex;
use std::any::Any;

lazy_static::lazy_static! {
    static ref STATE: Mutex<Box<dyn Any + Send + Sync + 'static>> =  Mutex::new(Box::new(0));
}

/// Represents global state
#[derive(Debug)]
pub struct AppState<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Sync + Send + 'static> AppState<T> {
    /// Creates a new global state
    pub fn new(val: T) -> Self {
        *STATE.lock().unwrap() = Box::new(val);
        AppState { marker: std::marker::PhantomData }
    }
    
    /// Modifies the global state
    pub fn modify<V: Clone, F: 'static + Fn(&mut T) -> V>(&self, cb: F) -> V {
        if let Some(val) = STATE.lock().unwrap().downcast_mut::<T>() {
            cb(val)
        } else {
            panic!("Wrong state type");
        }
    }

    /// Gets the already initialized global state
    pub fn get() -> Self {
        AppState { marker: std::marker::PhantomData }
    }
}
