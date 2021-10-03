#[macro_export]
macro_rules! impl_button_ext {
    ($name: tt, $flname: tt) => {
        unsafe impl ButtonExt for $name {
            fn shortcut(&self) -> Shortcut {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _shortcut)(self.inner))
                }
            }

            fn set_shortcut(&mut self, shortcut: Shortcut) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_shortcut)(self.inner, shortcut.bits() as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _clear)(self.inner);
                }
            }

            fn is_set(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _value)(self.inner) != 0
                }
            }

            fn set(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_value)(self.inner, flag as i32)
                }
            }

            fn value(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _value)(self.inner) != 0
                }
            }

            fn set_value(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_value)(self.inner, flag as i32)
                }
            }

            fn set_down_frame(&mut self, f: FrameType) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _set_down_box)(self.inner, f as i32) }
            }

            fn down_frame(&self) -> FrameType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(concat_idents!($flname, _down_box)(self.inner)) }
            }
        }
    };
}
