use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_menu_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(format!("{}", name_str).as_str(), name.span());
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
            fn add(&mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, mut cb: Box<dyn FnMut()>) {
                // debug_assert!(
                //     self.top_window().unwrap().takes_events() && self.takes_events(),
                //     "Handling events requires that the window and widget be active!"
                // );
                let temp = CString::new(name).unwrap();
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: Fl_Callback = Some(shim);
                    assert!(!self.was_deleted());
                    #add(self._inner, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn insert(&mut self, idx: u32, name: &str, shortcut: Shortcut, flag: MenuFlag, cb: Box<dyn FnMut()>) {
                // debug_assert!(
                //     self.top_window().unwrap().takes_events() && self.takes_events(),
                //     "Handling events requires that the window and widget be active!"
                // );
                let temp = CString::new(name).unwrap();
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: Fl_Callback = Some(shim);
                    assert!(!self.was_deleted());
                    #insert(self._inner, idx as i32, temp.as_ptr(), shortcut as i32, callback, data, flag as i32);
                }
            }

            fn remove(&mut self, idx: u32) {
                assert!(!self.was_deleted());
                assert!(idx < self.size());
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #remove(self._inner, idx as i32)
                }
            }

            fn find_item(&self, name: &str) -> Option<MenuItem> {
                let name = CString::new(name).unwrap().clone();
                unsafe {
                    assert!(!self.was_deleted());
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
                let label = CString::new(label).unwrap().into_raw() as *mut raw::c_char;
                unsafe {
                    #find_index(self._inner, label) as u32
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
                    debug_assert!(c <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
                    let arg2 = CString::new(text).unwrap();
                    assert!(!self.was_deleted());
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
                    let sz = self.size();
                    let mut v: Vec<MenuItem> = vec![];
                    if sz > 0 {
                        for i in 0..sz {
                            v.push(self.at(i).unwrap());
                        }
                    }
                    #clear(self._inner);
                    if sz > 0 {
                        for mut child in v {
                            child.unset_callback();
                            child._inner = 0 as *mut Fl_Menu_Item;
                        }
                    }
                }
            }

            fn clear_submenu(&mut self, idx: u32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    debug_assert!(
                        idx <= std::i32::MAX as u32,
                        "u32 entries have to be < std::i32::MAX for compatibility!"
                    );
                    let x = self.at(idx);
                    if x.is_none() {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    let x = x.unwrap();
                    if !x.is_submenu() {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    let mut i = idx;
                    loop {
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
            }
            

            fn size(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #size(self._inner) as u32
                }
            }

            fn text(&self, idx: u32) -> Option<String> {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    mem::transmute(#mode(self._inner, idx as i32))
                }
            }

            fn set_mode(&mut self, idx: u32, flag: crate::menu::MenuFlag) {
                assert!(!self.was_deleted());
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #set_mode(self._inner, idx as i32, flag as i32)
                }
            }
        }
    };
    gen.into()
}
