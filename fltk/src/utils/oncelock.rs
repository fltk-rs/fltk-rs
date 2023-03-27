use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};

/// A thread-safe write-once cell
pub struct OnceCell<T> {
    value: UnsafeCell<Option<T>>,
}

unsafe impl<T: Sync + Send> Sync for OnceCell<T> {}
unsafe impl<T: Send> Send for OnceCell<T> {}

impl<T> OnceCell<T> {
    /// Create a new OnceCell
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
        }
    }

    /// Get or initialize the OnceCell
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if let Some(v) = self.get() {
            v
        } else {
            let v = f();
            assert!(self.set(v).is_ok());
            self.get().unwrap()
        }
    }

    /// Get the optional value of a OnceCell
    pub fn get(&self) -> Option<&T> {
        unsafe { &*self.value.get() }.as_ref()
    }

    /// Get a mutable optional value of the OnceCell
    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe { &mut *self.value.get() }.as_mut()
    }

    /// Set the value of the OnceCell
    pub fn set(&self, value: T) -> Result<(), T> {
        if self.get().is_some() {
            Err(value)
        } else {
            unsafe {
                *self.value.get() = Some(value);
            }
            Ok(())
        }
    }
}

/// A lazily-initialized (at first access) static
pub struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}

impl<T, F> Lazy<T, F> {
    /// Create a new Lazy static
    pub const fn new(f: F) -> Lazy<T, F> {
        Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(f)),
        }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    fn eval(&self) -> &T {
        self.cell.get_or_init(|| match self.init.take() {
            Some(f) => f(),
            None => panic!("Value already evaluated!"),
        })
    }
}

unsafe impl<T, F: Send> Sync for Lazy<T, F> where OnceCell<T>: Sync {}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        self.eval()
    }
}

impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.eval();
        self.cell.get_mut().unwrap_or_else(|| unreachable!())
    }
}
