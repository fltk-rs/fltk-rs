use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_input_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let maximum_size = Ident::new(
        format!("{}_{}", name_str, "maximum_size").as_str(),
        name.span(),
    );
    let set_maximum_size = Ident::new(
        format!("{}_{}", name_str, "set_maximum_size").as_str(),
        name.span(),
    );
    let position = Ident::new(format!("{}_{}", name_str, "position").as_str(), name.span());
    let set_position = Ident::new(
        format!("{}_{}", name_str, "set_position").as_str(),
        name.span(),
    );
    let mark = Ident::new(format!("{}_{}", name_str, "mark").as_str(), name.span());
    let set_mark = Ident::new(format!("{}_{}", name_str, "set_mark").as_str(), name.span());
    let replace = Ident::new(format!("{}_{}", name_str, "replace").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let append = Ident::new(format!("{}_{}", name_str, "append").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());
    let undo = Ident::new(format!("{}_{}", name_str, "undo").as_str(), name.span());
    let copy_cuts = Ident::new(
        format!("{}_{}", name_str, "copy_cuts").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
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
    let readonly = Ident::new(format!("{}_{}", name_str, "readonly").as_str(), name.span());
    let set_readonly = Ident::new(
        format!("{}_{}", name_str, "set_readonly").as_str(),
        name.span(),
    );
    let wrap = Ident::new(format!("{}_{}", name_str, "wrap").as_str(), name.span());
    let set_wrap = Ident::new(format!("{}_{}", name_str, "set_wrap").as_str(), name.span());

    let gen = quote! {
        unsafe impl InputExt for #name {
            fn value(&self) -> String {
                unsafe {
                    assert!(!self.was_deleted());
                    let value_ptr = #value(self._inner);
                    assert!(!value_ptr.is_null());
                    CStr::from_ptr(value_ptr as *mut raw::c_char).to_string_lossy().to_string()
                }
            }

            fn set_value(&self, val: &str) {
                let temp = CString::new(val).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    #set_value(self._inner, temp.as_ptr());
                }
            }

            fn maximum_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #maximum_size(self._inner) as u32
                }
            }

            fn set_maximum_size(&mut self, val: u32) {
                unsafe {
                    debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    #set_maximum_size(self._inner, val as i32)
                }
            }

            fn position(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #position(self._inner)
                }
            }

            fn set_position(&mut self, val: i32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #set_position(self._inner, val);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn mark(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #mark(self._inner)
                }
            }

            fn set_mark(&mut self, val: i32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #set_mark(self._inner, val);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn replace(&mut self, beg: u32, end: u32, val: &str) -> Result<(), FltkError> {
                let val = CString::new(val).unwrap();
                unsafe {
                    debug_assert!(beg <= std::i32::MAX as u32 && end <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    let x = #replace(self._inner, beg as i32, end as i32, val.as_ptr(), 0);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn insert(&mut self, txt: &str) -> Result<(), FltkError> {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #insert(self._inner, txt.as_ptr(), 0);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn append(&mut self, txt: &str) -> Result<(), FltkError> {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #append(self._inner,  txt.as_ptr(), 0, 0);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn copy(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #copy(self._inner, 1);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn undo(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #undo(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn cut(&mut self) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #copy_cuts(self._inner);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#text_font(self._inner))
                }
            }

            fn set_text_font(&mut self, font: Font) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_font(self._inner, font as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#text_color(self._inner))
                }
            }

            fn set_text_color(&mut self, color: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_text_color(self._inner, color as u32)
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, sz: u32) {
                unsafe {
                    debug_assert!(sz <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(!self.was_deleted());
                    #set_text_size(self._inner, sz as i32)
                }
            }

            fn readonly(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #readonly(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_readonly(&mut self, val: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_readonly(self._inner, val as i32)
                }
            }

            fn wrap(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #wrap(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_wrap(&mut self, val: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_wrap(self._inner, val as i32)
                }
            }
        }
    };
    gen.into()
}
