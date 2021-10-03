#[macro_export]
macro_rules! impl_valuator_ext {
    ($name: ident, $flname: ident) => {
        paste! {
            unsafe impl ValuatorExt for $name {
                fn set_bounds(&mut self, a: f64, b: f64) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_bounds>](self.inner, a, b)
                    }
                }
    
                fn minimum(&self) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _minimum>](self.inner)
                    }
                }
    
                fn set_minimum(&mut self, a: f64) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_minimum>](self.inner, a)
                    }
                }
    
                fn maximum(&self) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _maximum>](self.inner)
                    }
                }
    
                fn set_maximum(&mut self, a: f64) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_maximum>](self.inner, a)
                    }
                }
    
                fn set_range(&mut self, a: f64, b: f64) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_range>](self.inner, a, b)
                    }
                }
    
                fn set_step(&mut self, a: f64, b: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(b != 0);
                        [<$flname _set_step>](self.inner, a, b)
                    }
                }
    
                fn step(&self) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _step>](self.inner)
                    }
                }
    
                fn set_precision(&mut self, digits: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_precision>](self.inner, digits)
                    }
                }
    
                fn value(&self) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _value>](self.inner)
                    }
                }
    
                fn set_value(&mut self, arg2: f64) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_value>](self.inner, arg2);
                    }
                }
    
                fn format(&mut self, arg2: &str) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let arg2 = CString::safe_new(arg2);
                        let x = [<$flname _format>](
                            self.inner,
                            arg2.as_ptr() as *mut raw::c_char,
                        );
                        if x < 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }
    
                fn round(&self, arg2: f64) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _round>](self.inner, arg2)
                    }
                }
    
                fn clamp(&self, arg2: f64) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _clamp>](self.inner, arg2)
                    }
                }
    
                fn increment(&mut self, arg2: f64, arg3: i32) -> f64 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _increment>](self.inner, arg2, arg3)
                    }
                }
            }
        }
    };
}
