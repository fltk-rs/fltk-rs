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

#[proc_macro_derive(ImageExt)]
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
    let set_image_with_size = Ident::new(
        format!("{}_{}", name_str, "set_image_with_size").as_str(),
        name.span(),
    );
    let image = Ident::new(format!("{}_{}", name_str, "image").as_str(), name.span());
    let set_handler = Ident::new(
        format!("{}_{}", name_str, "set_handler").as_str(),
        name.span(),
    );
    let set_trigger = Ident::new(
        format!("{}_{}", name_str, "set_trigger").as_str(),
        name.span(),
    );
    let set_draw = Ident::new(format!("{}_{}", name_str, "set_draw").as_str(), name.span());
    let parent = Ident::new(format!("{}_{}", name_str, "parent").as_str(), name.span());
    let selection_color = Ident::new(
        format!("{}_{}", name_str, "selection_color").as_str(),
        name.span(),
    );
    let set_selection_color = Ident::new(
        format!("{}_{}", name_str, "set_selection_color").as_str(),
        name.span(),
    );
    let do_callback = Ident::new(
        format!("{}_{}", name_str, "do_callback").as_str(),
        name.span(),
    );
    let inside = Ident::new(format!("{}_{}", name_str, "inside").as_str(), name.span());
    let window = Ident::new(format!("{}_{}", name_str, "window").as_str(), name.span());
    let top_window = Ident::new(
        format!("{}_{}", name_str, "top_window").as_str(),
        name.span(),
    );
    let takes_events = Ident::new(
        format!("{}_{}", name_str, "takes_events").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}

        impl From<crate::widget::Widget> for #name {
            fn from(wid: crate::widget::Widget) -> Self {
                let wid: *mut fltk_sys::widget::Fl_Widget = wid.as_ptr();
                assert!(!wid.is_null());
                unsafe {
                    #name {
                        _inner: mem::transmute(wid),
                    }
                }
            }
        }

        impl WidgetExt for #name {
            fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> #name {
                let temp = CString::new(title).unwrap();
                unsafe {
                    let widget_ptr = #new(x, y, width, height, temp.into_raw() as *const raw::c_char);
                    assert!(!widget_ptr.is_null(), "Failed to instantiate widget!");
                    #name {
                        _inner: widget_ptr,
                    }

                }
            }

            fn default() -> Self {
                let temp = CString::new("").unwrap();
                unsafe {
                    let widget_ptr = #new(
                        0,
                        0,
                        0,
                        0,
                        temp.into_raw() as *const raw::c_char);
                        assert!(!widget_ptr.is_null(), "Failed to instantiate widget!");
                    #name {
                        _inner: widget_ptr,
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

            fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self {
                unsafe {
                    #name {
                        _inner: mem::transmute(ptr),
                    }
                }
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

            fn tooltip(&self) -> Option<String> {
                unsafe {
                    let tooltip_ptr = #tooltip(self._inner);
                    if tooltip_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(
                            tooltip_ptr as *mut raw::c_char).to_string_lossy().to_string())
                    }
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

            fn label_size(&self) -> i32 {
                unsafe { #label_size(self._inner) }
            }

            fn set_label_size(&mut self, sz: i32) {
                unsafe { #set_label_size(self._inner, sz) }
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

            fn set_image<Image: ImageExt>(&mut self, image: &Image) {
                unsafe { #set_image(self._inner, image.as_ptr()) }
            }

            fn set_image_with_size<Image: ImageExt>(&mut self, image: &Image, w: i32, h: i32) {
                unsafe { #set_image_with_size(self._inner, image.as_ptr(), w, h) }
            }

            fn image(&self) -> Option<Image> {
                unsafe {
                    let image_ptr = #image(self._inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(image_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }

            fn set_callback<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>) {
                if !self.top_window().unwrap().takes_events() || !self.takes_events() {
                    panic!("The widget failed to capture events, probably it (or the window) is inactive");
                }
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
                if !self.top_window().unwrap().takes_events() || !self.takes_events() {
                    panic!("The widget failed to capture events, probably it (or the window) is inactive");
                }
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
                    #set_handler(self._inner, callback, data);
                }
            }
            
            fn set_custom_draw<'a>(&'a mut self, cb: Box<dyn FnMut() + 'a>) {
                if !self.top_window().unwrap().takes_events() || !self.takes_events() {
                    panic!("The widget failed to capture events, probably it (or the window) is inactive");
                }
                unsafe {
                    unsafe extern "C" fn shim<'a>(data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut() + 'a> = mem::transmute(data);
                        let f: &mut (dyn FnMut() + 'a) = &mut **a;
                        f();
                    }
                    let a: *mut Box<dyn FnMut() + 'a> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: custom_draw_callback = Some(shim);
                    #set_draw(self._inner, callback, data);
                }
            }
            
            fn set_trigger(&mut self, trigger: CallbackTrigger) {
                unsafe {
                    #set_trigger(self._inner, trigger as i32)
                }
            }
            
            fn below_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(self.width() != 0 && self.height() != 0, "below_of requires the size of the widget to be known!");
                self.resize(w.x(), w.y() + w.height() + padding, self.width(), self.height());
                self
            }
            
            fn above_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(self.width() != 0 && self.height() != 0, "above_of requires the size of the widget to be known!");
                self.resize(w.x(), w.y() - padding - self.height(), self.width(), self.height());
                self
            }
            
            fn right_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(self.width() != 0 && self.height() != 0, "right_of requires the size of the widget to be known!");
                self.resize(w.x() + self.width() + padding, w.y(), self.width(), self.height());
                self
            }
            
            fn left_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(self.width() != 0 && self.height() != 0, "left_of requires the size of the widget to be known!");
                self.resize(w.x() - self.width() - padding, w.y(), self.width(), self.height());
                self
            }
            
            fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
                assert!(w.width() != 0 && w.height() != 0, "center_of requires the size of the widget to be known!");
                let mut sw = self.width() as f64;
                let mut sh = self.height() as f64;
                let mut ww = w.width() as f64;
                let mut wh = w.height() as f64;
                let mut x = (ww - sw) / 2.0;
                let mut y = (wh - sh) / 2.0;
                self.resize(x as i32, y as i32, self.width(), self.height());
                self.redraw();
                self
            }
            
            fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
                assert!(w.width() != 0 && w.height() != 0, "size_of requires the size of the widget to be known!");
                self.resize(self.x(), self. y(), w.width(), w.height());
                self
            }
            
            fn parent(&self) -> Option<crate::widget::Widget> {
                unsafe {
                    let x = #parent(self._inner);
                    if x.is_null() {
                        None
                    } else {
                        Some(crate::widget::Widget::from_raw(x as *mut fltk_sys::widget::Fl_Widget))
                    }
                }
            }
            
            fn selection_color(&mut self) -> Color {
                unsafe {
                    mem::transmute(#selection_color(self._inner))
                }
            }
            
            fn set_selection_color(&mut self, color: Color) {
                unsafe {
                    #set_selection_color(self._inner, color as u32);
                }
            }
            
            fn do_callback(&mut self) {
                unsafe {
                    #do_callback(self._inner);
                }
            }
            
            fn inside(&self, wid: crate::widget::Widget) -> bool {
                unsafe {
                    match #inside(self._inner, wid.as_ptr() as *mut raw::c_void) {
                        0 => false,
                        _ => true,
                    }
                }
            }
            
            fn window(&self) -> Option<crate::window::Window> {
                unsafe {
                    let wind_ptr = #window(self._inner);
                    if wind_ptr.is_null() {
                        None
                    } else {
                        Some(crate::window::Window::from_widget_ptr(wind_ptr as *mut fltk_sys::widget::Fl_Widget))
                    }
                }
            }
            
            fn top_window(&self) -> Option<crate::window::Window> {
                unsafe {
                    let wind_ptr = #top_window(self._inner);
                    if wind_ptr.is_null() {
                        None
                    } else {
                        Some(crate::window::Window::from_widget_ptr(wind_ptr as *mut fltk_sys::widget::Fl_Widget))
                    }
                }
            }
            
            fn takes_events(&self) -> bool {
                unsafe {
                    match #takes_events(self._inner) {
                        0 => false,
                        _ => true,
                    }
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

fn impl_button_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());

    let shortcut = Ident::new(format!("{}_{}", name_str, "shortcut").as_str(), name.span());
    let set_shortcut = Ident::new(format!("{}_{}", name_str, "set_shortcut").as_str(), name.span());

    let gen = quote! {
        impl ButtonExt for #name {
            fn shortcut(&self) -> Shortcut {
                unsafe {
                    mem::transmute(#shortcut(self._inner))
                }
            }
            
            fn set_shortcut(&mut self, shortcut: Shortcut) {
                unsafe {
                    #set_shortcut(self._inner, shortcut as i32)
                }
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
    let child = Ident::new(format!("{}_{}", name_str, "child").as_str(), name.span());
    let resizable = Ident::new(
        format!("{}_{}", name_str, "resizable").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl GroupExt for #name {
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
                    assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #insert(self._inner, widget.as_widget_ptr() as *mut raw::c_void, index as i32)
                }
            }
            
            fn remove(&mut self, index: u32) {
                unsafe {
                    assert!(index <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #remove(self._inner, index as i32)
                }
            }
            
            fn clear(&mut self) {
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
                    assert!(idx <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let make_resizable = Ident::new(
        format!("{}_{}", name_str, "make_resizable").as_str(),
        name.span(),
    );
    let gen = quote! {
        impl WindowExt for #name {
            fn center_screen(mut self) -> Self {
                assert!(self.width() != 0 && self.height() != 0, "center_screen requires the size of the widget to be known!");
                let (mut x, mut y) = screen_size();
                x = x - self.width() as f64;
                y = y - self.height() as f64;
                self.resize((x / 2.0) as i32, (y / 2.0) as i32, self.width(), self.height());
                self
            }
            
            fn make_modal(&mut self, val: bool) {
                unsafe { #make_modal(self._inner, val as u32) }
            }

            fn fullscreen(&mut self, val: bool) {
                unsafe { #fullscreen(self._inner, val as u32) }
            }

            fn make_current(&mut self) {
                unsafe { #make_current(self._inner) }
            }
            
            fn set_icon<Image: ImageExt>(&mut self, image: &Image) {
                unsafe { #set_icon(self._inner, image.as_ptr()) }
            }
            
            fn icon(&self) -> Option<Image> {
                unsafe {
                    let icon_ptr = #icon(self._inner);
                    if icon_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(icon_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }
            
            fn make_resizable(&mut self, val: bool) {
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
        impl InputExt for #name {
            fn value(&self) -> String {
                unsafe {
                    let value_ptr = #value(self._inner);
                    assert!(!value_ptr.is_null(), "Failed to retrieve input/output value!");
                    CStr::from_ptr(value_ptr as *mut raw::c_char).to_string_lossy().to_string()
                }
            }
            
            fn set_value(&self, val: &str) {
                let temp = CString::new(val).unwrap();
                unsafe {
                    #set_value(self._inner, temp.into_raw() as *const raw::c_char);
                }
            }
            
            fn maximum_size(&self) -> u32 {
                unsafe {
                    #maximum_size(self._inner) as u32
                }
            }
            
            fn set_maximum_size(&mut self, val: u32) {
                unsafe {
                    assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
                    #mark(self._inner)
                }
            }
            
            fn set_mark(&mut self, val: i32) {
                unsafe {
                    #set_mark(self._inner, val);
                }
            }
            
            fn replace(&mut self, beg: u32, end: u32, val: &str) {
                let val = CString::new(val).unwrap();
                unsafe {
                    assert!(beg <= std::i32::MAX as u32 && end <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
            
            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }
            
            fn set_text_size(&mut self, sz: u32) {
                unsafe {
                    assert!(sz <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
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
        impl MenuExt for #name {
            fn add<'a>(&'a mut self, name: &str, shortcut: Shortcut, flag: MenuFlag, cb: Box<dyn FnMut() + 'a>) {
                if !self.top_window().unwrap().takes_events() || !self.takes_events() {
                    panic!("The widget failed to capture events, probably it (or the window) is inactive");
                }
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
            
            fn insert<'a>(&'a mut self, idx: u32, name: &str, shortcut: Shortcut, flag: MenuFlag, cb: Box<dyn FnMut() + 'a>) {
                if !self.top_window().unwrap().takes_events() || !self.takes_events() {
                    panic!("The widget failed to capture events, probably it (or the window) is inactive");
                }
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
                    #insert(self._inner, idx as i32, temp.into_raw() as *const raw::c_char, shortcut as i32, callback, data, flag as i32);
                }
            }
            
            fn get_item(&self, name: &str) -> Option<MenuItem> {
                let name = CString::new(name).unwrap().clone();
                unsafe {
                    let menu_item = #get_item(
                        self._inner,
                        name.into_raw() as *const raw::c_char);
                    if menu_item.is_null() {
                        None
                    } else {
                        Some(MenuItem {
                            _inner: menu_item,
                        })
                    }
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

            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, c: u32) {
                unsafe {
                    assert!(c <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
            
            fn get_choice(&self) -> Option<String> {
                unsafe {
                    let choice_ptr = #get_choice(self._inner);
                    if choice_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(choice_ptr as *mut raw::c_char).to_string_lossy().to_string())
                    }
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
        impl ValuatorExt for #name {
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

    let get_buffer = Ident::new(
        format!("{}_{}", name_str, "get_buffer").as_str(),
        name.span(),
    );
    let set_buffer = Ident::new(
        format!("{}_{}", name_str, "set_buffer").as_str(),
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
    let position_to_xy = Ident::new(
        format!("{}_{}", name_str, "position_to_xy").as_str(),
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
    let line_start = Ident::new(
        format!("{}_{}", name_str, "line_start").as_str(),
        name.span(),
    );
    let line_end = Ident::new(format!("{}_{}", name_str, "line_end").as_str(), name.span());
    let skip_lines = Ident::new(
        format!("{}_{}", name_str, "skip_lines").as_str(),
        name.span(),
    );
    let rewind_lines = Ident::new(
        format!("{}_{}", name_str, "rewind_lines").as_str(),
        name.span(),
    );
    let next_word = Ident::new(
        format!("{}_{}", name_str, "next_word").as_str(),
        name.span(),
    );
    let previous_word = Ident::new(
        format!("{}_{}", name_str, "previous_word").as_str(),
        name.span(),
    );
    let word_start = Ident::new(
        format!("{}_{}", name_str, "word_start").as_str(),
        name.span(),
    );
    let word_end = Ident::new(format!("{}_{}", name_str, "word_end").as_str(), name.span());
    let x_to_col = Ident::new(format!("{}_{}", name_str, "x_to_col").as_str(), name.span());
    let col_to_x = Ident::new(format!("{}_{}", name_str, "col_to_x").as_str(), name.span());
    let set_linenumber_width = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_width").as_str(),
        name.span(),
    );
    let linenumber_width = Ident::new(
        format!("{}_{}", name_str, "linenumber_width").as_str(),
        name.span(),
    );
    let set_linenumber_font = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_font").as_str(),
        name.span(),
    );
    let linenumber_font = Ident::new(
        format!("{}_{}", name_str, "linenumber_font").as_str(),
        name.span(),
    );
    let set_linenumber_size = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_size").as_str(),
        name.span(),
    );
    let linenumber_size = Ident::new(
        format!("{}_{}", name_str, "linenumber_size").as_str(),
        name.span(),
    );
    let set_linenumber_fgcolor = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_fgcolor").as_str(),
        name.span(),
    );
    let linenumber_fgcolor = Ident::new(
        format!("{}_{}", name_str, "linenumber_fgcolor").as_str(),
        name.span(),
    );
    let set_linenumber_bgcolor = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_bgcolor").as_str(),
        name.span(),
    );
    let linenumber_bgcolor = Ident::new(
        format!("{}_{}", name_str, "linenumber_bgcolor").as_str(),
        name.span(),
    );
    let set_linenumber_align = Ident::new(
        format!("{}_{}", name_str, "set_linenumber_align").as_str(),
        name.span(),
    );
    let linenumber_align = Ident::new(
        format!("{}_{}", name_str, "linenumber_align").as_str(),
        name.span(),
    );
    let in_selection = Ident::new(
        format!("{}_{}", name_str, "in_selection").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl DisplayExt for #name {
            fn buffer<'a>(&'a self) -> &'a TextBuffer {
                unsafe {
                    let buffer = #get_buffer(self._inner);
                    assert!(!buffer.is_null(), "Failed to get associated buffer!");
                    let x = Box::from(TextBuffer::from_ptr(buffer));
                    &*Box::into_raw(x)
                }
            }
            
            fn set_buffer<'a>(&'a mut self, buffer: &'a mut TextBuffer) {
                unsafe {
                    #set_buffer(self._inner, buffer.as_ptr())
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
            
            fn text_size(&self) -> u32{
                unsafe { #text_size(self._inner) as u32 }
            }
            
            fn set_text_size(&mut self, sz: u32) {
                assert!(sz <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe { #set_text_size(self._inner, sz as i32) }
            }
            
            fn scroll(&mut self, topLineNum: u32, horizOffset: u32) {
                unsafe {
                    assert!(topLineNum <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    assert!(horizOffset <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #scroll(self._inner, topLineNum as i32, horizOffset as i32)
                }
            }
            
            fn insert(&self, text: &str) {
                let text = CString::new(text).unwrap();
                unsafe {
                    #insert(self._inner, text.into_raw() as *const raw::c_char)
                }
            }
            
            fn set_insert_position(&mut self, newPos: u32) {
                unsafe {
                    assert!(newPos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #set_insert_position(self._inner, newPos as i32)
                }
            }
            
            fn insert_position(&self) -> u32 {
                unsafe {
                    #insert_position(self._inner) as u32
                }
            }
            
            fn position_to_xy(&self, pos: u32) -> (u32, u32) {
                assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    let mut x: i32 = 0;
                    let mut y: i32 = 0;
                    #position_to_xy(self._inner, pos as i32, &mut x, &mut y);
                    (x as u32, y as u32)
                }
            }
            
            fn count_lines(&self, start: u32, end: u32, is_line_start: bool) -> u32 {
                let x = match is_line_start {
                    true => 1,
                    false => 0,
                };
                assert!(start <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(end <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #count_lines(self._inner, start as i32, end as i32, x) as u32
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
            
            fn show_cursor(&mut self, val: bool) {
                unsafe {
                    #show_cursor(self._inner, val as i32);
                }
            }
            
            fn set_styly_table_entry(&mut self, style_buffer: &mut TextBuffer, entries: &Vec<StyleTableEntry>) {
                let mut colors: Vec<u32> = vec![];
                let mut fonts: Vec<i32> = vec![];
                let mut sizes: Vec<i32> = vec![];
                for entry in entries.iter() {
                    colors.push(entry.color as u32);
                    fonts.push(entry.font as i32);
                    sizes.push(entry.size as i32);
                }
                unsafe {
                    #set_style_table_entry(self._inner, style_buffer.as_ptr() as *mut raw::c_void, &mut colors[0], &mut fonts[0], &mut sizes[0], entries.len() as i32);
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
                    #set_scrollbar_width(self._inner, width)
                }
            }
            
            fn set_scrollbar_size(&mut self, size: u32){
                assert!(size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
            
            fn scrollbar_width(&self) -> u32 {
                unsafe {
                    #scrollbar_width(self._inner) as u32
                }
            }
            
            fn scrollbar_size(&self) -> u32 {
                unsafe {
                    #scrollbar_size(self._inner) as u32
                }
            }
            
            fn scrollbar_align(&self) -> Align {
                unsafe {
                    mem::transmute(#scrollbar_align(self._inner))
                }
            }
            
            fn line_start(&self, pos: u32) -> u32 {
                assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #line_start(self._inner, pos as i32) as u32
                }
            }
            
            fn line_end(&self, start_pos: u32, is_line_start: bool) -> u32 {
                assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #line_end(self._inner, start_pos as i32, is_line_start as i32) as u32
                }
            }
            
            fn skip_lines(&mut self, start_pos: u32, lines: u32, is_line_start: bool) -> u32 {
                assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(lines <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #skip_lines(self._inner, start_pos as i32, lines as i32, is_line_start as i32) as u32
                }
            }
            
            fn rewind_lines(&mut self, start_pos: u32, lines: u32) -> u32 {
                assert!(start_pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(lines <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #rewind_lines(self._inner, start_pos as i32, lines as i32) as u32
                }
            }
            
            fn next_word(&mut self) {
                unsafe {
                    #next_word(self._inner)
                }
            }
            
            fn previous_word(&mut self) {
                unsafe {
                    #previous_word(self._inner)
                }
            }
            
            fn word_start(&self, pos: u32) -> u32 {
                assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #word_start(self._inner, pos as i32) as u32
                }
            }
            
            fn word_end(&self, pos: u32) -> u32 {
                assert!(pos <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #word_end(self._inner, pos as i32) as u32
                }
            }
            
            fn x_to_col(&self, x: f64) -> f64 {
                unsafe {
                    #x_to_col(self._inner, x)
                }
            }
            
            fn col_to_x(&self, col: f64) -> f64 {
                unsafe {
                    #col_to_x(self._inner, col)
                }
            }
            
            fn set_linenumber_width(&mut self, w: i32) {
                unsafe {
                    #set_linenumber_width(self._inner, w)
                }
            }
            
            fn linenumber_width(&self) -> i32 {
                unsafe {
                    #linenumber_width(self._inner)
                }
            }
            
            fn set_linenumber_font(&mut self, font: Font) {
                unsafe {
                    #set_linenumber_font(self._inner, font as i32)
                }
            }
            
            fn linenumber_font(&self) -> Font {
                unsafe {
                    mem::transmute(#linenumber_font(self._inner))
                }
            }
            
            fn set_linenumber_size(&mut self, size: u32) {
                assert!(size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #set_linenumber_size(self._inner, size as i32)
                }
            }
            
            fn linenumber_size(&self) -> u32 {
                unsafe {
                    #linenumber_size(self._inner) as u32
                }
            }
            
            fn set_linenumber_fgcolor(&mut self, color: Color) {
                unsafe {
                    #set_linenumber_fgcolor(self._inner, color as u32)
                }
            }
            
            fn linenumber_fgcolor(&self) -> Color {
                unsafe {
                    mem::transmute(#linenumber_fgcolor(self._inner))
                }
            }
            
            fn set_linenumber_bgcolor(&mut self, color: Color) {
                unsafe {
                    #set_linenumber_bgcolor(self._inner, color as u32)
                }
            }
            
            fn linenumber_bgcolor(&self) -> Color {
                unsafe {
                    mem::transmute(#linenumber_bgcolor(self._inner))
                }
            }
            
            fn set_linenumber_align(&mut self, align: Align) {
                unsafe {
                    #set_linenumber_align(self._inner, align as i32)
                }
            }
            
            fn linenumber_align(&self) -> Align {
                unsafe {
                    mem::transmute(#linenumber_align(self._inner))
                }
            }
            
            fn in_selection(&self, x: i32, y: i32) -> bool {
                unsafe {
                    match #in_selection(self._inner, x, y) {
                        0 => false,
                        _ => true,
                    }
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

    let gen = quote! {
        impl BrowserExt for #name {
            fn remove(&mut self, line: u32) {
                assert!(line > 0, "Lines start at 1!");
                assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
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
            
            fn insert(&mut self, line: u32, item: &str) {
                assert!(line > 0, "Lines start at 1!");
                assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                let item = CString::new(item).unwrap();
                unsafe {
                    #insert(self._inner, line as i32, item.into_raw() as *const raw::c_char)
                }
            }
            
            fn move_item(&mut self, to: u32, from: u32) {
                assert!(to <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(from <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #move_item(self._inner, to as i32, from as i32)
                }
            }
            
            fn swap(&mut self, a: u32, b: u32) {
                assert!(a <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                assert!(b <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    #swap(self._inner, a as i32, b as i32)
                }
            }
            
            fn clear(&mut self) {
                unsafe {
                    #clear(self._inner)
                }
            }
            
            fn size(&self) -> u32 {
                unsafe {
                    #size(self._inner) as u32
                }
            }
            
            fn set_size(&mut self, w: i32, h: i32) {
                unsafe {
                    #set_size(self._inner, w, h)
                }
            }
            
            fn select(&mut self, line: u32) {
                assert!(line > 0, "Lines start at 1!");
                if line < self.size() {
                    unsafe {
                        #select(self._inner, line as i32);
                    }
                }
            }
            
            fn selected(&self, line: u32) -> bool {
                assert!(line > 0, "Lines start at 1!");
                assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    match #selected(self._inner, line as i32) {
                        0 => false,
                        _ => true,
                    }
                }
            }
            
            fn text(&self, line: u32) -> Option<String> {
                assert!(line > 0, "Lines start at 1!");
                assert!(line <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    let text = #text(self._inner, line as i32);
                    if text.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(text as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }
            
            fn set_text(&mut self, line: u32, txt: &str) {
                assert!(line > 0, "Lines start at 1!");
                assert!(line <= self.size(), "Line doesn't exist!");
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
            
            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }
            
            fn set_text_size(&mut self, c: u32) {
                unsafe {
                    #set_text_size(self._inner, c as i32)
                }
            }
            
            fn set_icon<Img: ImageExt>(&mut self, line: u32, image: &Img) {
                assert!(line > 0, "Lines start at 1!");
                unsafe {
                    #set_icon(self._inner, line as i32, image.as_ptr())
                }
            }
            
            fn icon(&self, line: u32) -> Option<Image> {
                assert!(line > 0, "Lines start at 1!");
                unsafe {
                    let icon_ptr = #icon(self._inner, line as i32);
                    if icon_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(icon_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }
            
            fn remove_icon(&mut self, line: u32) {
                assert!(line > 0, "Lines start at 1!");
                unsafe {
                    #remove_icon(self._inner, line as i32)
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
    let delete = Ident::new(format!("{}_{}", name_str, "delete").as_str(), name.span());
    let count = Ident::new(format!("{}_{}", name_str, "count").as_str(), name.span());
    let data = Ident::new(format!("{}_{}", name_str, "data").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());

    let gen = quote! {
        unsafe impl Sync for #name {}
        unsafe impl Send for #name {}

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe { #delete(self._inner) }
            }
        }

        impl ImageExt for #name {
            fn new(path: &std::path::Path) -> #name {
                assert!(path.exists(), "Proper image initialization requires an existent path!");
                unsafe {
                    let temp = path.to_str().unwrap();
                    let temp = CString::new(temp).unwrap();
                    let image_ptr = #new(temp.into_raw() as *const raw::c_char);
                    assert!(!image_ptr.is_null(), "Image invalid or doesn't exist!");
                    #name {
                        _inner: image_ptr,
                    }
                }
            }

            fn copy(&self) -> Self {
                unsafe {
                    let img = #copy(self._inner);
                    assert!(!img.is_null(), "Coulnd't copy image!");
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

            fn as_ptr(&self) -> *mut raw::c_void {
                unsafe {
                    mem::transmute(self._inner)
                }
            }
            
            fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image {
                unsafe {
                    mem::transmute(self._inner)
                }
            }
            
            fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
                unsafe {
                    assert!(!ptr.is_null(), "Image pointer is null!");
                    #name {
                        _inner: mem::transmute(ptr),
                    }
                }
            }
            
            fn as_bytes<'a>(&self) -> &'a [u8] {
                unsafe {
                    let ptr = #data(self._inner);
                    let cnt = #width(self._inner) * #height(self._inner) * 3;
                    let ret: &[u8] = std::slice::from_raw_parts(ptr as *const u8, cnt as usize);
                    ret
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
