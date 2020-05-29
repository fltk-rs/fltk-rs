use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_menu_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let get_item = Ident::new(format!("{}_{}", name_str, "get_item").as_str(), name.span());
    let set_item = Ident::new(format!("{}_{}", name_str, "set_item").as_str(), name.span());
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
                    #clear(self._inner)
                }
            }

            fn clear_submenu(&mut self, idx: u32) -> Result<(), FltkError> {
                debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    match #clear_submenu(self._inner, idx as i32) {
                        0 => Ok(()),
                        _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                    }
                }
            }
        }
    };
    gen.into()
}
