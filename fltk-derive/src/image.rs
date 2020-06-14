use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_image_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let draw = Ident::new(format!("{}_{}", name_str, "draw").as_str(), name.span());
    let width = Ident::new(format!("{}_{}", name_str, "width").as_str(), name.span());
    let height = Ident::new(format!("{}_{}", name_str, "height").as_str(), name.span());
    let delete = Ident::new(format!("{}_{}", name_str, "delete").as_str(), name.span());
    let count = Ident::new(format!("{}_{}", name_str, "count").as_str(), name.span());
    let data = Ident::new(format!("{}_{}", name_str, "data").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());
    let scale = Ident::new(format!("{}_{}", name_str, "scale").as_str(), name.span());

    let gen = quote! {
        unsafe impl Sync for #name {}
        unsafe impl Send for #name {}

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe { #delete(self._inner) }
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                self.copy()
            }
        }

        unsafe impl ImageExt for #name {
            fn copy(&self) -> Self {
                unsafe {
                    let img = #copy(self._inner);
                    assert!(!img.is_null());
                    #name {
                        _inner: img,
                    }
                }
            }

            fn draw(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32) {
                unsafe { #draw(self._inner, arg2, arg3, arg4, arg5) }
            }

            fn width(&self) -> i32 {
                unsafe {
                    #width(self._inner)
                }
            }

            fn height(&self) -> i32 {
                unsafe {
                    #height(self._inner)
                }
            }

            unsafe fn as_ptr(&self) -> *mut raw::c_void {
                unsafe {
                    mem::transmute(self._inner)
                }
            }

            unsafe fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image {
                unsafe {
                    mem::transmute(self._inner)
                }
            }

            unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
                unsafe {
                    assert!(!ptr.is_null());
                    #name {
                        _inner: mem::transmute(ptr),
                    }
                }
            }

            unsafe fn to_raw_rgb_data(&self) -> Vec<u8> {
                unsafe {
                    let ptr = #data(self._inner);
                    let cnt = #width(self._inner) * #height(self._inner) * 3;
                    let ret: &[u8] = std::slice::from_raw_parts(ptr as *const u8, cnt as usize);
                    ret.to_vec()
                }
            }

            fn scale(&mut self, width: i32, height: i32, proportional: bool, can_expand: bool) {
                unsafe {
                    #scale(self._inner, width, height, proportional as i32, can_expand as i32)
                }
            }

            fn count(&self) -> u32 {
                unsafe {
                    #count(self._inner) as u32
                }
            }

        }
    };
    gen.into()
}
