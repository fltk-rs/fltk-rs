#[doc(hidden)]
#[macro_export]
/// Implements ValuatorExt
macro_rules! impl_valuator_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
                unsafe impl ValuatorExt for $name {
                    fn set_bounds(&mut self, a: f64, b: f64) {
                    unsafe {
        [<$flname _set_bounds>](self.inner.widget() as _, a, b)
                        }
                    }

                    fn minimum(&self) -> f64 {
                    unsafe {
        [<$flname _minimum>](self.inner.widget() as _)
                        }
                    }

                    fn set_minimum(&mut self, a: f64) {
                    unsafe {
        [<$flname _set_minimum>](self.inner.widget() as _, a)
                        }
                    }

                    fn maximum(&self) -> f64 {
                    unsafe {
        [<$flname _maximum>](self.inner.widget() as _)
                        }
                    }

                    fn set_maximum(&mut self, a: f64) {
                    unsafe {
        [<$flname _set_maximum>](self.inner.widget() as _, a)
                        }
                    }

                    fn set_range(&mut self, a: f64, b: f64) {
                    unsafe {
        [<$flname _set_range>](self.inner.widget() as _, a, b)
                        }
                    }

                    fn set_step(&mut self, a: f64, b: i32) {
                    unsafe {
        assert!(b != 0);
                            [<$flname _set_step>](self.inner.widget() as _, a, b)
                        }
                    }

                    fn step(&self) -> f64 {
                    unsafe {
        [<$flname _step>](self.inner.widget() as _)
                        }
                    }

                    fn set_precision(&mut self, digits: i32) {
                    unsafe {
        [<$flname _set_precision>](self.inner.widget() as _, digits)
                        }
                    }

                    fn value(&self) -> f64 {
                    unsafe {
        [<$flname _value>](self.inner.widget() as _)
                        }
                    }

                    fn set_value(&mut self, arg2: f64) {
                    unsafe {
        [<$flname _set_value>](self.inner.widget() as _, arg2);
                        }
                    }

                    fn format(&mut self, arg2: &str) -> Result<(), FltkError> {
                    unsafe {
        let arg2 = CString::safe_new(arg2);
                            let x = [<$flname _format>](
                                self.inner.widget() as _,
                                arg2.as_ptr() as *mut std::os::raw::c_char,
                            );
                            if x < 0 {
                                return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                            }
                            Ok(())
                        }
                    }

                    fn round(&self, arg2: f64) -> f64 {
                    unsafe {
        [<$flname _round>](self.inner.widget() as _, arg2)
                        }
                    }

                    fn clamp(&self, arg2: f64) -> f64 {
                    unsafe {
        [<$flname _clamp>](self.inner.widget() as _, arg2)
                        }
                    }

                    fn increment(&mut self, arg2: f64, arg3: i32) -> f64 {
                    unsafe {
        [<$flname _increment>](self.inner.widget() as _, arg2, arg3)
                        }
                    }
                }
            }
    };
}

pub use impl_valuator_ext;
