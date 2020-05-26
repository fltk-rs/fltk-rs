use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;


pub fn impl_browser_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let move_item = Ident::new(format!("{}_{}", name_str, "move").as_str(), name.span());
    let swap = Ident::new(format!("{}_{}", name_str, "swap").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let size = Ident::new(format!("{}_{}", name_str, "size").as_str(), name.span());
    let set_size = Ident::new(format!("{}_{}", name_str, "set_size").as_str(), name.span());
    let select = Ident::new(format!("{}_{}", name_str, "select").as_str(), name.span());
    let selected = Ident::new(format!("{}_{}", name_str, "selected").as_str(), name.span());
    let text = Ident::new(format!("{}_{}", name_str, "text").as_str(), name.span());
    let set_text = Ident::new(format!("{}_{}", name_str, "set_text").as_str(), name.span());
    let load_file = Ident::new(
        format!("{}_{}", name_str, "load_file").as_str(),
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
    let set_icon = Ident::new(format!("{}_{}", name_str, "set_icon").as_str(), name.span());
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let remove_icon = Ident::new(
        format!("{}_{}", name_str, "remove_icon").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl BrowserExt for #name {
            fn remove(&mut self, line: u32) {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #remove(self._inner, line as i32)
                }
            }

            fn add(&mut self, item: &str) {
                let item = CString::new(item).unwrap();
                unsafe {
                    #add(self._inner, item.as_ptr())
                }
            }

            fn insert(&mut self, line: u32, item: &str) {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                let item = CString::new(item).unwrap();
                unsafe {
                    #insert(self._inner, line as i32, item.as_ptr())
                }
            }

            fn move_item(&mut self, to: u32, from: u32) {
                debug_assert!(to <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(from <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #move_item(self._inner, to as i32, from as i32)
                }
            }

            fn swap(&mut self, a: u32, b: u32) {
                debug_assert!(a <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                debug_assert!(b <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #swap(self._inner, a as i32, b as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    #clear(self._inner)
                }
            }

            fn size(&self) -> u32 {
                unsafe {
                    #size(self._inner) as u32
                }
            }

            fn set_size(&mut self, w: i32, h: i32) {
                unsafe {
                    #set_size(self._inner, w, h)
                }
            }

            fn select(&mut self, line: u32) {
                assert!(line > 0);
                if line < self.size() {
                    unsafe {
                        #select(self._inner, line as i32);
                    }
                }
            }

            fn selected(&self, line: u32) -> bool {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    match #selected(self._inner, line as i32) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn text(&self, line: u32) -> Option<String> {
                assert!(line > 0);
                debug_assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    let text = #text(self._inner, line as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(text as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn set_text(&mut self, line: u32, txt: &str) {
                assert!(line > 0);
                assert!(line <= self.size());
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #set_text(self._inner, line as i32, txt.as_ptr())
                }
            }

            fn load(&mut self, path: &std::path::Path) -> Result<(), FltkError> {
                if !path.exists() {
                    return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
                }
                let path = path.to_str().unwrap();
                let path = CString::new(path)?;
                unsafe {
                    #load_file(self._inner, path.as_ptr());
                    Ok(())
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, c: u32) {
                unsafe {
                    #set_text_size(self._inner, c as i32)
                }
            }

            fn set_icon<Img: ImageExt>(&mut self, line: u32, image: &Img) {
                assert!(line > 0);
                unsafe {
                    #set_icon(self._inner, line as i32, image.as_ptr())
                }
            }

            fn icon(&self, line: u32) -> Option<Image> {
                assert!(line > 0);
                unsafe {
                    let icon_ptr = #icon(self._inner, line as i32);
                    if icon_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(icon_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }

            fn remove_icon(&mut self, line: u32) {
                assert!(line > 0);
                unsafe {
                    #remove_icon(self._inner, line as i32)
                }
            }
        }
    };
    gen.into()
}
