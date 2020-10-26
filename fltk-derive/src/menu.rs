use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_menu_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(name_str.as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let get_item = Ident::new(format!("{}_{}", name_str, "get_item").as_str(), name.span());
    let set_item = Ident::new(format!("{}_{}", name_str, "set_item").as_str(), name.span());
    let find_index = Ident::new(
        format!("{}_{}", name_str, "find_index").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let add_choice = Ident::new(
        format!("{}_{}", name_str, "add_choice").as_str(),
        name.span(),
    );
    let get_choice = Ident::new(
        format!("{}_{}", name_str, "get_choice").as_str(),
        name.span(),
    );
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let clear_submenu = Ident::new(
        format!("{}_{}", name_str, "clear_submenu").as_str(),
        name.span(),
    );
    let size = Ident::new(format!("{}_{}", name_str, "size").as_str(), name.span());
    let text = Ident::new(format!("{}_{}", name_str, "text").as_str(), name.span());
    let at = Ident::new(format!("{}_{}", name_str, "at").as_str(), name.span());
    let mode = Ident::new(format!("{}_{}", name_str, "mode").as_str(), name.span());
    let set_mode = Ident::new(format!("{}_{}", name_str, "set_mode").as_str(), name.span());

    let gen = quote! {
        unsafe impl MenuExt for #name {
            fn add<F: FnMut() + 'static>(&mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, mut cb: F) {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(name);
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
                        let f: &mut (dyn FnMut()) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    #add(self._inner, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn add2<F: FnMut(&mut Self) + 'static>(&mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, mut cb: F) {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(name);
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = crate::widget::Widget::from_raw(wid as *mut _);
                        let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> = data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                        let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    #add(self._inner, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn insert<F: FnMut() + 'static>(&mut self, idx: u32, label: &str, shortcut: Shortcut, flag: MenuFlag, cb: F) {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(label);
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
                        let f: &mut (dyn FnMut()) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    #insert(self._inner, idx as i32, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn insert2<F: FnMut(&mut Self) + 'static>(&mut self, idx: u32, name: &str, shortcut: Shortcut, flag: MenuFlag, mut cb: F) {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(name);
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = crate::widget::Widget::from_raw(wid as *mut _);
                        let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> = data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                        let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    #insert(self._inner, idx as i32, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn add_emit<T: 'static + Copy + Send + Sync>(
                &mut self,
                label: &str,
                shortcut: Shortcut,
                flag: crate::menu::MenuFlag,
                sender: crate::app::Sender<T>,
                msg: T,
            ) {
                self.add(label, shortcut, flag, move|| sender.send(msg))
            }

            fn insert_emit<T: 'static + Copy + Send + Sync>(
                &mut self,
                idx: u32,
                label: &str,
                shortcut: Shortcut,
                flag: crate::menu::MenuFlag,
                sender: crate::app::Sender<T>,
                msg: T,
            ) {
                self.insert(idx, label, shortcut, flag, move|| sender.send(msg))
            }

            fn remove(&mut self, idx: u32) {
                assert!(!self.was_deleted());
                let idx = if idx < self.size() { idx } else { self.size() - 1 };
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                unsafe {
                    #remove(self._inner, idx as i32)
                }
            }

            fn find_item(&self, name: &str) -> Option<MenuItem> {
                assert!(!self.was_deleted());
                let name = CString::safe_new(name);
                unsafe {
                    let menu_item = #get_item(
                        self._inner,
                        name.as_ptr());
                    if menu_item.is_null() {
                        None
                    } else {
                        Some(MenuItem {
                            _inner: menu_item,
                            _parent: self as *const _ as *const MenuBar,
                            _alloc: false,
                        })
                    }
                }
            }

            fn set_item(&mut self, item: &MenuItem) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_item(
                        self._inner,
                        item._inner) != 0
                }
            }

            fn find_index(&self, label: &str) -> u32 {
                assert!(!self.was_deleted());
                let label = CString::safe_new(label);
                unsafe {
                    #find_index(self._inner, label.as_ptr()) as u32
                }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#text_font(self._inner))
                }
            }

            fn set_text_font(&mut self, c: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_font(self._inner, c as i32)
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, c: u32) {
                unsafe {
                    debug_assert!(c <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    #set_text_size(self._inner, c as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#text_color(self._inner))
                }
            }

            fn set_text_color(&mut self, c: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_color(self._inner, c as u32)
                }
            }

            fn add_choice(&mut self, text: &str) {
                unsafe {
                    assert!(!self.was_deleted());
                    let arg2 = CString::safe_new(text);
                    #add_choice(self._inner, arg2.as_ptr() as *mut raw::c_char)
                }
            }

            fn choice(&self) -> Option<String> {
                unsafe {
                    assert!(!self.was_deleted());
                    let choice_ptr = #get_choice(self._inner);
                    if choice_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(choice_ptr as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn value(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #value(self._inner)
                }
            }

            fn set_value(&mut self,v:i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_value(self._inner,v) != 0
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self._inner);
                }
            }

            unsafe fn unsafe_clear(&mut self) {
                assert!(!self.was_deleted());
                let sz = self.size();
                if sz > 0 {
                    for i in 0..sz {
                        // Shouldn't fail
                        let mut c = self.at(i).unwrap();
                        c.unset_callback();
                    }
                }
                #clear(self._inner);
            }

            fn clear_submenu(&mut self, idx: u32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    debug_assert!(
                        idx <= std::isize::MAX as u32,
                        "u32 entries have to be < std::isize::MAX for compatibility!"
                    );
                    match #clear_submenu(self._inner, idx as i32) {
                        0 => Ok(()),
                        _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                    }
                }
            }

            unsafe fn unsafe_clear_submenu(&mut self, idx: u32) -> Result<(), FltkError> {
                assert!(!self.was_deleted());
                debug_assert!(
                    idx <= std::isize::MAX as u32,
                    "u32 entries have to be < std::isize::MAX for compatibility!"
                );
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
                    item.unset_callback();
                    i += 1;
                }
                match #clear_submenu(self._inner, idx as i32) {
                    0 => Ok(()),
                    _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                }
            }


            fn size(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #size(self._inner) as u32
                }
            }

            fn text(&self, idx: u32) -> Option<String> {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                unsafe {
                    let text = #text(self._inner, idx as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(text as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn at(&self, idx: u32) -> Option<crate::menu::MenuItem> {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                if idx >= self.size() {
                    return None;
                }
                unsafe {
                    let ptr = #at(self._inner, idx as i32) as *mut Fl_Menu_Item;
                    if ptr.is_null() {
                        None
                    } else {
                        Some(MenuItem {
                            _inner: ptr,
                            _parent: self as *const _ as *const MenuBar,
                            _alloc: false,
                        })
                    }
                }
            }

            fn mode(&self, idx: u32) -> crate::menu::MenuFlag {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                unsafe {
                    mem::transmute(#mode(self._inner, idx as i32))
                }
            }

            fn set_mode(&mut self, idx: u32, flag: crate::menu::MenuFlag) {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                unsafe {
                    #set_mode(self._inner, idx as i32, flag as i32)
                }
            }
        }
    };
    gen.into()
}
