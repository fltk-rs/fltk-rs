use std::sync::Mutex;
use std::any::Any;
use once_cell::sync::OnceCell;

static STATE: OnceCell<Mutex<Box<dyn Any + Send + Sync + 'static>>> =  OnceCell::new();

/// Represents global state
#[derive(Debug, Copy)]
pub struct GlobalState<T: Send + Sync> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> Clone for GlobalState<T> {
    fn clone(&self) -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: Sync + Send + 'static> GlobalState<T> {
    /// Creates a new global state
    pub fn new(val: T) -> Self {
        STATE.set(Mutex::new(Box::new(val))).unwrap();
        GlobalState { marker: std::marker::PhantomData }
    }
    
    /// Modifies the global state by acquiring a mutable reference
    pub fn with<V: Clone, F: 'static + Fn(&mut T) -> V>(&self, cb: F) -> V {
        if let Some(val) = STATE.get().unwrap().lock().unwrap().downcast_mut::<T>() {
            cb(val)
        } else {
            panic!("Wrong state type");
        }
    }

    /// Gets the already initialized global state
    pub fn get() -> Self {
        GlobalState { marker: std::marker::PhantomData }
    }
}
