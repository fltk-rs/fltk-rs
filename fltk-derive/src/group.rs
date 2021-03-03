use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_group_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let begin = Ident::new(format!("{}_{}", name_str, "begin").as_str(), name.span());
    let end = Ident::new(format!("{}_{}", name_str, "end").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let children = Ident::new(format!("{}_{}", name_str, "children").as_str(), name.span());
    let child = Ident::new(format!("{}_{}", name_str, "child").as_str(), name.span());
    let find = Ident::new(format!("{}_{}", name_str, "find").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let remove_by_index = Ident::new(format!("{}_{}", name_str, "remove_by_index").as_str(), name.span());
    let resizable = Ident::new(
        format!("{}_{}", name_str, "resizable").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl IntoIterator for #name {
            type Item = Box<dyn WidgetExt>;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                let mut v: Vec<Box<dyn WidgetExt>> = vec![];
                for i in 0..self.children() {
                    v.push(self.child(i).unwrap());
                }
                v.into_iter()
            }
        }

        unsafe impl GroupExt for #name {
            fn begin(&self) {
                assert!(!self.was_deleted());
                unsafe { #begin(self._inner) }
            }

            fn end(&self) {
                assert!(!self.was_deleted());
                unsafe { #end(self._inner) }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self._inner);
                }
            }

            fn children(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #children(self._inner) as u32
                }
            }

            fn child(&self, idx: u32) -> Option<Box<dyn WidgetExt>> {
                unsafe {
                    debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    let child_widget = #child(self._inner, idx as i32);
                    if child_widget.is_null() {
                        None
                    } else {
                        Some(Box::new(Widget::from_widget_ptr(child_widget as *mut fltk_sys::widget::Fl_Widget)))
                    }
                }
            }

            fn find<W: WidgetExt>(&self, widget: &W) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #find(self._inner, widget.as_widget_ptr() as *mut _) as u32
                }
            }

            fn add<W: WidgetExt>(&mut self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #add(self._inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn insert<W: WidgetExt>(&mut self, widget: &W, index: u32) {
                unsafe {
                    debug_assert!(index <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #insert(self._inner, widget.as_widget_ptr() as *mut _, index as i32)
                }
            }

            fn remove<W: WidgetExt>(&mut self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #remove(self._inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn remove_by_index(&mut self, idx: u32) {
                debug_assert!(idx <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(idx < self.children());
                    #remove(self._inner, idx as i32);
                }
            }

            fn resizable<W: WidgetExt>(&self, widget: &W) {
                unsafe {
                    assert!(!self.was_deleted());
                    assert!(!widget.was_deleted());
                    #resizable(self._inner, widget.as_widget_ptr() as *mut _)
                }
            }

            fn make_resizable(&mut self, val: bool) {
                assert!(!self.was_deleted());
                let ptr = if val { self._inner } else { std::ptr::null_mut() };
                unsafe {
                    #resizable(self._inner, ptr as *mut _)
                }
            }
        }
    };
    gen.into()
}
