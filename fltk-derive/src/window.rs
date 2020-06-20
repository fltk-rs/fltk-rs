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
    let set_border = Ident::new(
        format!("{}_{}", name_str, "set_border").as_str(),
        name.span(),
    );
    let border = Ident::new(format!("{}_{}", name_str, "border").as_str(), name.span());
    let make_resizable = Ident::new(
        format!("{}_{}", name_str, "make_resizable").as_str(),
        name.span(),
    );
    let set_cursor = Ident::new(
        format!("{}_{}", name_str, "set_cursor").as_str(),
        name.span(),
    );
    let shown = Ident::new(format!("{}_{}", name_str, "shown").as_str(), name.span());
    let raw_handle = Ident::new(
        format!("{}_{}", name_str, "raw_handle").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl WindowExt for #name {
            fn center_screen(mut self) -> Self {
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "center_screen requires the size of the widget to be known!");
                let (mut x, mut y) = screen_size();
                x = x - self.width() as f64;
                y = y - self.height() as f64;
                self.resize((x / 2.0) as i32, (y / 2.0) as i32, self.width(), self.height());
                self
            }

            fn make_modal(&mut self, val: bool) {
                assert!(!self.was_deleted());
                unsafe { #make_modal(self._inner, val as u32) }
            }

            fn fullscreen(&mut self, val: bool) {
                assert!(!self.was_deleted());
                unsafe { #fullscreen(self._inner, val as u32) }
            }

            fn make_current(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #make_current(self._inner) }
            }

            fn set_icon<T: ImageExt>(&mut self, image: &T) {
                assert!(!self.was_deleted());
                assert!(!image.was_deleted());
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage icons are not supported!");
                let i = self.icon();
                unsafe { 
                    #set_icon(self._inner, image.as_ptr());
                    if let Some(mut i) = i {
                        unsafe { i.delete(); }
                    }
                }
            }

            fn icon(&self) -> Option<Image> {
                unsafe {
                    assert!(!self.was_deleted());
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
                        assert!(!self.was_deleted());
                        #make_resizable(self._inner, self._inner as *mut raw::c_void)
                    }
                }
            }

            fn set_cursor(&mut self, cursor: Cursor) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cursor(self._inner, cursor as i32)
                }
            }

            fn shown(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #shown(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_border(&mut self, flag: bool) {
                unsafe {
                    #set_border(self._inner, flag as i32)
                }
            }

            fn border(&self) -> bool {
                unsafe {
                    #border(self._inner) != 0
                }
            }

            unsafe fn raw_handle(&self) -> *const raw::c_void {
                unsafe {
                    assert!(!self.was_deleted());
                    #raw_handle(self._inner)
                }
            }
        }
    };
    gen.into()
}
