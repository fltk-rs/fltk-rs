#[macro_export]
macro_rules! impl_menu_ext {
    ($name: tt, $flname: tt) => {
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

        unsafe impl MenuExt for $name {
            fn add<F: FnMut(&mut Self) + 'static>(
                &mut self,
                name: &str,
                shortcut: Shortcut,
                flag: MenuFlag,
                mut cb: F,
            ) -> i32 {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(name);
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                        let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                            data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                        let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                        let _ =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    concat_idents!($flname, _add)(
                        self.inner,
                        temp.as_ptr(),
                        shortcut.bits() as i32,
                        callback,
                        data,
                        flag as i32,
                    )
                }
            }

            fn insert<F: FnMut(&mut Self) + 'static>(
                &mut self,
                idx: i32,
                name: &str,
                shortcut: Shortcut,
                flag: MenuFlag,
                mut cb: F,
            ) -> i32 {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(name);
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                        let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                            data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                        let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                        let _ =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    concat_idents!($flname, _insert)(
                        self.inner,
                        idx as i32,
                        temp.as_ptr(),
                        shortcut.bits() as i32,
                        callback,
                        data,
                        flag as i32,
                    )
                }
            }

            fn add_emit<T: 'static + Clone + Send + Sync>(
                &mut self,
                label: &str,
                shortcut: Shortcut,
                flag: crate::menu::MenuFlag,
                sender: crate::app::Sender<T>,
                msg: T,
            ) -> i32 {
                self.add(label, shortcut, flag, move |_| sender.send(msg.clone()))
            }

            fn insert_emit<T: 'static + Clone + Send + Sync>(
                &mut self,
                idx: i32,
                label: &str,
                shortcut: Shortcut,
                flag: crate::menu::MenuFlag,
                sender: crate::app::Sender<T>,
                msg: T,
            ) -> i32 {
                self.insert(idx, label, shortcut, flag, move |_| {
                    sender.send(msg.clone())
                })
            }

            fn remove(&mut self, idx: i32) {
                assert!(!self.was_deleted());
                let idx = if idx < self.size() {
                    idx
                } else {
                    self.size() - 1
                };
                unsafe { concat_idents!($flname, _remove)(self.inner, idx as i32) }
            }

            fn find_item(&self, name: &str) -> Option<MenuItem> {
                assert!(!self.was_deleted());
                let name = CString::safe_new(name);
                unsafe {
                    let menu_item = concat_idents!($flname, _get_item)(self.inner, name.as_ptr());
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
                    concat_idents!($flname, _set_item)(self.inner, item.inner) != 0
                }
            }

            fn find_index(&self, label: &str) -> i32 {
                assert!(!self.was_deleted());
                let label = CString::safe_new(label);
                unsafe { concat_idents!($flname, _find_index)(self.inner, label.as_ptr()) as i32 }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _text_font)(self.inner))
                }
            }

            fn set_text_font(&mut self, c: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_text_font)(self.inner, c.bits() as i32)
                }
            }

            fn text_size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _text_size)(self.inner) as i32
                }
            }

            fn set_text_size(&mut self, c: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_text_size)(self.inner, c as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(concat_idents!($flname, _text_color)(self.inner))
                }
            }

            fn set_text_color(&mut self, c: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_text_color)(self.inner, c.bits() as u32)
                }
            }

            fn add_choice(&mut self, text: &str) {
                unsafe {
                    assert!(!self.was_deleted());
                    let arg2 = CString::safe_new(text);
                    concat_idents!($flname, _add_choice)(
                        self.inner,
                        arg2.as_ptr() as *mut raw::c_char,
                    )
                }
            }

            fn choice(&self) -> Option<String> {
                unsafe {
                    assert!(!self.was_deleted());
                    let choice_ptr = concat_idents!($flname, _get_choice)(self.inner);
                    if choice_ptr.is_null() {
                        None
                    } else {
                        Some(
                            CStr::from_ptr(choice_ptr as *mut raw::c_char)
                                .to_string_lossy()
                                .to_string(),
                        )
                    }
                }
            }

            fn value(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _value)(self.inner)
                }
            }

            fn set_value(&mut self, v: i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _set_value)(self.inner, v) != 0
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    concat_idents!($flname, _clear)(self.inner);
                }
            }

            unsafe fn unsafe_clear(&mut self) {
                assert!(!self.was_deleted());
                let sz = self.size();
                if sz > 0 {
                    for i in 0..sz {
                        // Shouldn't fail
                        let mut c = self.at(i).unwrap();
                        let _ = c.user_data();
                    }
                }
                concat_idents!($flname, _clear)(self.inner);
            }

            fn clear_submenu(&mut self, idx: i32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    match concat_idents!($flname, _clear_submenu)(self.inner, idx as i32) {
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
                    let mut item = self.at(i).unwrap();
                    if item.label().is_none() {
                        break;
                    }
                    let _ = item.user_data();
                    i += 1;
                }
                match concat_idents!($flname, _clear_submenu)(self.inner, idx as i32) {
                    0 => Ok(()),
                    _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                }
            }

            fn size(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _size)(self.inner) as i32 }
            }

            fn text(&self, idx: i32) -> Option<String> {
                assert!(!self.was_deleted());
                unsafe {
                    let text = concat_idents!($flname, _text)(self.inner, idx as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(
                            CStr::from_ptr(text as *mut raw::c_char)
                                .to_string_lossy()
                                .to_string(),
                        )
                    }
                }
            }

            fn at(&self, idx: i32) -> Option<crate::menu::MenuItem> {
                assert!(!self.was_deleted());
                if idx >= self.size() || idx < 0 {
                    return None;
                }
                unsafe {
                    let ptr =
                        concat_idents!($flname, _at)(self.inner, idx as i32) as *mut Fl_Menu_Item;
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

            fn mode(&self, idx: i32) -> crate::menu::MenuFlag {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(concat_idents!($flname, _mode)(self.inner, idx as i32)) }
            }

            fn set_mode(&mut self, idx: i32, flag: crate::menu::MenuFlag) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _set_mode)(self.inner, idx as i32, flag as i32) }
            }

            fn end(&mut self) {
                //
            }

            fn set_down_frame(&mut self, f: FrameType) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _set_down_box)(self.inner, f as i32) }
            }

            fn down_frame(&self) -> FrameType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(concat_idents!($flname, _down_box)(self.inner)) }
            }

            fn global(&mut self) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _global)(self.inner) }
            }

            fn menu(&self) -> Option<crate::menu::MenuItem> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = concat_idents!($flname, _menu)(self.inner);
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

            unsafe fn set_menu(&mut self, item: crate::menu::MenuItem) {
                assert!(!self.was_deleted());
                unsafe { concat_idents!($flname, _set_menu)(self.inner, item.inner) }
            }
        }
    };
}
