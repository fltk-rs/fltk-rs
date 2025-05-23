use crate::prelude::WidgetExt;
use crate::utils::oncelock::*;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;

#[cfg(feature = "single-threaded")]
use std::cell::RefCell;

static STATE: OnceCell<Mutex<Box<dyn Any + Send + Sync + 'static>>> = OnceCell::new();

/// Represents global state
#[derive(Debug, Copy)]
pub struct GlobalState<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> Clone for GlobalState<T> {
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
        GlobalState {
            marker: std::marker::PhantomData,
        }
    }

    /// Modifies the global state by acquiring a mutable reference
    /// # Panics
    /// Panics on state downcast errors
    pub fn with<V: Clone, F: 'static + FnMut(&mut T) -> V>(&self, mut cb: F) -> V {
        if let Some(state) = STATE.get() {
            if let Ok(mut state) = state.try_lock() {
                if let Some(state) = state.downcast_mut::<T>() {
                    cb(state)
                } else {
                    panic!("failed to downcast state");
                }
            } else {
                panic!("failed to lock state");
            }
        } else {
            panic!("failed to get state");
        }
    }

    /// Gets the already initialized global state
    pub fn get() -> Self {
        GlobalState {
            marker: std::marker::PhantomData,
        }
    }
}

#[cfg(not(feature = "single-threaded"))]
static WIDGET_MAP: Lazy<Mutex<HashMap<String, Box<dyn Any + Send + Sync + 'static>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

#[cfg(feature = "single-threaded")]
thread_local! {
    static WIDGET_MAP: RefCell<HashMap<String, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Allows setting a an id to a widget.
pub trait WidgetId<W>
where
    W: WidgetExt,
{
    /// Set the widget's Id
    fn set_id(&mut self, id: &str);

    #[allow(clippy::return_self_not_must_use)]
    /// Construct a widget with an Id
    fn with_id(self, id: &str) -> Self
    where
        Self: Sized;
}

#[cfg(not(feature = "single-threaded"))]
impl<W> WidgetId<W> for W
where
    W: WidgetExt + Clone + Send + Sync + 'static,
{
    fn set_id(&mut self, id: &str) {
        WIDGET_MAP
            .lock()
            .unwrap()
            .insert(id.to_string(), Box::new(self.clone()));
    }
    fn with_id(mut self, id: &str) -> Self {
        self.set_id(id);
        self
    }
}

#[cfg(feature = "single-threaded")]
impl<W> WidgetId<W> for W
where
    W: WidgetExt + Clone + 'static,
{
    fn set_id(&mut self, id: &str) {
        WIDGET_MAP.with(|w| {
            w.borrow_mut()
                .insert(id.to_string(), Box::new(self.clone()))
        });
    }
    fn with_id(mut self, id: &str) -> Self {
        self.set_id(id);
        self
    }
}

/// Get back the widget thru its id
pub fn widget_from_id<T: 'static + Clone>(id: &str) -> Option<T> {
    #[cfg(not(feature = "single-threaded"))]
    if let Some(w) = WIDGET_MAP.lock().unwrap().get(id) {
        w.downcast_ref::<T>().map(|w| (*w).clone())
    } else {
        None
    }
    #[cfg(feature = "single-threaded")]
    WIDGET_MAP.with(|w| {
        if let Some(w) = w.borrow().get(id) {
            w.downcast_ref::<T>().map(|w| (*w).clone())
        } else {
            None
        }
    })
}
