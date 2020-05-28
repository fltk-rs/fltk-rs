use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_group_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let begin = Ident::new(format!("{}_{}", name_str, "begin").as_str(), name.span());
    let end = Ident::new(format!("{}_{}", name_str, "end").as_str(), name.span());
    let find = Ident::new(format!("{}_{}", name_str, "find").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let children = Ident::new(format!("{}_{}", name_str, "children").as_str(), name.span());
    let child = Ident::new(format!("{}_{}", name_str, "child").as_str(), name.span());
    let resizable = Ident::new(
        format!("{}_{}", name_str, "resizable").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl GroupExt for #name {
            fn begin(&self) {
                unsafe { #begin(self._inner) }
            }

            fn end(&self) {
                unsafe { #end(self._inner) }
            }

            fn find<Widget: WidgetExt>(&self, widget: &Widget) -> u32 {
                unsafe {
                    #find(self._inner, widget.as_widget_ptr() as *mut raw::c_void) as u32
                }
            }

            fn add<Widget: WidgetExt>(&mut self, widget: &Widget) {
                unsafe {
                    #add(self._inner, widget.as_widget_ptr() as *mut raw::c_void)
                }
            }

            fn insert<Widget: WidgetExt>(&mut self, widget: &Widget, index: u32) {
                unsafe {
                    debug_assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #insert(self._inner, widget.as_widget_ptr() as *mut raw::c_void, index as i32)
                }
            }

            fn remove<Widget: WidgetExt>(&mut self, widget: &Widget) {
                unsafe {
                    #remove(self._inner, widget.as_widget_ptr() as *mut raw::c_void)
                }
            }

            unsafe fn clear(&mut self) {
                unsafe {
                    #clear(self._inner)
                }
            }

            fn children(&self) -> u32 {
                unsafe {
                    #children(self._inner) as u32
                }
            }

            fn child(&self, idx: u32) -> Option<Widget> {
                unsafe {
                    debug_assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    let child_widget = #child(self._inner, idx as i32);
                    if child_widget.is_null() {
                        None
                    } else {
                        Some(Widget::from_raw(child_widget as *mut fltk_sys::widget::Fl_Widget))
                    }
                }
            }

            fn resizable<Widget: WidgetExt>(&self, widget: &mut Widget) {
                unsafe {
                    #resizable(self._inner, widget.as_widget_ptr() as *mut raw::c_void)
                }
            }
        }
    };
    gen.into()
}
