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

fn get_fl_name(txt: String) -> String {
    if txt == "Frame" {
        return String::from("Fl_Box");
    }

    let mut fl_name = String::from("Fl");
    for c in txt.chars() {
        if c.is_uppercase() {
            fl_name.push('_');
            fl_name.push(c);
        } else {
            fl_name.push(c);
        }
    }
    fl_name
}

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

#[proc_macro_derive(WindowTrait)]
pub fn window_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_window_trait(&ast)
}

#[proc_macro_derive(InputTrait)]
pub fn input_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_input_trait(&ast)
}

fn impl_widget_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let set_label = Ident::new(format!("{}_{}", name_str, "set_label").as_str(), name.span());
    let redraw = Ident::new(format!("{}_{}", name_str, "redraw").as_str(), name.span());
    let show = Ident::new(format!("{}_{}", name_str, "show").as_str(), name.span());
    let hide = Ident::new(format!("{}_{}", name_str, "hide").as_str(), name.span());
    let activate = Ident::new(format!("{}_{}", name_str, "activate").as_str(), name.span());
    let deactivate = Ident::new(format!("{}_{}", name_str, "deactivate").as_str(), name.span());
    let redraw_label = Ident::new(format!("{}_{}", name_str, "redraw_label").as_str(), name.span());
    let resize = Ident::new(format!("{}_{}", name_str, "resize").as_str(), name.span());
    let tooltip = Ident::new(format!("{}_{}", name_str, "tooltip").as_str(), name.span());
    let set_tooltip = Ident::new(format!("{}_{}", name_str, "set_tooltip").as_str(), name.span());
    let get_type = Ident::new(format!("{}_{}", name_str, "get_type").as_str(), name.span());
    let set_type = Ident::new(format!("{}_{}", name_str, "set_type").as_str(), name.span());
    let color = Ident::new(format!("{}_{}", name_str, "color").as_str(), name.span());
    let set_color = Ident::new(format!("{}_{}", name_str, "set_color").as_str(), name.span());
    let label_color = Ident::new(format!("{}_{}", name_str, "label_color").as_str(), name.span());
    let set_label_color = Ident::new(format!("{}_{}", name_str, "set_label_color").as_str(), name.span());
    let label_font = Ident::new(format!("{}_{}", name_str, "label_font").as_str(), name.span());
    let set_label_font = Ident::new(format!("{}_{}", name_str, "set_label_font").as_str(), name.span());
    let label_size = Ident::new(format!("{}_{}", name_str, "label_size").as_str(), name.span());
    let set_label_size = Ident::new(format!("{}_{}", name_str, "set_label_size").as_str(), name.span());
    let label_type = Ident::new(format!("{}_{}", name_str, "label_type").as_str(), name.span());
    let set_label_type = Ident::new(format!("{}_{}", name_str, "set_label_type").as_str(), name.span());
    let frame = Ident::new(format!("{}_{}", name_str, "box").as_str(), name.span());
    let set_frame = Ident::new(format!("{}_{}", name_str, "set_box").as_str(), name.span());

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
                    #new(
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
                    #set_label(
                        self._inner,
                        self._title.as_ptr() as *const libc::c_char,
                    )
                }
            }

            fn redraw(&mut self) {
                unsafe {
                    #redraw(self._inner);
                }
            }

            fn show(&mut self) {
                unsafe { #show(self._inner) }
            }

            fn hide(&mut self) {
                unsafe { #hide(self._inner) }
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

            fn label(&self) -> String {
                self._title.clone().into_string().unwrap()
            }

            fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
                unsafe { mem::transmute(self._inner) }
            }

            fn activate(&mut self) {
                unsafe { #activate(self._inner) }
            }

            fn deactivate(&mut self) {
                unsafe { #deactivate(self._inner) }
            }

            fn redraw_label(&mut self) {
                unsafe { #redraw_label(self._inner) }
            }

            fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                unsafe { #resize(self._inner, x, y, width, height) }
            }

            fn tooltip(&self) -> String {
                unsafe {
                    ffi::CString::from_raw(
                        #tooltip(self._inner) as *mut libc::c_char
                    ).into_string().unwrap()
                }
            }

            fn set_tooltip(&mut self, txt: &str) {
                let txt = ffi::CString::new(txt).unwrap();
                unsafe {
                    #set_tooltip(
                        self._inner,
                        txt.as_ptr() as *const libc::c_char,
                    )
                }
            }

            fn get_type<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(#get_type(self._inner)) }
            }

            fn set_type<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    #set_type(self._inner, typ.to_int());
                }
            }

            fn color(&self) -> Color {
                unsafe { mem::transmute(#color(self._inner)) }
            }

            fn set_color(&mut self, color: Color) {
                unsafe { #set_color(self._inner, color as i32) }
            }

            fn label_color(&self) -> Color {
                unsafe { mem::transmute(#label_color(self._inner)) }
            }

            fn set_label_color(&mut self, color: Color) {
                unsafe { #set_label_color(self._inner, color as i32) }
            }

            fn label_font(&self) -> Font {
                unsafe { mem::transmute(#label_font(self._inner)) }
            }

            fn set_label_font(&mut self, font: Font) {
                unsafe { #set_label_color(self._inner, font as i32) }
            }

            fn label_size(&self) -> usize {
                unsafe { #label_size(self._inner) as usize }
            }

            fn set_label_size(&mut self, sz: usize) {
                unsafe { #set_label_size(self._inner, sz as i32) }
            }

            fn label_type<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(#label_type(self._inner)) }
            }

            fn set_label_type<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    #set_label_type(self._inner, typ.to_int());
                }
            }

            fn frame<T: WidgetType>(&self) -> T {
                unsafe { T::from_i32(#frame(self._inner)) }
            }

            fn set_frame<T: WidgetType>(&mut self, typ: T) {
                unsafe {
                    #set_frame(self._inner, typ.to_int());
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
    let name_str = get_fl_name(name.to_string());

    let begin = Ident::new(format!("{}_{}", name_str, "begin").as_str(), name.span());
    let end = Ident::new(format!("{}_{}", name_str, "end").as_str(), name.span());

    let gen = quote! {
        impl GroupTrait for #name {
            fn begin(&self) {
                unsafe { #begin(self._inner) }
            }

            fn end(&self) {
                unsafe { #end(self._inner) }
            }
        }
    };
    gen.into()
}

fn impl_window_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    
    let make_modal = Ident::new(format!("{}_{}", name_str, "make_modal").as_str(), name.span());
    let fullscreen = Ident::new(format!("{}_{}", name_str, "fullscreen").as_str(), name.span());
    let make_current = Ident::new(format!("{}_{}", name_str, "make_current").as_str(), name.span());

    let gen = quote! {
        impl WindowTrait for #name {
            fn make_modal(&mut self, val: bool) {
                unsafe { #make_modal(self._inner, val as u32) }
            }

            fn fullscreen(&mut self, val: bool) {
                unsafe { #fullscreen(self._inner, val as u32) }
            }

            fn make_current(&mut self) {
                unsafe { #make_current(self._inner) }
            }
        }
    };
    gen.into()
}

fn impl_input_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(format!("{}_{}", name_str, "set_value").as_str(), name.span());
    let maximum_size = Ident::new(format!("{}_{}", name_str, "maximum_size").as_str(), name.span());
    let set_maximum_size = Ident::new(format!("{}_{}", name_str, "set_maximum_size").as_str(), name.span());
    let position = Ident::new(format!("{}_{}", name_str, "position").as_str(), name.span());
    let set_position = Ident::new(format!("{}_{}", name_str, "set_position").as_str(), name.span());
    let mark = Ident::new(format!("{}_{}", name_str, "mark").as_str(), name.span());
    let set_mark = Ident::new(format!("{}_{}", name_str, "set_mark").as_str(), name.span());
    let replace = Ident::new(format!("{}_{}", name_str, "replace").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let append = Ident::new(format!("{}_{}", name_str, "append").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());
    let undo = Ident::new(format!("{}_{}", name_str, "undo").as_str(), name.span());
    let copy_cuts = Ident::new(format!("{}_{}", name_str, "copy_cuts").as_str(), name.span());
    let text_font = Ident::new(format!("{}_{}", name_str, "text_font").as_str(), name.span());
    let set_text_font = Ident::new(format!("{}_{}", name_str, "set_text_font").as_str(), name.span());
    let text_color = Ident::new(format!("{}_{}", name_str, "text_color").as_str(), name.span());
    let set_text_color = Ident::new(format!("{}_{}", name_str, "set_text_color").as_str(), name.span());
    let text_size = Ident::new(format!("{}_{}", name_str, "text_size").as_str(), name.span());
    let set_text_size = Ident::new(format!("{}_{}", name_str, "set_text_size").as_str(), name.span());
    let readonly = Ident::new(format!("{}_{}", name_str, "readonly").as_str(), name.span());
    let set_readonly = Ident::new(format!("{}_{}", name_str, "set_readonly").as_str(), name.span());
    let wrap = Ident::new(format!("{}_{}", name_str, "wrap").as_str(), name.span());
    let set_wrap = Ident::new(format!("{}_{}", name_str, "set_wrap").as_str(), name.span());


    let gen = quote! {
        impl InputTrait for #name {
            fn value(&self) -> String {
                unsafe {
                    ffi::CString::from_raw(#value(self._inner) as *mut libc::c_char).into_string().unwrap()
                }
            }
            fn set_value(&mut self, val: &str) {
                let val = ffi::CString::new(val).unwrap();
                unsafe {
                    #set_value(self._inner, val.as_ptr() as *const libc::c_char);
                }
            }
            fn maximum_size(&self) -> usize {
                unsafe {
                    #maximum_size(self._inner) as usize
                }
            }
            fn set_maximum_size(&mut self, val: usize) {
                unsafe {
                    #set_maximum_size(self._inner, val as i32)
                }
            }
            fn position(&self) -> i32 {
                unsafe {
                    #position(self._inner)
                }
            }
            fn set_position(&mut self, val: i32) {
                unsafe {
                    #set_position(self._inner, val);
                }
            }
            fn mark(&self) -> i32 {
                unsafe {
                    #mark(self._inner) as i32
                }
            }
            fn set_mark(&mut self, val: i32) {
                unsafe {
                    #set_mark(self._inner, val);
                }
            }
            fn replace(&mut self, beg: usize, end: usize, val: &str) {
                let val = ffi::CString::new(val).unwrap();
                unsafe {
                    #replace(self._inner, beg as i32, end as i32, val.as_ptr() as *const libc::c_char, 0);
                }
            }
            fn insert(&mut self, txt: &str) {
                let txt = ffi::CString::new(txt).unwrap();
                unsafe {
                    #insert(self._inner, txt.as_ptr() as *const libc::c_char, 0);
                }
            }
            fn append(&mut self, txt: &str) {
                let txt = ffi::CString::new(txt).unwrap();
                unsafe {
                    #append(self._inner,  txt.as_ptr() as *const libc::c_char, 0, 0);
                }
            }
            fn copy(&mut self) {
                unsafe {
                    #copy(self._inner, 1);
                }
            }
            fn undo(&mut self) {
                unsafe {
                    #undo(self._inner);
                }
            }
            fn cut(&mut self) {
                unsafe {
                    #copy_cuts(self._inner);
                }
            }
            fn text_font(&self) -> Font {
                unsafe {
                    mem::transmute(#text_font(self._inner))
                }
            }
            fn set_text_font(&mut self, font: Font) {
                unsafe {
                    #set_text_font(self._inner, font as i32)
                }
            }
            fn text_color(&self) -> Color {
                unsafe {
                    mem::transmute(#text_color(self._inner))
                }
            }
            fn set_text_color(&mut self, color: Color) {
                unsafe {
                    #set_text_color(self._inner, color as i32)
                }
            }
            fn text_size(&self) -> usize {
                unsafe {
                    #text_size(self._inner) as usize
                }
            }
            fn set_text_size(&mut self, sz: usize) {
                unsafe {
                    #set_text_size(self._inner, sz as i32)
                }
            }
            fn readonly(&self) -> bool {
                unsafe {
                    match #readonly(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }
            fn set_readonly(&mut self, val: bool) {
                unsafe {
                    #set_readonly(self._inner, val as i32)
                }
            }
            fn wrap(&self) -> bool {
                unsafe {
                    match #wrap(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }
            fn set_wrap(&mut self, val: bool) {
                unsafe {
                    #set_wrap(self._inner, val as i32)
                }
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
