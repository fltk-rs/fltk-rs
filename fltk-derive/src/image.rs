use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_image_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(name_str.as_str(), name.span());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let draw = Ident::new(format!("{}_{}", name_str, "draw").as_str(), name.span());
    let width = Ident::new(format!("{}_{}", name_str, "width").as_str(), name.span());
    let height = Ident::new(format!("{}_{}", name_str, "height").as_str(), name.span());
    let delete = Ident::new(format!("{}_{}", name_str, "delete").as_str(), name.span());
    let count = Ident::new(format!("{}_{}", name_str, "count").as_str(), name.span());
    let data = Ident::new(format!("{}_{}", name_str, "data").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());
    let scale = Ident::new(format!("{}_{}", name_str, "scale").as_str(), name.span());
    let data_w = Ident::new(format!("{}_{}", name_str, "data_w").as_str(), name.span());
    let data_h = Ident::new(format!("{}_{}", name_str, "data_h").as_str(), name.span());
    let d = Ident::new(format!("{}_{}", name_str, "d").as_str(), name.span());
    let ld = Ident::new(format!("{}_{}", name_str, "ld").as_str(), name.span());
    let inactive = Ident::new(format!("{}_{}", name_str, "inactive").as_str(), name.span());

    let gen = quote! {
        unsafe impl Sync for #name {}
        unsafe impl Send for #name {}

        impl Clone for #name {
            fn clone(&self) -> Self {
                assert!(!self.was_deleted());
                let x = self._refcount.fetch_add(1, Ordering::Relaxed);
                #name { _inner: self._inner, _refcount: AtomicUsize::new(x) }
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                if !self.was_deleted() {
                    self._refcount.fetch_sub(1, Ordering::Relaxed);
                    if *self._refcount.get_mut() == 0 {
                        unsafe {
                            #delete(self._inner);
                            self._inner = std::ptr::null_mut();
                        }
                    }
                }
            }
        }

        unsafe impl ImageExt for #name {
            fn copy(&self) -> Self {
                assert!(!self.was_deleted());
                unsafe {
                    let img = #copy(self._inner);
                    assert!(!img.is_null());
                    #name {
                        _inner: img,
                        _refcount: AtomicUsize::new(1)
                    }
                }
            }

            fn draw(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32) {
                assert!(!self.was_deleted());
                unsafe { #draw(self._inner, arg2, arg3, arg4, arg5) }
            }

            fn width(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #width(self._inner)
                }
            }

            fn height(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #height(self._inner)
                }
            }

            fn w(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #width(self._inner)
                }
            }

            fn h(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #height(self._inner)
                }
            }

            unsafe fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image {
                assert!(!self.was_deleted());
                self._inner as *mut fltk_sys::image::Fl_Image
            }

            unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
                assert!(!ptr.is_null());
                #name {
                    _inner: ptr as *mut #ptr_name,
                    _refcount: AtomicUsize::new(2),
                }
            }

            fn to_rgb_data(&self) -> Vec<u8> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #data(self._inner);
                    assert!(!ptr.is_null());
                    assert!(!(*ptr).is_null());
                    let cnt = self.data_w() * self.data_h() * self.depth() as u32;
                    let ret: &[u8] = std::slice::from_raw_parts(*ptr as *const u8, cnt as usize);
                    ret.to_vec()
                }
            }

            fn to_raw_data(&self) -> *const *const u8 {
                assert!(!self.was_deleted());
                unsafe {
                    #data(self._inner) as *const *const u8
                }
            }

            fn to_rgb(&self) -> Result<crate::image::RgbImage, FltkError> {
                assert!(!self.was_deleted());
                let data = self.to_rgb_data();
                unsafe { RgbImage::new(&data, self.data_w(), self.data_h(), self.depth()) }
            }

            fn scale(&mut self, width: i32, height: i32, proportional: bool, can_expand: bool) {
                assert!(!self.was_deleted());
                unsafe {
                    #scale(self._inner, width, height, proportional as i32, can_expand as i32)
                }
            }

            fn count(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #count(self._inner) as u32
                }
            }

            fn data_w(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #data_w(self._inner) as u32
                }
            }

            fn data_h(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #data_h(self._inner) as u32
                }
            }

            fn depth(&self) -> ColorDepth {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#d(self._inner) as u8)
                }
            }

            fn ld(&self) -> u32 {
                assert!(!self.was_deleted());
                unsafe {
                    #ld(self._inner) as u32
                }
            }

            fn inactive(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #inactive(self._inner)
                }
            }

            unsafe fn delete(mut img: Self) {
                assert!(!img._inner.is_null());
                #delete(img._inner);
                img._inner = std::ptr::null_mut() as *mut #ptr_name;
            }

            unsafe fn increment_arc(&mut self) {
                assert!(!self.was_deleted());
                self._refcount.fetch_add(1, Ordering::Relaxed);
            }

            unsafe fn decrement_arc(&mut self) {
                assert!(!self.was_deleted());
                self._refcount.fetch_sub(1, Ordering::Relaxed);
            }

            fn was_deleted(&self) -> bool {
                self._inner.is_null()
            }

            unsafe fn into_image<I: ImageExt>(self) -> I {
                I::from_image_ptr(self._inner as *mut _)
            }
        }
    };
    gen.into()
}
