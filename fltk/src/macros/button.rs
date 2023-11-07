#[doc(hidden)]
#[macro_export]
/// Implements ButtonExt
macro_rules! impl_button_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl ButtonExt for $name {
                fn shortcut(&self) -> $crate::enums::Shortcut {
                    unsafe {
                        std::mem::transmute([<$flname _shortcut>](self.inner.widget() as _))
                    }
                }

                fn set_shortcut(&mut self, shortcut: $crate::enums::Shortcut) {
                    unsafe {
                        [<$flname _set_shortcut>](self.inner.widget() as _, shortcut.bits() as i32)
                    }
                }

                fn clear(&mut self) {
                    unsafe {
                        [<$flname _clear>](self.inner.widget() as _);
                    }
                }

                fn is_set(&self) -> bool {
                    unsafe {
                        [<$flname _value>](self.inner.widget() as _) != 0
                    }
                }

                fn set(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_value>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn value(&self) -> bool {
                    unsafe {
                        [<$flname _value>](self.inner.widget() as _) != 0
                    }
                }

                fn set_value(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_value>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn set_down_frame(&mut self, f: $crate::enums::FrameType) {

                    unsafe { [<$flname _set_down_box>](self.inner.widget() as _, f.as_i32()) }
                }

                fn down_frame(&self) -> $crate::enums::FrameType {

                    unsafe { $crate::enums::FrameType::from_i32([<$flname _down_box>](self.inner.widget() as _)) }
                }
            }
        }
    };
}

pub use impl_button_ext;
