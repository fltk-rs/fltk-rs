use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_browser_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let move_item = Ident::new(format!("{}_{}", name_str, "move").as_str(), name.span());
    let swap = Ident::new(format!("{}_{}", name_str, "swap").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let size = Ident::new(format!("{}_{}", name_str, "size").as_str(), name.span());
    let set_size = Ident::new(format!("{}_{}", name_str, "set_size").as_str(), name.span());
    let select = Ident::new(format!("{}_{}", name_str, "select").as_str(), name.span());
    let selected = Ident::new(format!("{}_{}", name_str, "selected").as_str(), name.span());
    let text = Ident::new(format!("{}_{}", name_str, "text").as_str(), name.span());
    let set_text = Ident::new(format!("{}_{}", name_str, "set_text").as_str(), name.span());
    let load_file = Ident::new(
        format!("{}_{}", name_str, "load_file").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let set_icon = Ident::new(format!("{}_{}", name_str, "set_icon").as_str(), name.span());
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let remove_icon = Ident::new(
        format!("{}_{}", name_str, "remove_icon").as_str(),
        name.span(),
    );
    let topline = Ident::new(format!("{}_{}", name_str, "topline").as_str(), name.span());
    let middleline = Ident::new(
        format!("{}_{}", name_str, "middleline").as_str(),
        name.span(),
    );
    let bottomline = Ident::new(
        format!("{}_{}", name_str, "bottomline").as_str(),
        name.span(),
    );
    let format_char = Ident::new(
        format!("{}_{}", name_str, "format_char").as_str(),
        name.span(),
    );
    let set_format_char = Ident::new(
        format!("{}_{}", name_str, "set_format_char").as_str(),
        name.span(),
    );
    let column_char = Ident::new(
        format!("{}_{}", name_str, "column_char").as_str(),
        name.span(),
    );
    let set_column_char = Ident::new(
        format!("{}_{}", name_str, "set_column_char").as_str(),
        name.span(),
    );
    let column_widths = Ident::new(
        format!("{}_{}", name_str, "column_widths").as_str(),
        name.span(),
    );
    let set_column_widths = Ident::new(
        format!("{}_{}", name_str, "set_column_widths").as_str(),
        name.span(),
    );
    let displayed = Ident::new(
        format!("{}_{}", name_str, "displayed").as_str(),
        name.span(),
    );
    let make_visible = Ident::new(
        format!("{}_{}", name_str, "make_visible").as_str(),
        name.span(),
    );
    let position = Ident::new(format!("{}_{}", name_str, "position").as_str(), name.span());
    let set_position = Ident::new(
        format!("{}_{}", name_str, "set_position").as_str(),
        name.span(),
    );
    let hposition = Ident::new(
        format!("{}_{}", name_str, "hposition").as_str(),
        name.span(),
    );
    let set_hposition = Ident::new(
        format!("{}_{}", name_str, "set_hposition").as_str(),
        name.span(),
    );
    let has_scrollbar = Ident::new(
        format!("{}_{}", name_str, "has_scrollbar").as_str(),
        name.span(),
    );
    let set_has_scrollbar = Ident::new(
        format!("{}_{}", name_str, "set_has_scrollbar").as_str(),
        name.span(),
    );
    let scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "scrollbar_size").as_str(),
        name.span(),
    );
    let set_scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_size").as_str(),
        name.span(),
    );
    let sort = Ident::new(format!("{}_{}", name_str, "sort").as_str(), name.span());
    let scrollbar = Ident::new(
        format!("{}_{}", name_str, "scrollbar").as_str(),
        name.span(),
    );
    let hscrollbar = Ident::new(
        format!("{}_{}", name_str, "hscrollbar").as_str(),
        name.span(),
    );
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());

    let gen = quote! {
        unsafe impl BrowserExt for #name {
            fn remove(&mut self, line: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #remove(self.inner, line as i32)
                }
            }

            fn add(&mut self, item: &str) {
                assert!(!self.was_deleted());
                let item = CString::safe_new(item);
                unsafe {
                    #add(self.inner, item.as_ptr())
                }
            }

            fn insert(&mut self, line: i32, item: &str) {
                assert!(!self.was_deleted());
                let item = CString::safe_new(item);
                unsafe {
                    #insert(self.inner, line as i32, item.as_ptr())
                }
            }

            fn move_item(&mut self, to: i32, from: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #move_item(self.inner, to as i32, from as i32)
                }
            }

            fn swap(&mut self, a: i32, b: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #swap(self.inner, a as i32, b as i32)
                }
            }

            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self.inner)
                }
            }

            fn size(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #size(self.inner) as i32
                }
            }

            fn select(&mut self, line: i32) {
                assert!(!self.was_deleted());
                if line <= self.size() {
                    unsafe {
                        #select(self.inner, line as i32);
                    }
                }
            }

            fn selected(&self, line: i32) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #selected(self.inner, line as i32)  != 0
                }
            }

            fn text(&self, line: i32) -> Option<String> {
                assert!(!self.was_deleted());
                unsafe {
                    let text = #text(self.inner, line as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(text as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn selected_text(&self) -> Option<String> {
                self.text(self.value())
            }

            fn set_text(&mut self, line: i32, txt: &str) {
                assert!(!self.was_deleted());
                let txt = CString::safe_new(txt);
                unsafe {
                    #set_text(self.inner, line as i32, txt.as_ptr())
                }
            }

            fn load<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), FltkError> {
                assert!(!self.was_deleted());
                if !path.as_ref().exists() {
                    return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
                }
                let path = path.as_ref().to_str().ok_or(FltkError::Unknown(String::from("Failed to convert path to string")))?;
                let path = CString::new(path)?;
                unsafe {
                    #load_file(self.inner, path.as_ptr());
                    Ok(())
                }
            }

            fn text_size(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #text_size(self.inner) as i32
                }
            }

            fn set_text_size(&mut self, c: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_size(self.inner, c as i32)
                }
            }

            fn set_icon<Img: ImageExt>(&mut self, line: i32, image: Option<Img>) {
                assert!(!self.was_deleted());
                if let Some(image) = image {
                    unsafe { #set_icon(self.inner, line as i32, image.as_image_ptr() as *mut _) }
                } else {
                    unsafe { #set_icon(self.inner, line as i32, std::ptr::null_mut() as *mut raw::c_void) }
                }
            }

            fn icon(&self, line: i32) -> Option<Box<dyn ImageExt>> {
                unsafe {
                    assert!(!self.was_deleted());
                    let image_ptr = #icon(self.inner, line as i32);
                    if image_ptr.is_null() {
                        None
                    } else {
                        let mut img = Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                        Some(Box::new(img))
                    }
                }
            }

            fn remove_icon(&mut self, line: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #remove_icon(self.inner, line as i32)
                }
            }

            fn top_line(&mut self, line: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #topline(self.inner, line as i32)
                }
            }

            fn bottom_line(&mut self, line: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #bottomline(self.inner, line as i32)
                }
            }

            fn middle_line(&mut self, line: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #middleline(self.inner, line as i32)
                }
            }

            fn format_char(&self) -> char {
                assert!(!self.was_deleted());
                unsafe {
                    #format_char(self.inner) as u8 as char
                }
            }

            fn set_format_char(&mut self, c: char) {
                assert!(!self.was_deleted());
                debug_assert!(c != 0 as char);
                let c = if c as i32 > 128 { 128 as char } else { c };
                unsafe {
                    #set_format_char(self.inner, c as raw::c_char)
                }
            }

            fn column_char(&self) -> char {
                assert!(!self.was_deleted());
                unsafe {
                    #column_char(self.inner) as u8 as char
                }
            }

            fn set_column_char(&mut self, c: char) {
                assert!(!self.was_deleted());
                debug_assert!(c != 0 as char);
                let c = if c as i32 > 128 { 128 as char } else { c };
                unsafe {
                    #set_column_char(self.inner, c as raw::c_char)
                }
            }

            fn column_widths(&self) -> Vec<i32> {
                assert!(!self.was_deleted());
                unsafe {
                    let widths = #column_widths(self.inner);
                    // Should never throw
                    assert!(!widths.is_null());
                    let mut v: Vec<i32> = vec![];
                    let mut i = 0;
                    while (*widths.offset(i) != 0) {
                        v.push(*widths.offset(i));
                        i += 1;
                    }
                    v
                }
            }

            fn set_column_widths(&mut self, arr: &'static [i32]) {
                assert!(!self.was_deleted());
                unsafe {
                    let mut v = arr.to_vec();
                    v.push(0);
                    let v = mem::ManuallyDrop::new(v);
                    #set_column_widths(self.inner, v.as_ptr());
                }
            }

            fn displayed(&self, line: i32,) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #displayed(self.inner, line as i32,) != 0
                }
            }

            fn make_visible(&mut self, line: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #make_visible(self.inner, line as i32)
                }
            }

            fn position(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #position(self.inner) as i32
                }
            }

            fn set_position(&mut self, pos: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_position(self.inner, pos as i32)
                }
            }

            fn hposition(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #hposition(self.inner) as i32
                }
            }

            fn set_hposition(&mut self, pos: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_hposition(self.inner, pos as i32)
                }
            }

            fn has_scrollbar(&self) -> BrowserScrollbar {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#has_scrollbar(self.inner))
                }
            }

            fn set_has_scrollbar(&mut self, mode: BrowserScrollbar) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_has_scrollbar(self.inner, mode as raw::c_uchar)
                }
            }

            fn scrollbar_size(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #scrollbar_size(self.inner) as i32
                }
            }

            fn set_scrollbar_size(&mut self, new_size: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_scrollbar_size(self.inner, new_size as i32)
                }
            }

            fn sort(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #sort(self.inner)
                }
            }

            fn scrollbar(&self) -> Box<dyn ValuatorExt> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #scrollbar(self.inner);
                    assert!(!ptr.is_null());
                    Box::new(crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget))
                }
            }

            fn hscrollbar(&self) -> Box<dyn ValuatorExt> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #hscrollbar(self.inner);
                    assert!(!ptr.is_null());
                    Box::new(crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget))
                }
            }

            fn value(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #value(self.inner) as i32 }
            }
        }
    };
    gen.into()
}
