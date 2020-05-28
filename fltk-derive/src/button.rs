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

    let gen = quote! {
        unsafe impl ButtonExt for #name {
            fn shortcut(&self) -> Shortcut {
                unsafe {
                    mem::transmute(#shortcut(self._inner))
                }
            }

            fn set_shortcut(&mut self, shortcut: Shortcut) {
                unsafe {
                    #set_shortcut(self._inner, shortcut as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    #clear(self._inner);
                }
            }

            fn is_set(&self) -> bool {
                unsafe {
                    match #value(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set(&mut self, flag: bool) {
                unsafe {
                    #set_value(self._inner, flag as i32)
                }
            }
        }
    };
    gen.into()
}
