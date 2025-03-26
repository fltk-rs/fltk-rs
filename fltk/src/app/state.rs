use crate::prelude::WidgetExt;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

static STATE: OnceLock<Mutex<Box<dyn Any + Send + Sync + 'static>>> = OnceLock::new();

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

thread_local! {
    static WIDGET_MAP: RefCell<HashMap<String, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Allows setting a an id to a widget.
/// Will not work with the single-threaded feature.
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
    WIDGET_MAP.with(|w| {
        if let Some(w) = w.borrow().get(id) {
            w.downcast_ref::<T>().map(|w| (*w).clone())
        } else {
            None
        }
    })
}
