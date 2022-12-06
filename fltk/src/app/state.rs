use crate::prelude::WidgetExt;
use crate::utils::oncelock::{Lazy, OnceCell};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;

static STATE: OnceCell<Mutex<Box<dyn Any + Send + Sync + 'static>>> = OnceCell::new();

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
        GlobalState {
            marker: std::marker::PhantomData,
        }
    }

    /// Modifies the global state by acquiring a mutable reference
    /// # Panics
    /// Panics on state downcast errors
    pub fn with<V: Clone, F: 'static + FnMut(&mut T) -> V>(&self, mut cb: F) -> V {
        if let Some(val) = STATE.get().unwrap().lock().unwrap().downcast_mut::<T>() {
            cb(val)
        } else {
            panic!("Wrong state type");
        }
    }

    /// Gets the already initialized global state
    pub fn get() -> Self {
        GlobalState {
            marker: std::marker::PhantomData,
        }
    }
}

static WIDGET_MAP: Lazy<Mutex<HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

/// Allows setting a an id to a widget
pub trait WidgetId<W>
where
    W: WidgetExt,
{
    /// Set the widget's Id
    fn set_id(&mut self, id: &'static str);
    /// Construct a widget with an Id
    fn with_id(self, id: &'static str) -> Self
    where
        Self: Sized;
}

impl<W> WidgetId<W> for W
where
    W: WidgetExt + Send + Sync + Clone + 'static,
{
    fn set_id(&mut self, id: &'static str) {
        WIDGET_MAP
            .lock()
            .unwrap()
            .insert(id, Box::new(self.clone()));
    }
    fn with_id(mut self, id: &'static str) -> Self {
        self.set_id(id);
        self
    }
}

/// Get back the widget thru its id
pub fn widget_from_id<T: 'static + Clone>(id: &'static str) -> Option<T> {
    if let Some(w) = WIDGET_MAP.lock().unwrap().get(&id) {
        w.downcast_ref::<T>().map(|w| (*w).clone())
    } else {
        None
    }
}
