use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;


pub fn impl_window_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let make_modal = Ident::new(
        format!("{}_{}", name_str, "make_modal").as_str(),
        name.span(),
    );
    let fullscreen = Ident::new(
        format!("{}_{}", name_str, "fullscreen").as_str(),
        name.span(),
    );
    let make_current = Ident::new(
        format!("{}_{}", name_str, "make_current").as_str(),
        name.span(),
    );
    let set_icon = Ident::new(format!("{}_{}", name_str, "set_icon").as_str(), name.span());
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let make_resizable = Ident::new(
        format!("{}_{}", name_str, "make_resizable").as_str(),
        name.span(),
    );
    let gen = quote! {
        unsafe impl WindowExt for #name {
            fn center_screen(mut self) -> Self {
                debug_assert!(self.width() != 0 && self.height() != 0, "center_screen requires the size of the widget to be known!");
                let (mut x, mut y) = screen_size();
                x = x - self.width() as f64;
                y = y - self.height() as f64;
                self.resize((x / 2.0) as i32, (y / 2.0) as i32, self.width(), self.height());
                self
            }

            fn make_modal(&mut self, val: bool) {
                unsafe { #make_modal(self._inner, val as u32) }
            }

            fn fullscreen(&mut self, val: bool) {
                unsafe { #fullscreen(self._inner, val as u32) }
            }

            fn make_current(&mut self) {
                unsafe { #make_current(self._inner) }
            }

            fn set_icon<Image: ImageExt>(&mut self, image: &Image) {
                unsafe { #set_icon(self._inner, image.as_ptr()) }
            }

            fn icon(&self) -> Option<Image> {
                unsafe {
                    let icon_ptr = #icon(self._inner);
                    if icon_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(icon_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }

            fn make_resizable(&mut self, val: bool) {
                if val {
                    unsafe {
                        #make_resizable(self._inner, self._inner as *mut raw::c_void)
                    }
                }
            }
        }
    };
    gen.into()
}