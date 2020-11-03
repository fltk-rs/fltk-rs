#![recursion_limit = "256"]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod browser;
mod button;
mod display;
mod group;
mod image;
mod input;
mod menu;
mod table;
mod utils;
mod valuator;
mod widget;
mod window;

use crate::browser::*;
use crate::button::*;
use crate::display::*;
use crate::group::*;
use crate::image::*;
use crate::input::*;
use crate::menu::*;
use crate::table::*;
use crate::valuator::*;
use crate::widget::*;
use crate::window::*;
use proc_macro::TokenStream;

#[proc_macro_derive(WidgetBase)]
pub fn base_widget_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_base_trait(&ast)
}

#[proc_macro_derive(WidgetExt)]
pub fn widget_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_trait(&ast)
}

#[proc_macro_derive(WidgetType)]
pub fn widget_type_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_widget_type(&ast)
}

#[proc_macro_derive(ButtonExt)]
pub fn button_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_button_trait(&ast)
}

#[proc_macro_derive(GroupExt)]
pub fn group_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_group_trait(&ast)
}

#[proc_macro_derive(WindowExt)]
pub fn window_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_window_trait(&ast)
}

#[proc_macro_derive(InputExt)]
pub fn input_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_input_trait(&ast)
}

#[proc_macro_derive(MenuExt)]
pub fn menu_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_menu_trait(&ast)
}

#[proc_macro_derive(ValuatorExt)]
pub fn valuator_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_valuator_trait(&ast)
}

#[proc_macro_derive(DisplayExt)]
pub fn display_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_display_trait(&ast)
}

#[proc_macro_derive(BrowserExt)]
pub fn browser_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_browser_trait(&ast)
}

#[proc_macro_derive(TableExt)]
pub fn table_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_table_trait(&ast)
}

#[proc_macro_derive(ImageExt)]
pub fn image_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_image_trait(&ast)
}
