use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};

pub(crate) struct OnceCell<T> {
    value: UnsafeCell<Option<T>>,
}

unsafe impl<T: Send + Sync> Send for OnceCell<T> {}
unsafe impl<T: Send + Sync> Sync for OnceCell<T> {}

impl<T> OnceCell<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
        }
    }

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

    pub fn get(&self) -> Option<&T> {
        unsafe { &*self.value.get() }.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        unsafe { &mut *self.value.get() }.as_mut()
    }

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

pub(crate) struct Lazy<T, F = fn() -> T> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}

impl<T, F> Lazy<T, F> {
    pub const fn new(f: F) -> Lazy<T, F> {
        Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(f)),
        }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn eval(&self) -> &T {
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
