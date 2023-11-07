#[doc(hidden)]
#[macro_export]
/// Implements InputExt
macro_rules! impl_input_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl InputExt for $name {
                fn value(&self) -> String {
                    unsafe {
                        let value_ptr = [<$flname _value>](self.inner.widget() as _);
                        assert!(!value_ptr.is_null());
                        CStr::from_ptr(value_ptr as *mut std::os::raw::c_char)
                            .to_string_lossy()
                            .to_string()
                    }
                }

                fn set_value(&mut self, val: &str) {

                    let temp = CString::safe_new(val);
                    unsafe {
                        [<$flname _set_value>](self.inner.widget() as _, temp.as_ptr());
                    }
                }

                fn maximum_size(&self) -> i32 {
                    unsafe {
                        [<$flname _maximum_size>](self.inner.widget() as _) as i32
                    }
                }

                fn set_maximum_size(&mut self, val: i32) {
                    unsafe {
                        [<$flname _set_maximum_size>](self.inner.widget() as _, val as i32)
                    }
                }

                fn position(&self) -> i32 {
                    unsafe {
                        [<$flname _position>](self.inner.widget() as _) as i32
                    }
                }

                fn set_position(&mut self, val: i32) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _set_position>](self.inner.widget() as _, val as i32);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn mark(&self) -> i32 {
                    unsafe {
                        [<$flname _mark>](self.inner.widget() as _) as i32
                    }
                }

                fn set_mark(&mut self, val: i32) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _set_mark>](self.inner.widget() as _, val as i32);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn replace(&mut self, beg: i32, end: i32, val: &str) -> Result<(), FltkError> {

                    let val = CString::safe_new(val);
                    unsafe {
                        let x = [<$flname _replace>](
                            self.inner.widget() as _,
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

                fn insert(&mut self, txt: &str) -> Result<(), FltkError> {

                    let txt = CString::safe_new(txt);
                    unsafe {
                        let x = [<$flname _insert>](self.inner.widget() as _, txt.as_ptr(), 0);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn append(&mut self, txt: &str) -> Result<(), FltkError> {

                    let txt = CString::safe_new(txt);
                    unsafe {
                        let x = [<$flname _append>](self.inner.widget() as _, txt.as_ptr(), 0, 0);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn copy(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _copy>](self.inner.widget() as _, 1);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn undo(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _undo>](self.inner.widget() as _);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn cut(&mut self) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _copy_cuts>](self.inner.widget() as _);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn text_font(&self) -> $crate::enums::Font {
                    unsafe {
                        std::mem::transmute([<$flname _text_font>](self.inner.widget() as _))
                    }
                }

                fn set_text_font(&mut self, font: $crate::enums::Font) {
                    unsafe {
                        [<$flname _set_text_font>](self.inner.widget() as _, font.bits() as i32)
                    }
                }

                fn cursor_color(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _cursor_color>](self.inner.widget() as _))
                    }
                }

                fn set_cursor_color(&mut self, color: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_cursor_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn text_color(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _text_color>](self.inner.widget() as _))
                    }
                }

                fn set_text_color(&mut self, color: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_text_color>](self.inner.widget() as _, color.bits() as u32)
                    }
                }

                fn text_size(&self) -> i32 {
                    unsafe {
                        [<$flname _text_size>](self.inner.widget() as _) as i32
                    }
                }

                fn set_text_size(&mut self, sz: i32) {
                    unsafe {
                        [<$flname _set_text_size>](self.inner.widget() as _, sz as i32)
                    }
                }

                fn readonly(&self) -> bool {
                    unsafe {
                        [<$flname _readonly>](self.inner.widget() as _) != 0
                    }
                }

                fn set_readonly(&mut self, val: bool) {
                    unsafe {
                        [<$flname _set_readonly>](self.inner.widget() as _, val as i32)
                    }
                }

                fn wrap(&self) -> bool {
                    unsafe {
                        [<$flname _wrap>](self.inner.widget() as _) != 0
                    }
                }

                fn set_wrap(&mut self, val: bool) {
                    unsafe {
                        [<$flname _set_wrap>](self.inner.widget() as _, val as i32)
                    }
                }

                fn set_tab_nav(&mut self, val: bool) {
                    unsafe {
                        [<$flname _set_tab_nav>](self.inner.widget() as _, val as i32)
                    }
                }

                fn tab_nav(&self) -> bool {
                    unsafe {
                        [<$flname _tab_nav>](self.inner.widget() as _) != 0
                    }
                }
            }
        }
    };
}

pub use impl_input_ext;
