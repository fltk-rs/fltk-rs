#[doc(hidden)]
#[macro_export]
/// Implements InputExt
macro_rules! impl_input_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl InputExt for $name {
                fn value(&self) -> String {
                    unsafe {
                        assert!(!self.was_deleted());
                        let value_ptr = [<$flname _value>](self.inner);
                        assert!(!value_ptr.is_null());
                        CStr::from_ptr(value_ptr as *mut std::os::raw::c_char)
                            .to_string_lossy()
                            .to_string()
                    }
                }

                fn set_value(&self, val: &str) {
                    assert!(!self.was_deleted());
                    let temp = CString::safe_new(val);
                    unsafe {
                        [<$flname _set_value>](self.inner, temp.as_ptr());
                    }
                }

                fn maximum_size(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _maximum_size>](self.inner) as i32
                    }
                }

                fn set_maximum_size(&self, val: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_maximum_size>](self.inner, val as i32)
                    }
                }

                fn position(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _position>](self.inner) as i32
                    }
                }

                fn set_position(&self, val: i32) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let x = [<$flname _set_position>](self.inner, val as i32);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn mark(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _mark>](self.inner) as i32
                    }
                }

                fn set_mark(&self, val: i32) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let x = [<$flname _set_mark>](self.inner, val as i32);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn replace(&self, beg: i32, end: i32, val: &str) -> Result<(), FltkError> {
                    assert!(!self.was_deleted());
                    let val = CString::safe_new(val);
                    unsafe {
                        let x = [<$flname _replace>](
                            self.inner,
                            beg as i32,
                            end as i32,
                            val.as_ptr(),
                            0,
                        );
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn insert(&self, txt: &str) -> Result<(), FltkError> {
                    assert!(!self.was_deleted());
                    let txt = CString::safe_new(txt);
                    unsafe {
                        let x = [<$flname _insert>](self.inner, txt.as_ptr(), 0);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn append(&self, txt: &str) -> Result<(), FltkError> {
                    assert!(!self.was_deleted());
                    let txt = CString::safe_new(txt);
                    unsafe {
                        let x = [<$flname _append>](self.inner, txt.as_ptr(), 0, 0);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn copy(&self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let x = [<$flname _copy>](self.inner, 1);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn undo(&self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let x = [<$flname _undo>](self.inner);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn cut(&self) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let x = [<$flname _copy_cuts>](self.inner);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn text_font(&self) -> $crate::enums::Font {
                    unsafe {
                        assert!(!self.was_deleted());
                        std::mem::transmute([<$flname _text_font>](self.inner))
                    }
                }

                fn set_text_font(&self, font: $crate::enums::Font) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_font>](self.inner, font.bits() as i32)
                    }
                }

                fn text_color(&self) -> $crate::enums::Color {
                    unsafe {
                        assert!(!self.was_deleted());
                        std::mem::transmute([<$flname _text_color>](self.inner))
                    }
                }

                fn set_text_color(&self, color: $crate::enums::Color) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_color>](self.inner, color.bits() as u32)
                    }
                }

                fn text_size(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _text_size>](self.inner) as i32
                    }
                }

                fn set_text_size(&self, sz: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_size>](self.inner, sz as i32)
                    }
                }

                fn readonly(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _readonly>](self.inner) != 0
                    }
                }

                fn set_readonly(&self, val: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_readonly>](self.inner, val as i32)
                    }
                }

                fn wrap(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _wrap>](self.inner) != 0
                    }
                }

                fn set_wrap(&self, val: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_wrap>](self.inner, val as i32)
                    }
                }
            }
        }
    };
}

pub use impl_input_ext;
