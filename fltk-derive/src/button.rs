use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_button_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());

    let shortcut = Ident::new(format!("{}_{}", name_str, "shortcut").as_str(), name.span());
    let set_shortcut = Ident::new(
        format!("{}_{}", name_str, "set_shortcut").as_str(),
        name.span(),
    );
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let down_box = Ident::new(format!("{}_{}", name_str, "down_box").as_str(), name.span());
    let set_down_box = Ident::new(format!("{}_{}", name_str, "set_down_box").as_str(), name.span());

    let gen = quote! {
        unsafe impl ButtonExt for #name {
            fn shortcut(&self) -> Shortcut {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#shortcut(self._inner))
                }
            }

            fn set_shortcut(&mut self, shortcut: Shortcut) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_shortcut(self._inner, shortcut.bits() as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self._inner);
                }
            }

            fn is_set(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    #value(self._inner)  != 0
                }
            }

            fn set(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_value(self._inner, flag as i32)
                }
            }

            fn set_down_frame(&mut self, f: FrameType) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_down_box(self._inner, f as i32)
                }
            }
            
            fn down_frame(&self) -> FrameType {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#down_box(self._inner))
                }
            }
        }
    };
    gen.into()
}
