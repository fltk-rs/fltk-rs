#[doc(hidden)]
#[macro_export]
/// Implements MenuExt
macro_rules! impl_menu_ext {
    ($name: ident, $flname: ident) => {
        impl IntoIterator for $name {
            type Item = MenuItem;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                let mut v: Vec<MenuItem> = vec![];
                for i in 0..self.size() {
                    v.push(self.at(i).unwrap());
                }
                v.into_iter()
            }
        }

        paste::paste! {
            unsafe impl MenuExt for $name {
                fn add<F: FnMut(&mut Self) + 'static>(
                    &mut self,
                    name: &str,
                    shortcut: $crate::enums::Shortcut,
                    flag: MenuFlag,
                    cb: F,
                ) -> i32 {
                    assert!(!self.was_deleted());
                    let temp = CString::safe_new(name);
                    unsafe {
                        unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut std::os::raw::c_void) {
                            let mut wid = $crate::widget::Widget::from_widget_ptr(wid as *mut _);
                            let a: *mut Box<dyn FnMut(&mut $crate::widget::Widget)> =
                                data as *mut Box<dyn FnMut(&mut $crate::widget::Widget)>;
                            let f: &mut (dyn FnMut(&mut $crate::widget::Widget)) = &mut **a;
                            let _ =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                        }
                        let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
                        let callback: Fl_Callback = Some(shim);
                        [<$flname _add>](
                            self.inner,
                            temp.as_ptr(),
                            shortcut.bits() as i32,
                            callback,
                            data,
                            flag.bits(),
                        )
                    }
                }

                fn insert<F: FnMut(&mut Self) + 'static>(
                    &mut self,
                    idx: i32,
                    name: &str,
                    shortcut: $crate::enums::Shortcut,
                    flag: MenuFlag,
                    cb: F,
                ) -> i32 {
                    assert!(!self.was_deleted());
                    let idx = if idx < self.size() && idx >= 0 {
                        idx
                    } else {
                        if self.size() == 0 {
                            0
                        } else {
                            self.size() - 1
                        }
                    };
                    let temp = CString::safe_new(name);
                    unsafe {
                        unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut std::os::raw::c_void) {
                            let mut wid = $crate::widget::Widget::from_widget_ptr(wid as *mut _);
                            let a: *mut Box<dyn FnMut(&mut $crate::widget::Widget)> =
                                data as *mut Box<dyn FnMut(&mut $crate::widget::Widget)>;
                            let f: &mut (dyn FnMut(&mut $crate::widget::Widget)) = &mut **a;
                            let _ =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                        }
                        let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
                        let callback: Fl_Callback = Some(shim);
                        [<$flname _insert>](
                            self.inner,
                            idx as i32,
                            temp.as_ptr(),
                            shortcut.bits() as i32,
                            callback,
                            data,
                            flag.bits(),
                        )
                    }
                }

                fn add_emit<T: 'static + Clone + Send + Sync>(
                    &mut self,
                    label: &str,
                    shortcut: $crate::enums::Shortcut,
                    flag: $crate::menu::MenuFlag,
                    sender: $crate::app::Sender<T>,
                    msg: T,
                ) -> i32 {
                    self.add(label, shortcut, flag, move |_| sender.send(msg.clone()))
                }

                fn insert_emit<T: 'static + Clone + Send + Sync>(
                    &mut self,
                    idx: i32,
                    label: &str,
                    shortcut: $crate::enums::Shortcut,
                    flag: $crate::menu::MenuFlag,
                    sender: $crate::app::Sender<T>,
                    msg: T,
                ) -> i32 {
                    self.insert(idx, label, shortcut, flag, move |_| {
                        sender.send(msg.clone())
                    })
                }

                fn remove(&mut self, idx: i32) {
                    assert!(!self.was_deleted());
                    assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                    unsafe { [<$flname _remove>](self.inner, idx as i32) }
                }

                fn find_item(&self, name: &str) -> Option<MenuItem> {
                    assert!(!self.was_deleted());
                    let name = CString::safe_new(name);
                    unsafe {
                        let menu_item = [<$flname _get_item>](self.inner, name.as_ptr());
                        if menu_item.is_null() {
                            None
                        } else {
                            Some(MenuItem {
                                inner: menu_item,
                                size: Fl_Menu_Item_children(menu_item),
                            })
                        }
                    }
                }

                fn set_item(&mut self, item: &MenuItem) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_item>](self.inner, item.inner) != 0
                    }
                }

                fn find_index(&self, label: &str) -> i32 {
                    assert!(!self.was_deleted());
                    let label = CString::safe_new(label);
                    unsafe { [<$flname _find_index>](self.inner, label.as_ptr()) as i32 }
                }

                fn text_font(&self) -> $crate::enums::Font {
                    unsafe {
                        assert!(!self.was_deleted());
                        std::mem::transmute([<$flname _text_font>](self.inner))
                    }
                }

                fn set_text_font(&mut self, c: $crate::enums::Font) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_font>](self.inner, c.bits() as i32)
                    }
                }

                fn text_size(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _text_size>](self.inner) as i32
                    }
                }

                fn set_text_size(&mut self, c: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_size>](self.inner, c as i32)
                    }
                }

                fn text_color(&self) -> $crate::enums::Color {
                    unsafe {
                        assert!(!self.was_deleted());
                        std::mem::transmute([<$flname _text_color>](self.inner))
                    }
                }

                fn set_text_color(&mut self, c: $crate::enums::Color) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_text_color>](self.inner, c.bits() as u32)
                    }
                }

                fn add_choice(&mut self, text: &str) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        let arg2 = CString::safe_new(text);
                        [<$flname _add_choice>](
                            self.inner,
                            arg2.as_ptr() as *mut std::os::raw::c_char,
                        )
                    }
                }

                fn choice(&self) -> Option<String> {
                    unsafe {
                        assert!(!self.was_deleted());
                        let choice_ptr = [<$flname _get_choice>](self.inner);
                        if choice_ptr.is_null() {
                            None
                        } else {
                            Some(
                                CStr::from_ptr(choice_ptr as *mut std::os::raw::c_char)
                                    .to_string_lossy()
                                    .to_string(),
                            )
                        }
                    }
                }

                fn value(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _value>](self.inner)
                    }
                }

                fn set_value(&mut self, v: i32) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_value>](self.inner, v) != 0
                    }
                }

                fn clear(&mut self) {
                    unsafe {
                        assert!(!self.was_deleted());
                        if $crate::app::grab().is_none() {
                            [<$flname _clear>](self.inner);
                        }
                    }
                }

                unsafe fn unsafe_clear(&mut self) {
                    assert!(!self.was_deleted());
                    let sz = self.size();
                    if sz > 0 {
                        for i in 0..sz {
                            // Shouldn't fail
                            let c = self.at(i).unwrap();
                            let _ = c.user_data();
                        }
                    }
                    [<$flname _clear>](self.inner);
                }

                fn clear_submenu(&mut self, idx: i32) -> Result<(), FltkError> {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                        match [<$flname _clear_submenu>](self.inner, idx as i32) {
                            0 => Ok(()),
                            _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                        }
                    }
                }

                unsafe fn unsafe_clear_submenu(&mut self, idx: i32) -> Result<(), FltkError> {
                    assert!(!self.was_deleted());
                    let x = self.at(idx);
                    if x.is_none() {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    // Shouldn't fail
                    let x = x.unwrap();
                    if !x.is_submenu() {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    let mut i = idx;
                    loop {
                        // Shouldn't fail
                        let item = self.at(i).unwrap();
                        if item.label().is_none() {
                            break;
                        }
                        let _ = item.user_data();
                        i += 1;
                    }
                    match [<$flname _clear_submenu>](self.inner, idx as i32) {
                        0 => Ok(()),
                        _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                    }
                }

                fn size(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _size>](self.inner) as i32 }
                }

                fn text(&self, idx: i32) -> Option<String> {
                    assert!(!self.was_deleted());
                    assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                    unsafe {
                        let text = [<$flname _text>](self.inner, idx as i32);
                        if text.is_null() {
                            None
                        } else {
                            Some(
                                CStr::from_ptr(text as *mut std::os::raw::c_char)
                                    .to_string_lossy()
                                    .to_string(),
                            )
                        }
                    }
                }

                fn at(&self, idx: i32) -> Option<$crate::menu::MenuItem> {
                    assert!(!self.was_deleted());
                    assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                    unsafe {
                        let ptr =
                            [<$flname _at>](self.inner, idx as i32) as *mut Fl_Menu_Item;
                        if ptr.is_null() {
                            None
                        } else {
                            Some(MenuItem {
                                inner: ptr,
                                size: Fl_Menu_Item_children(ptr),
                            })
                        }
                    }
                }

                fn mode(&self, idx: i32) -> $crate::menu::MenuFlag {
                    assert!(!self.was_deleted());
                    assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                    unsafe { std::mem::transmute([<$flname _mode>](self.inner, idx as i32)) }
                }

                fn set_mode(&mut self, idx: i32, flag: $crate::menu::MenuFlag) {
                    assert!(!self.was_deleted());
                    assert!(self.size() != 0 && idx >= 0 && idx < self.size());
                    unsafe { [<$flname _set_mode>](self.inner, idx as i32, flag.bits()) }
                }

                fn end(&mut self) {
                    //
                }

                fn set_down_frame(&mut self, f: $crate::enums::FrameType) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_down_box>](self.inner, f as i32) }
                }

                fn down_frame(&self) -> $crate::enums::FrameType {
                    assert!(!self.was_deleted());
                    unsafe { std::mem::transmute([<$flname _down_box>](self.inner)) }
                }

                fn global(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _global>](self.inner) }
                }

                fn menu(&self) -> Option<$crate::menu::MenuItem> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _menu>](self.inner);
                        if ptr.is_null() {
                            None
                        } else {
                            Some(MenuItem {
                                inner: ptr as _,
                                size: Fl_Menu_Item_children(ptr),
                            })
                        }
                    }
                }

                unsafe fn set_menu(&mut self, item: $crate::menu::MenuItem) {
                    assert!(!self.was_deleted());
                    [<$flname _set_menu>](self.inner, item.inner)
                }

                fn item_pathname(&self, item: Option<&$crate::menu::MenuItem>) -> Result<String, FltkError> {
                    assert!(!self.was_deleted());
                    let item = if let Some(item) = item {
                        item.inner
                    } else {
                        std::ptr::null_mut()
                    };
                    let mut temp = vec![0u8; 256];
                    unsafe {
                        let ret = [<$flname _item_pathname>](self.inner, temp.as_mut_ptr() as _, 256, item);
                        if ret == 0 {
                            if let Some(pos) = temp.iter().position(|x| *x == 0) {
                                temp = temp.split_at(pos).0.to_vec();
                            }
                            Ok(String::from_utf8_lossy(&temp).to_string())
                        } else {
                            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                        }
                    }
                }
            }
        }
    };
}

pub use impl_menu_ext;
