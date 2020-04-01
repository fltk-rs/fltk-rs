#![recursion_limit = "256"]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::*;
use syn::*;

fn get_fl_name(txt: String) -> String {
    if txt == "Frame" {
        return String::from("Fl_Box");
    }
    if txt == "JpegImage" {
        return String::from("Fl_JPEG_Image");
    }
    if txt == "PngImage" {
        return String::from("Fl_PNG_Image");
    }
    if txt == "BmpImage" {
        return String::from("Fl_BMP_Image");
    }
    if txt == "SvgImage" {
        return String::from("Fl_SVG_Image");
    }
    if txt == "GifImage" {
        return String::from("Fl_GIF_Image");
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

#[proc_macro_derive(MenuTrait)]
pub fn menu_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_menu_trait(&ast)
}

#[proc_macro_derive(ValuatorTrait)]
pub fn valuator_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_valuator_trait(&ast)
}

#[proc_macro_derive(DisplayTrait)]
pub fn display_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_display_trait(&ast)
}

#[proc_macro_derive(BrowserTrait)]
pub fn browser_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_browser_trait(&ast)
}

#[proc_macro_derive(ImageTrait)]
pub fn image_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_image_trait(&ast)
}

fn impl_widget_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let x = Ident::new(format!("{}_{}", name_str, "x").as_str(), name.span());
    let y = Ident::new(format!("{}_{}", name_str, "y").as_str(), name.span());
    let width = Ident::new(format!("{}_{}", name_str, "width").as_str(), name.span());
    let height = Ident::new(format!("{}_{}", name_str, "height").as_str(), name.span());
    let label = Ident::new(format!("{}_{}", name_str, "label").as_str(), name.span());
    let set_label = Ident::new(
        format!("{}_{}", name_str, "set_label").as_str(),
        name.span(),
    );
    let redraw = Ident::new(format!("{}_{}", name_str, "redraw").as_str(), name.span());
    let show = Ident::new(format!("{}_{}", name_str, "show").as_str(), name.span());
    let hide = Ident::new(format!("{}_{}", name_str, "hide").as_str(), name.span());
    let activate = Ident::new(format!("{}_{}", name_str, "activate").as_str(), name.span());
    let deactivate = Ident::new(
        format!("{}_{}", name_str, "deactivate").as_str(),
        name.span(),
    );
    let redraw_label = Ident::new(
        format!("{}_{}", name_str, "redraw_label").as_str(),
        name.span(),
    );
    let resize = Ident::new(format!("{}_{}", name_str, "resize").as_str(), name.span());
    let tooltip = Ident::new(format!("{}_{}", name_str, "tooltip").as_str(), name.span());
    let set_tooltip = Ident::new(
        format!("{}_{}", name_str, "set_tooltip").as_str(),
        name.span(),
    );
    let get_type = Ident::new(format!("{}_{}", name_str, "get_type").as_str(), name.span());
    let set_type = Ident::new(format!("{}_{}", name_str, "set_type").as_str(), name.span());
    let color = Ident::new(format!("{}_{}", name_str, "color").as_str(), name.span());
    let set_color = Ident::new(
        format!("{}_{}", name_str, "set_color").as_str(),
        name.span(),
    );
    let label_color = Ident::new(
        format!("{}_{}", name_str, "label_color").as_str(),
        name.span(),
    );
    let set_label_color = Ident::new(
        format!("{}_{}", name_str, "set_label_color").as_str(),
        name.span(),
    );
    let label_font = Ident::new(
        format!("{}_{}", name_str, "label_font").as_str(),
        name.span(),
    );
    let set_label_font = Ident::new(
        format!("{}_{}", name_str, "set_label_font").as_str(),
        name.span(),
    );
    let label_size = Ident::new(
        format!("{}_{}", name_str, "label_size").as_str(),
        name.span(),
    );
    let set_label_size = Ident::new(
        format!("{}_{}", name_str, "set_label_size").as_str(),
        name.span(),
    );
    let label_type = Ident::new(
        format!("{}_{}", name_str, "label_type").as_str(),
        name.span(),
    );
    let set_label_type = Ident::new(
        format!("{}_{}", name_str, "set_label_type").as_str(),
        name.span(),
    );
    let frame = Ident::new(format!("{}_{}", name_str, "box").as_str(), name.span());
    let set_frame = Ident::new(format!("{}_{}", name_str, "set_box").as_str(), name.span());
    let changed = Ident::new(format!("{}_{}", name_str, "changed").as_str(), name.span());
    let set_changed = Ident::new(
        format!("{}_{}", name_str, "set_changed").as_str(),
        name.span(),
    );
    let clear_changed = Ident::new(
        format!("{}_{}", name_str, "clear_changed").as_str(),
        name.span(),
    );
    let align = Ident::new(format!("{}_{}", name_str, "align").as_str(), name.span());
    let set_align = Ident::new(
        format!("{}_{}", name_str, "set_align").as_str(),
        name.span(),
    );
    let set_image = Ident::new(
        format!("{}_{}", name_str, "set_image").as_str(),
        name.span(),
    );
    let set_handler = Ident::new(
        format!("{}_{}", name_str, "set_handler").as_str(),
        name.span(),
    );
    let set_trigger = Ident::new(
        format!("{}_{}", name_str, "set_trigger").as_str(),
        name.span(),
    );
    let gen = quote! {
        unsafe impl Send for #name {}
        impl Copy for #name {}
        impl WidgetTrait for #name {
            fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> #name {
                let temp = CString::new(title).unwrap();
                unsafe {
                    #name {
                        _inner: #new(
                            x,
                            y,
                            width,
                            height,
                            temp.into_raw() as *const raw::c_char,
                        ),
                    }

                }
            }
            fn default() -> Self {
                let temp = CString::new("").unwrap();
                unsafe {
                    #name {
                        _inner: #new(
                            0,
                            0,
                            0,
                            0,
                            temp.into_raw() as *const raw::c_char,
                        ),
                    }
                }
            }
            fn with_pos(mut self, x: i32, y: i32) -> Self {
                self.resize(x, y, self.width(), self.height());
                self
            }
            fn with_size(mut self, width: i32, height: i32) -> Self {
                self.resize(self.x(), self.y(), width, height);
                self
            }
            fn with_label(mut self, title: &str) -> Self {
                self.set_label(title);
                self
            }
            fn set_label(&mut self, title: &str) {
                let temp = CString::new(title).unwrap();
                unsafe {
                    #set_label(
                        self._inner,
                        temp.into_raw() as *const raw::c_char,
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
                unsafe { #x(self._inner)}
            }

            fn y(&self) -> i32 {
                unsafe { #y(self._inner) }
            }

            fn width(&self) -> i32 {
                unsafe { #width(self._inner) }
            }

            fn height(&self) -> i32 {
                unsafe { #height(self._inner) }
            }

            fn label(&self) -> String {
                unsafe {
                    CStr::from_ptr(#label(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
                }
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
                    CStr::from_ptr(
                        #tooltip(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
                }
            }

            fn set_tooltip(&mut self, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #set_tooltip(
                        self._inner,
                        txt.into_raw() as *mut raw::c_char,
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
                unsafe { #set_color(self._inner, color as u32) }
            }

            fn label_color(&self) -> Color {
                unsafe { mem::transmute(#label_color(self._inner)) }
            }

            fn set_label_color(&mut self, color: Color) {
                unsafe { #set_label_color(self._inner, color as u32) }
            }

            fn label_font(&self) -> Font {
                unsafe { mem::transmute(#label_font(self._inner)) }
            }

            fn set_label_font(&mut self, font: Font) {
                unsafe { #set_label_font(self._inner, font as i32) }
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

            fn changed(&self) -> bool {
                unsafe {
                    match #changed(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_changed(&mut self) {
                unsafe { #set_changed(self._inner) }
            }

            fn clear_changed(&mut self) {
                unsafe {#clear_changed(self._inner) }
            }

            fn align(&self) -> Align {
                unsafe { mem::transmute(#align(self._inner)) }
            }

            fn set_align(&mut self, align: Align) {
                unsafe { #set_align(self._inner, align as i32) }
            }

            fn set_image<Image: ImageTrait>(&mut self, image: Image) {
                unsafe { #set_image(self._inner, image.as_ptr()) }
            }

            fn set_callback<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>) {
                unsafe {
                    unsafe extern "C" fn shim<'a>(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
                        let f: &mut (dyn FnMut() + 'a) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: fltk_sys::widget::Fl_Callback = Some(shim);
                    fltk_sys::widget::Fl_Widget_callback_with_captures(self.as_widget_ptr(), callback, data);
                }
            }
            fn set_custom_handler<'a>(&'a mut self, cb: Box<dyn FnMut(Event) -> bool + 'a>) {
                unsafe {
                    unsafe extern "C" fn shim<'a>(_ev: std::os::raw::c_int, data: *mut raw::c_void) -> i32 {
                        let ev: Event = mem::transmute(_ev);
                        let a: *mut Box<dyn FnMut(Event) -> bool + 'a> = mem::transmute(data);
                        let f: &mut (dyn FnMut(Event) -> bool + 'a) = &mut **a;
                        match f(ev) {
                            true => return 1,
                            false => return 0,
                        }
                    }
                    let a: *mut Box<dyn FnMut(Event) -> bool + 'a> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: custom_handler_callback = Some(shim);
                    #set_handler(&mut self._inner, callback, data);
                }
            }
            fn set_trigger(&mut self, trigger: CallbackTrigger) {
                unsafe {
                    #set_trigger(self._inner, trigger as i32)
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
                unsafe { std::mem::transmute(val) }
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
    let find = Ident::new(format!("{}_{}", name_str, "find").as_str(), name.span());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let remove = Ident::new(format!("{}_{}", name_str, "remove").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let children = Ident::new(format!("{}_{}", name_str, "children").as_str(), name.span());

    let gen = quote! {
        impl GroupTrait for #name {
            fn begin(&self) {
                unsafe { #begin(self._inner) }
            }

            fn end(&self) {
                unsafe { #end(self._inner) }
            }
            fn find<Widget: WidgetTrait>(&self, widget: &Widget) -> usize {
                unsafe {
                    #find(self._inner, widget.as_widget_ptr() as *mut raw::c_void) as usize
                }
            }
            fn add<Widget: WidgetTrait>(&mut self, widget: &Widget) {
                unsafe {
                    #add(self._inner, widget.as_widget_ptr() as *mut raw::c_void)
                }
            }
            fn insert<Widget: WidgetTrait>(&mut self, widget: &Widget, index: usize) {
                unsafe {
                    #insert(self._inner, widget.as_widget_ptr() as *mut raw::c_void, index as i32)
                }
            }
            fn remove(&mut self, index: usize) {
                unsafe {
                    #remove(self._inner, index as i32)
                }
            }
            fn clear(&mut self) {
                unsafe {
                    #clear(self._inner)
                }
            }
            fn children(&self) -> usize {
                unsafe {
                    #children(self._inner) as usize
                }
            }
        }
    };
    gen.into()
}

fn impl_window_trait(ast: &syn::DeriveInput) -> TokenStream {
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
    let make_resizable = Ident::new(
        format!("{}_{}", name_str, "make_resizable").as_str(),
        name.span(),
    );
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
            fn set_icon<Image: ImageTrait>(&mut self, image: Image) {
                unsafe { #set_icon(self._inner, image.as_ptr()) }
            }
            fn make_resizable(&self, val: bool) {
                if val {
                    unsafe {
                        #make_resizable(self._inner, self._inner as *mut raw::c_void)
                    }
                }
            }
        }
    };
    gen.into()
}

fn impl_input_trait(ast: &syn::DeriveInput) -> TokenStream {
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
        impl InputTrait for #name {
            fn value(&self) -> String {
                unsafe {
                    CStr::from_ptr(#value(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
                }
            }
            fn set_value(&self, val: &str) {
                let temp = CString::new(val).unwrap();
                unsafe {
                    #set_value(self._inner, temp.into_raw() as *const raw::c_char);
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
                let val = CString::new(val).unwrap();
                unsafe {
                    #replace(self._inner, beg as i32, end as i32, val.into_raw() as *const raw::c_char, 0);
                }
            }
            fn insert(&mut self, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #insert(self._inner, txt.into_raw() as *const raw::c_char, 0);
                }
            }
            fn append(&mut self, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #append(self._inner,  txt.into_raw() as *const raw::c_char, 0, 0);
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
                    #set_text_color(self._inner, color as u32)
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

fn impl_menu_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let add = Ident::new(format!("{}_{}", name_str, "add").as_str(), name.span());
    let get_item = Ident::new(format!("{}_{}", name_str, "get_item").as_str(), name.span());
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
    let add_choice = Ident::new(
        format!("{}_{}", name_str, "add_choice").as_str(),
        name.span(),
    );
    let get_choice = Ident::new(
        format!("{}_{}", name_str, "get_choice").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl MenuTrait for #name {
            fn add<'a>(&'a mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, cb: Box<dyn FnMut() + 'a>) {
                let temp = CString::new(name).unwrap();
                unsafe {
                    unsafe extern "C" fn shim<'a>(_wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
                        let f: &mut (dyn FnMut() + 'a) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: Fl_Callback = Some(shim);
                    #add(self._inner, temp.into_raw() as *const raw::c_char, shortcut as i32, callback, data, flag as i32);
                }
            }

            fn get_item(&self, name: &str) -> MenuItem {
                let name = CString::new(name).unwrap().clone();
                MenuItem {
                    _title: name.clone(),
                    _inner: unsafe {
                        #get_item(
                            self._inner,
                            name.into_raw() as *const raw::c_char,
                        )
                    },
                }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    mem::transmute(#text_font(self._inner))
                }
            }

            fn set_text_font(&mut self, c: Font) {
                unsafe {
                    #set_text_font(self._inner, c as i32)
                }
            }

            fn text_size(&self) -> usize {
                unsafe {
                    #text_size(self._inner) as usize
                }
            }

            fn set_text_size(&mut self, c: usize) {
                unsafe {
                    #set_text_size(self._inner, c as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    mem::transmute(#text_color(self._inner))
                }
            }

            fn set_text_color(&mut self, c: Color) {
                unsafe {
                    #set_text_color(self._inner, c as u32)
                }
            }
            fn add_choice(&mut self, text: &str) {
                unsafe {
                    let arg2 = CString::new(text).unwrap();
                    #add_choice(self._inner, arg2.into_raw() as *mut raw::c_char)
                }
            }
            fn get_choice(&self) -> String {
                unsafe {
                    CStr::from_ptr(#get_choice(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
                }
            }
        }
    };
    gen.into()
}

fn impl_valuator_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let set_bounds = Ident::new(
        format!("{}_{}", name_str, "set_bounds").as_str(),
        name.span(),
    );
    let minimum = Ident::new(format!("{}_{}", name_str, "minimum").as_str(), name.span());
    let set_minimum = Ident::new(
        format!("{}_{}", name_str, "set_minimum").as_str(),
        name.span(),
    );
    let maximum = Ident::new(format!("{}_{}", name_str, "maximum").as_str(), name.span());
    let set_maximum = Ident::new(
        format!("{}_{}", name_str, "set_maximum").as_str(),
        name.span(),
    );
    let set_range = Ident::new(
        format!("{}_{}", name_str, "set_range").as_str(),
        name.span(),
    );
    let step = Ident::new(format!("{}_{}", name_str, "step").as_str(), name.span());
    let set_step = Ident::new(format!("{}_{}", name_str, "set_step").as_str(), name.span());
    let set_precision = Ident::new(
        format!("{}_{}", name_str, "set_precision").as_str(),
        name.span(),
    );
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let format = Ident::new(format!("{}_{}", name_str, "format").as_str(), name.span());
    let round = Ident::new(format!("{}_{}", name_str, "round").as_str(), name.span());
    let clamp = Ident::new(format!("{}_{}", name_str, "clamp").as_str(), name.span());
    let increment = Ident::new(
        format!("{}_{}", name_str, "increment").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl ValuatorTrait for #name {
            fn set_bounds(&mut self, a: f64, b: f64) {
                unsafe {
                    #set_bounds(self._inner, a, b)
                }
            }

            fn minimum(&self) -> f64 {
                unsafe {
                    #minimum(self._inner)
                }
            }

            fn set_minimum(&mut self, a: f64) {
                unsafe {
                    #set_minimum(self._inner, a)
                }
            }

            fn maximum(&self) -> f64 {
                unsafe {
                    #maximum(self._inner)
                }
            }

            fn set_maximum(&mut self, a: f64) {
                unsafe {
                    #set_maximum(self._inner, a)
                }
            }

            fn set_range(&mut self, a: f64, b: f64) {
                unsafe {
                    #set_range(self._inner, a, b)
                }
            }

            fn set_step(&mut self, a: f64, b: i32) {
                unsafe {
                    #set_step(self._inner, a, b)
                }
            }

            fn step(&self) -> f64 {
                unsafe {
                    #step(self._inner)
                }
            }

            fn set_precision(&mut self, digits: i32) {
                unsafe {
                    #set_precision(self._inner, digits)
                }
            }

            fn value(&self) -> f64 {
                unsafe {
                    #value(self._inner)
                }
            }


            fn set_value(&mut self, arg2: f64) {
                unsafe {
                    #set_value(self._inner, arg2);
                }
            }


            fn format(&mut self, arg2: &str) {
                unsafe {
                    let arg2 = CString::new(arg2).unwrap();
                    #format(self._inner, arg2.into_raw() as *mut raw::c_char);
                }
            }

            fn round(&self, arg2: f64) -> f64 {
                unsafe {
                    #round(self._inner, arg2)
                }
            }


            fn clamp(&self, arg2: f64) -> f64 {
                unsafe {
                    #clamp(self._inner, arg2)
                }
            }

            fn increment(&mut self, arg2: f64, arg3: i32) -> f64 {
                unsafe {
                    #increment(self._inner, arg2, arg3)
                }
            }
        }
    };
    gen.into()
}

fn impl_display_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let set_text = Ident::new(format!("{}_{}", name_str, "set_text").as_str(), name.span());
    let text = Ident::new(format!("{}_{}", name_str, "text").as_str(), name.span());
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let append = Ident::new(format!("{}_{}", name_str, "append").as_str(), name.span());
    let buffer_length = Ident::new(
        format!("{}_{}", name_str, "buffer_length").as_str(),
        name.span(),
    );
    let scroll = Ident::new(format!("{}_{}", name_str, "scroll").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let set_insert_position = Ident::new(
        format!("{}_{}", name_str, "set_insert_position").as_str(),
        name.span(),
    );
    let insert_position = Ident::new(
        format!("{}_{}", name_str, "insert_position").as_str(),
        name.span(),
    );
    let count_lines = Ident::new(
        format!("{}_{}", name_str, "count_lines").as_str(),
        name.span(),
    );
    let move_right = Ident::new(
        format!("{}_{}", name_str, "move_right").as_str(),
        name.span(),
    );
    let move_left = Ident::new(
        format!("{}_{}", name_str, "move_left").as_str(),
        name.span(),
    );
    let move_up = Ident::new(format!("{}_{}", name_str, "move_up").as_str(), name.span());
    let move_down = Ident::new(
        format!("{}_{}", name_str, "move_down").as_str(),
        name.span(),
    );
    let remove = Ident::new(
        format!("{}_{}", name_str, "remove").as_str(),
        name.span(),
    );
    let show_cursor = Ident::new(
        format!("{}_{}", name_str, "show_cursor").as_str(),
        name.span(),
    );
    let set_style_table_entry = Ident::new(
        format!("{}_{}", name_str, "set_style_table_entry").as_str(),
        name.span(),
    );
    let set_cursor_style = Ident::new(
        format!("{}_{}", name_str, "set_cursor_style").as_str(),
        name.span(),
    );
    let set_cursor_color = Ident::new(
        format!("{}_{}", name_str, "set_cursor_color").as_str(),
        name.span(),
    );
    let set_scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_size").as_str(),
        name.span(),
    );
    let set_scrollbar_align = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_align").as_str(),
        name.span(),
    );
    let set_scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_width").as_str(),
        name.span(),
    );
    let cursor_style = Ident::new(
        format!("{}_{}", name_str, "cursor_style").as_str(),
        name.span(),
    );
    let cursor_color = Ident::new(
        format!("{}_{}", name_str, "cursor_color").as_str(),
        name.span(),
    );
    let scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "scrollbar_size").as_str(),
        name.span(),
    );
    let scrollbar_align = Ident::new(
        format!("{}_{}", name_str, "scrollbar_align").as_str(),
        name.span(),
    );
    let scrollbar_width = Ident::new(
        format!("{}_{}", name_str, "scrollbar_width").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl DisplayTrait for #name {
            fn set_text(&mut self, txt: &str) {
                unsafe {
                    let txt = CString::new(txt).unwrap();
                    #set_text(self._inner, txt.into_raw() as *const raw::c_char)
                }
            }

            fn text(&self) -> String {
                unsafe {
                    CStr::from_ptr(#text(self._inner) as *mut raw::c_char)
                        .to_string_lossy().to_string()
                }
            }

            fn text_font(&self) -> Font {
                unsafe { mem::transmute(#text_font(self._inner)) }
            }

            fn set_text_font(&mut self, font: Font) {
                unsafe { #set_text_font(self._inner, font as i32) }
            }
            fn text_color(&self) -> Color{
                unsafe { mem::transmute(#text_color(self._inner)) }
            }
            fn set_text_color(&mut self, color: Color){
                unsafe { #set_text_color(self._inner, color as u32) }
            }
            fn text_size(&self) -> usize{
                unsafe { #text_size(self._inner) as usize }
            }
            fn set_text_size(&mut self, sz: usize) {
                unsafe { #set_text_size(self._inner, sz as i32) }
            }
            fn append(&mut self, text: &str) {
                let text = CString::new(text).unwrap();
                unsafe {
                    #append(self._inner, text.into_raw() as *const raw::c_char)
                }
            }
            fn buffer_length(&self) -> usize {
                unsafe {
                    #buffer_length(self._inner) as usize
                }
            }
            fn scroll(&mut self, topLineNum: usize, horizOffset: usize) {
                unsafe {
                    #scroll(self._inner, topLineNum as i32, horizOffset as i32)
                }
            }
            fn insert(&self, text: &str) {
                let text = CString::new(text).unwrap();
                unsafe {
                    #insert(self._inner, text.into_raw() as *const raw::c_char)
                }
            }
            fn set_insert_position(&mut self, newPos: usize) {
                unsafe {
                    #set_insert_position(self._inner, newPos as i32)
                }
            }
            fn insert_position(&self) -> usize {
                unsafe {
                    #insert_position(self._inner) as usize
                }
            }
            fn count_lines(&self, start: usize, end: usize, is_line_start: bool) -> usize {
                let x = match is_line_start {
                    true => 1,
                    false => 0,
                };
                unsafe {
                    #count_lines(self._inner, start as i32, end as i32, x) as usize
                }
            }
            fn move_right(&mut self) {
                unsafe {
                    #move_right(self._inner);
                }
            }
            fn move_left(&mut self){
                unsafe {
                    #move_left(self._inner);
                }
            }
            fn move_up(&mut self){
                unsafe {
                    #move_up(self._inner);
                }
            }
            fn move_down(&mut self){
                unsafe {
                    #move_down(self._inner);
                }
            }
            fn remove(&mut self, start: usize, end: usize) {
                unsafe {
                    #remove(self._inner, start as i32, end as i32);
                }
            }
            fn show_cursor(&mut self, val: bool) {
                unsafe {
                    #show_cursor(self._inner, val as i32);
                }
            }
            fn set_styly_table_entry(&mut self, entries: &Vec<StyleTableEntry>) {
                let mut colors: Vec<u32> = vec![];
                let mut fonts: Vec<i32> = vec![];
                let mut sizes: Vec<i32> = vec![];
                for entry in entries.iter() {
                    colors.push(entry.color as u32);
                    fonts.push(entry.font as i32);
                    sizes.push(entry.size as i32);
                }
                unsafe {
                    #set_style_table_entry(self._inner, &mut colors[0], &mut fonts[0], &mut sizes[0], entries.len() as i32);
                }
            }
            fn set_cursor_style(&mut self, style: CursorStyle) {
                unsafe {
                    #set_cursor_style(self._inner, style as i32)
                }
            }
            fn set_cursor_color(&mut self, color: Color){
                unsafe {
                    #set_cursor_color(self._inner, color as u32)
                }
            }
            fn set_scrollbar_width(&mut self, width: i32){
                unsafe {
                    #set_scrollbar_width(self._inner, width as i32)
                }
            }
            fn set_scrollbar_size(&mut self, size: usize){
                unsafe {
                    #set_scrollbar_size(self._inner, size as i32)
                }
            }
            fn set_scrollbar_align(&mut self, align: Align){
                unsafe {
                    #set_scrollbar_align(self._inner, align as i32)
                }
            }
            fn cursor_style(&self) -> CursorStyle {
                unsafe {
                    mem::transmute(#cursor_style(self._inner))
                }
            }
            fn cursor_color(&self) -> Color {
                unsafe {
                    mem::transmute(#cursor_color(self._inner))
                }
            }
            fn scrollbar_width(&self) -> i32 {
                unsafe {
                    #scrollbar_width(self._inner)
                }
            }
            fn scrollbar_size(&self) -> usize {
                unsafe {
                    #scrollbar_size(self._inner) as usize
                }
            }
            fn scrollbar_align(&self) -> Align {
                unsafe {
                    mem::transmute(#scrollbar_align(self._inner))
                }
            }
        }
    };
    gen.into()
}

fn impl_browser_trait(ast: &syn::DeriveInput) -> TokenStream {
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
    let load_file = Ident::new(format!("{}_{}", name_str, "load_file").as_str(), name.span());

    let gen = quote! {
        impl BrowserTrait for #name {
            fn remove(&mut self, line: usize) {
                unsafe {
                    #remove(self._inner, line as i32)
                }
            }
            fn add(&mut self, item: &str) {
                let item = CString::new(item).unwrap();
                unsafe {
                    #add(self._inner, item.into_raw() as *const raw::c_char)
                }
            }
            fn insert(&mut self, line: usize, item: &str) {
                let item = CString::new(item).unwrap();
                unsafe {
                    #insert(self._inner, line as i32, item.into_raw() as *const raw::c_char)
                }
            }
            fn move_item(&mut self, to: usize, from: usize) {
                unsafe {
                    #move_item(self._inner, to as i32, from as i32)
                }
            }
            fn swap(&mut self, a: usize, b: usize) {
                unsafe {
                    #swap(self._inner, a as i32, b as i32)
                }
            }
            fn clear(&mut self) {
                unsafe {
                    #clear(self._inner)
                }
            }
            fn size(&self) -> usize {
                unsafe {
                    #size(self._inner) as usize
                }
            }
            fn set_size(&mut self, w: i32, h: i32) {
                unsafe {
                    #set_size(self._inner, w, h)
                }
            }
            fn select(&mut self, line: usize) {
                if line < self.size() {
                    unsafe {
                        #select(self._inner, line as i32);
                    }
                }
            }
            fn selected(&self, line: usize) -> bool {
                unsafe {
                    match #selected(self._inner, line as i32) {
                        0 => false,
                        _ => true,
                    }
                }
            }
            fn text(&self, line: usize) -> String {
                unsafe {
                    CStr::from_ptr(#text(self._inner, line as i32) as *mut raw::c_char).to_string_lossy().to_string()
                }
            }
            fn set_text(&mut self, line: usize, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #set_text(self._inner, line as i32, txt.into_raw() as *const raw::c_char)
                }
            }
            fn load_file(&mut self, path: &std::path::Path) {
                let path = path.to_str().unwrap();
                let path = CString::new(path).unwrap();
                unsafe {
                    #load_file(self._inner, path.into_raw() as *const raw::c_char)
                }
            }
        }
    };
    gen.into()
}

fn impl_image_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let draw = Ident::new(format!("{}_{}", name_str, "draw").as_str(), name.span());
    let width = Ident::new(format!("{}_{}", name_str, "width").as_str(), name.span());
    let height = Ident::new(format!("{}_{}", name_str, "height").as_str(), name.span());

    let gen = quote! {
        impl ImageTrait for #name {
            fn new(path: std::path::PathBuf) -> #name {
                unsafe {
                    let temp = path.into_os_string().to_string_lossy().to_string();
                    let temp = CString::new(temp.as_str()).unwrap();
                    #name {
                        _inner: #new(temp.into_raw() as *const raw::c_char),
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

            fn as_ptr(&self) -> *mut raw::c_void {
                unsafe {
                    mem::transmute(self._inner)
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
