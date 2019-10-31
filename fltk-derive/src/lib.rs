#![recursion_limit="256"]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use quote::*;
use syn::*;
use proc_macro::TokenStream;
use std::{mem, ptr, ffi};

#[proc_macro_derive(WidgetTrait)]
pub fn widget_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_trait(&ast)
}

#[proc_macro_derive(WidgetType)]
pub fn widget_type_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_type(&ast)
}

#[proc_macro_derive(GroupTrait)]
pub fn group_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_group_trait(&ast)
}

fn impl_widget_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut name_str = name.to_string();
    if name_str == "Frame" {
        name_str = String::from("Box");
    }
    let name_lower = Ident::new(name.to_string().to_lowercase().as_str(), name.span());

    let new = Ident::new(format!("Fl_{}_{}", name_str, "new").as_str(), name.span());
    let set_label = Ident::new(format!("Fl_{}_{}", name_str, "set_label").as_str(), name.span());
    let redraw = Ident::new(format!("Fl_{}_{}", name_str, "redraw").as_str(), name.span());
    let show = Ident::new(format!("Fl_{}_{}", name_str, "show").as_str(), name.span());
    let hide = Ident::new(format!("Fl_{}_{}", name_str, "hide").as_str(), name.span());
    let activate = Ident::new(format!("Fl_{}_{}", name_str, "activate").as_str(), name.span());
    let deactivate = Ident::new(format!("Fl_{}_{}", name_str, "deactivate").as_str(), name.span());
    let redraw_label = Ident::new(format!("Fl_{}_{}", name_str, "redraw_label").as_str(), name.span());
    let resize = Ident::new(format!("Fl_{}_{}", name_str, "resize").as_str(), name.span());
    let tooltip = Ident::new(format!("Fl_{}_{}", name_str, "tooltip").as_str(), name.span());
    let set_tooltip = Ident::new(format!("Fl_{}_{}", name_str, "set_tooltip").as_str(), name.span());
    let get_type = Ident::new(format!("Fl_{}_{}", name_str, "get_type").as_str(), name.span());
    let set_type = Ident::new(format!("Fl_{}_{}", name_str, "set_type").as_str(), name.span());
    let color = Ident::new(format!("Fl_{}_{}", name_str, "color").as_str(), name.span());
    let set_color = Ident::new(format!("Fl_{}_{}", name_str, "set_color").as_str(), name.span());
    let label_color = Ident::new(format!("Fl_{}_{}", name_str, "label_color").as_str(), name.span());
    let set_label_color = Ident::new(format!("Fl_{}_{}", name_str, "set_label_color").as_str(), name.span());
    let label_font = Ident::new(format!("Fl_{}_{}", name_str, "label_font").as_str(), name.span());
    let set_label_font = Ident::new(format!("Fl_{}_{}", name_str, "set_label_font").as_str(), name.span());
    let label_size = Ident::new(format!("Fl_{}_{}", name_str, "label_size").as_str(), name.span());
    let set_label_size = Ident::new(format!("Fl_{}_{}", name_str, "set_label_size").as_str(), name.span());
    let label_type = Ident::new(format!("Fl_{}_{}", name_str, "label_type").as_str(), name.span());
    let set_label_type = Ident::new(format!("Fl_{}_{}", name_str, "set_label_type").as_str(), name.span());
    let get_box = Ident::new(format!("Fl_{}_{}", name_str, "box").as_str(), name.span());
    let set_box = Ident::new(format!("Fl_{}_{}", name_str, "set_box").as_str(), name.span());

    let gen = quote! {
        impl WidgetTrait for #name {
            fn new() -> #name {
                    #name {
                        _inner: ptr::null_mut(),
                        _x: 0,
                        _y: 0,
                        _width: 0,
                        _height: 0,
                        _title: ffi::CString::new("").unwrap(),
                    }
                }

            fn set(mut self, x: i32, y: i32, width: i32, height: i32, title: &str) -> #name {
                self._x = x;
                self._y = y;
                self._width = width;
                self._height = height;
                self._title = ffi::CString::new(title).unwrap();
                self._inner = unsafe {
                    fltk_sys::#name_lower::#new(
                        self._x,
                        self._y,
                        self._width,
                        self._height,
                        self._title.as_ptr() as *const libc::c_char,
                    )
                };
                self
            }

            fn set_label(&mut self, title: &str) {
                self._title = ffi::CString::new(title).unwrap();
                unsafe {
                    fltk_sys::#name_lower::#set_label(
                        self._inner,
                        self._title.as_ptr() as *const libc::c_char,
                    )
                }
            }

            fn redraw(&mut self) {
                unsafe {
                    fltk_sys::#name_lower::#redraw(self._inner);
                }
            }

            fn show(&mut self) {
                unsafe { fltk_sys::#name_lower::#show(self._inner) }
            }

            fn hide(&mut self) {
                unsafe { fltk_sys::#name_lower::#hide(self._inner) }
            }

            fn x(&self) -> i32 {
                self._x
            }

            fn y(&self) -> i32 {
                self._y
            }

            fn width(&self) -> i32 {
                self._width
            }

            fn height(&self) -> i32 {
                self._height
            }

            fn label(&self) -> ffi::CString {
                self._title.clone()
            }

            fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
                unsafe { mem::transmute(self._inner) }
            }

            fn activate(&mut self) {
                unsafe { fltk_sys::#name_lower::#activate(self._inner) }
            }

            fn deactivate(&mut self) {
                unsafe { fltk_sys::#name_lower::#deactivate(self._inner) }
            }

            fn redraw_label(&mut self) {
                unsafe { fltk_sys::#name_lower::#redraw_label(self._inner) }
            }

            fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                unsafe { fltk_sys::#name_lower::#resize(self._inner, x, y, width, height) }
            }

            fn tooltip(&self) -> ffi::CString {
                unsafe {
                    ffi::CString::from_raw(
                        fltk_sys::#name_lower::#tooltip(self._inner) as *mut libc::c_char
                    )
                }
            }

            fn set_tooltip(&mut self, txt: &str) {
                let txt = ffi::CString::new(txt).unwrap();
                unsafe {
                    fltk_sys::#name_lower::#set_tooltip(
                        self._inner,
                        txt.as_ptr() as *const libc::c_char,
                    )
                }
            }

            fn get_type<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(fltk_sys::#name_lower::#get_type(self._inner)) }
            }

            fn set_type<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    fltk_sys::#name_lower::#set_type(self._inner, typ.to_int());
                }
            }

            fn color(&self) -> Color {
                unsafe { mem::transmute(fltk_sys::#name_lower::#color(self._inner)) }
            }

            fn set_color(&mut self, color: Color) {
                unsafe { fltk_sys::#name_lower::#set_color(self._inner, color as i32) }
            }

            fn label_color(&self) -> Color {
                unsafe { mem::transmute(fltk_sys::#name_lower::#label_color(self._inner)) }
            }

            fn set_label_color(&mut self, color: Color) {
                unsafe { fltk_sys::#name_lower::#set_label_color(self._inner, color as i32) }
            }

            fn label_font(&self) -> Font {
                unsafe { mem::transmute(fltk_sys::#name_lower::#label_font(self._inner)) }
            }

            fn set_label_font(&mut self, font: Font) {
                unsafe { fltk_sys::#name_lower::#set_label_color(self._inner, font as i32) }
            }

            fn label_size(&self) -> usize {
                unsafe { fltk_sys::#name_lower::#label_size(self._inner) as usize }
            }

            fn set_label_size(&mut self, sz: usize) {
                unsafe { fltk_sys::#name_lower::#set_label_size(self._inner, sz as i32) }
            }

            fn label_type<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(fltk_sys::#name_lower::#label_type(self._inner)) }
            }

            fn set_label_type<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    fltk_sys::#name_lower::#set_label_type(self._inner, typ.to_int());
                }
            }

            fn get_box<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(fltk_sys::#name_lower::#get_box(self._inner)) }
            }

            fn set_box<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    fltk_sys::#name_lower::#set_box(self._inner, typ.to_int());
                }
            }
        }
    };
    gen.into()
}

fn impl_widget_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl WidgetType for #name {
            fn to_int(self) -> i32 {
                self as i32
            }

            fn from_i32(val: i32) -> #name {
                unsafe { mem::transmute(val) }
            }
        }
    };
    gen.into()
}

fn impl_group_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    let name_lower = Ident::new(name.to_string().to_lowercase().as_str(), name.span());
    let begin = Ident::new(format!("Fl_{}_{}", name_str, "begin").as_str(), name.span());
    let end = Ident::new(format!("Fl_{}_{}", name_str, "end").as_str(), name.span());

    let gen = quote! {
        impl GroupTrait for #name {
            fn begin(&self) {
                unsafe { fltk_sys::#name_lower::#begin(self._inner) }
            }

            fn end(&self) {
                unsafe { fltk_sys::#name_lower::#end(self._inner) }
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
