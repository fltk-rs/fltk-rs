use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_widget_base_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(name_str.as_str(), name.span());
    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let handle = Ident::new(format!("{}_{}", name_str, "handle").as_str(), name.span());
    let draw = Ident::new(format!("{}_{}", name_str, "draw").as_str(), name.span());
    let handle_data = Ident::new(
        format!("{}_{}", name_str, "handle_data").as_str(),
        name.span(),
    );
    let draw_data = Ident::new(
        format!("{}_{}", name_str, "draw_data").as_str(),
        name.span(),
    );
    let set_handle_data = Ident::new(
        format!("{}_{}", name_str, "set_handle_data").as_str(),
        name.span(),
    );
    let set_deleter = Ident::new(
        format!("{}_{}", name_str, "set_deleter").as_str(),
        name.span(),
    );

    let gen = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self::new(0, 0, 0, 0, None)
            }
        }

        unsafe impl WidgetBase for #name {
            fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, width: i32, height: i32, title: T) -> #name {
                let temp = if let Some(title) = title.into() {
                    CString::safe_new(title).into_raw()
                } else {
                    std::ptr::null_mut()
                };
                unsafe {
                    let widget_ptr = #new(x, y, width, height, temp);
                    assert!(!widget_ptr.is_null());
                    let tracker =
                        fltk_sys::fl::Fl_Widget_Tracker_new(widget_ptr as *mut fltk_sys::fl::Fl_Widget);
                    assert!(!tracker.is_null());
                    unsafe extern "C" fn shim(data: *mut raw::c_void) {
                        if !data.is_null() {
                            let x = data as *mut Box<dyn FnMut()>;
                            let _x = Box::from_raw(x);
                        }
                    }
                    #set_deleter(widget_ptr, Some(shim));
                    #name {
                        inner: widget_ptr,
                        tracker: tracker,
                    }
                }
            }

            fn delete(mut wid: Self) {
                assert!(!wid.was_deleted());
                unsafe {
                    fltk_sys::fl::Fl_delete_widget(wid.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget);
                    wid.inner = std::ptr::null_mut() as *mut _;
                    wid.tracker = std::ptr::null_mut() as *mut fltk_sys::fl::Fl_Widget_Tracker;
                }
            }

            unsafe fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self {
                assert!(!ptr.is_null());
                fltk_sys::fl::Fl_lock();
                let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(ptr as *mut fltk_sys::fl::Fl_Widget);
                assert!(!tracker.is_null());
                let temp = #name {
                    inner: ptr as *mut #ptr_name,
                    tracker: tracker,
                };
                fltk_sys::fl::Fl_unlock();
                temp
            }

            unsafe fn from_widget<W: WidgetExt>(w: W) -> Self {
                Self::from_widget_ptr(w.as_widget_ptr() as *mut _)
            }

            fn handle<F: FnMut(&mut Self, Event) -> bool + 'static>(&mut self, cb: F) {
                assert!(!self.was_deleted());
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, ev: std::os::raw::c_int, data: *mut raw::c_void) -> i32 {
                        let mut wid = #name::from_widget_ptr(wid as *mut _);
                        let ev: Event = mem::transmute(ev);
                        let a: *mut Box<dyn FnMut(&mut #name, Event) -> bool> = data as *mut Box<dyn FnMut(&mut #name, Event) -> bool>;
                        let f: &mut (dyn FnMut(&mut #name, Event) -> bool) = &mut **a;
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match f(&mut wid, ev) {
                            true => return 1,
                            false => return 0,
                        }));
                        if let Ok(ret) = result {
                            ret
                        } else {
                            0
                        }
                    }
                    let _old_data = self.handle_data();
                    let a: *mut Box<dyn FnMut(&mut Self, Event) -> bool> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: custom_handler_callback = Some(shim);
                    #handle(self.inner, callback, data);
                }
            }

            fn draw<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
                assert!(!self.was_deleted());
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = #name::from_widget_ptr(wid as *mut _);
                        let a: *mut Box<dyn FnMut(&mut #name)> = data as *mut Box<dyn FnMut(&mut #name)>;
                        let f: &mut (dyn FnMut(&mut #name)) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let _old_data = self.draw_data();
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: custom_draw_callback = Some(shim);
                    #draw(self.inner, callback, data);
                }
            }

            unsafe fn draw_data(&mut self) -> Option<Box<dyn FnMut()>> {
                let ptr = #draw_data(self.inner);
                if ptr.is_null() {
                    return None;
                }
                let data = ptr as *mut Box<dyn FnMut()>;
                let data = Box::from_raw(data);
                #draw(self.inner, None, std::ptr::null_mut());
                Some(*data)
            }

            unsafe fn handle_data(&mut self) -> Option<Box<dyn FnMut(Event) -> bool>> {
                let ptr = #handle_data(self.inner);
                if ptr.is_null() {
                    return None;
                }
                let data = ptr as *mut Box<dyn FnMut(Event) -> bool>;
                let data = Box::from_raw(data);
                #handle(self.inner, None, std::ptr::null_mut());
                Some(*data)
            }
        }
    };
    gen.into()
}

pub fn impl_widget_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(name_str.as_str(), name.span());
    let x = Ident::new(format!("{}_{}", name_str, "x").as_str(), name.span());
    let y = Ident::new(format!("{}_{}", name_str, "y").as_str(), name.span());
    let width = Ident::new(format!("{}_{}", name_str, "width").as_str(), name.span());
    let height = Ident::new(format!("{}_{}", name_str, "height").as_str(), name.span());
    let label = Ident::new(format!("{}_{}", name_str, "label").as_str(), name.span());
    let measure_label = Ident::new(
        format!("{}_{}", name_str, "measure_label").as_str(),
        name.span(),
    );
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
    let widget_resize = Ident::new(
        format!("{}_{}", name_str, "widget_resize").as_str(),
        name.span(),
    );
    let tooltip = Ident::new(format!("{}_{}", name_str, "tooltip").as_str(), name.span());
    let set_tooltip = Ident::new(
        format!("{}_{}", name_str, "set_tooltip").as_str(),
        name.span(),
    );
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
    let trigger = Ident::new(format!("{}_{}", name_str, "when").as_str(), name.span());
    let set_trigger = Ident::new(format!("{}_{}", name_str, "set_when").as_str(), name.span());
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
    let window = Ident::new(format!("{}_{}", name_str, "window").as_str(), name.span());
    let top_window = Ident::new(
        format!("{}_{}", name_str, "top_window").as_str(),
        name.span(),
    );
    let takes_events = Ident::new(
        format!("{}_{}", name_str, "takes_events").as_str(),
        name.span(),
    );
    let set_callback = Ident::new(
        format!("{}_{}", name_str, "set_callback").as_str(),
        name.span(),
    );
    let user_data = Ident::new(
        format!("{}_{}", name_str, "user_data").as_str(),
        name.span(),
    );
    let take_focus = Ident::new(
        format!("{}_{}", name_str, "take_focus").as_str(),
        name.span(),
    );
    let set_visible_focus = Ident::new(
        format!("{}_{}", name_str, "set_visible_focus").as_str(),
        name.span(),
    );
    let clear_visible_focus = Ident::new(
        format!("{}_{}", name_str, "clear_visible_focus").as_str(),
        name.span(),
    );
    let visible_focus = Ident::new(
        format!("{}_{}", name_str, "visible_focus").as_str(),
        name.span(),
    );
    let has_visible_focus = Ident::new(
        format!("{}_{}", name_str, "has_visible_focus").as_str(),
        name.span(),
    );
    let set_handle_data = Ident::new(
        format!("{}_{}", name_str, "set_handle_data").as_str(),
        name.span(),
    );
    let damage = Ident::new(format!("{}_{}", name_str, "damage").as_str(), name.span());
    let set_damage = Ident::new(
        format!("{}_{}", name_str, "set_damage").as_str(),
        name.span(),
    );
    let clear_damage = Ident::new(
        format!("{}_{}", name_str, "clear_damage").as_str(),
        name.span(),
    );
    let as_window = Ident::new(
        format!("{}_{}", name_str, "as_window").as_str(),
        name.span(),
    );
    let as_group = Ident::new(format!("{}_{}", name_str, "as_group").as_str(), name.span());
    let inside = Ident::new(format!("{}_{}", name_str, "inside").as_str(), name.span());
    let get_type = Ident::new(format!("{}_{}", name_str, "get_type").as_str(), name.span());
    let set_type = Ident::new(format!("{}_{}", name_str, "set_type").as_str(), name.span());
    let set_image = Ident::new(
        format!("{}_{}", name_str, "set_image").as_str(),
        name.span(),
    );
    let set_image_with_size = Ident::new(
        format!("{}_{}", name_str, "set_image_with_size").as_str(),
        name.span(),
    );
    let image = Ident::new(format!("{}_{}", name_str, "image").as_str(), name.span());
    let deimage = Ident::new(format!("{}_{}", name_str, "deimage").as_str(), name.span());
    let set_deimage = Ident::new(
        format!("{}_{}", name_str, "set_deimage").as_str(),
        name.span(),
    );
    let set_callback = Ident::new(
        format!("{}_{}", name_str, "set_callback").as_str(),
        name.span(),
    );
    let visible = Ident::new(format!("{}_{}", name_str, "visible").as_str(), name.span());
    let visible_r = Ident::new(
        format!("{}_{}", name_str, "visible_r").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}

        impl Clone for #name {
            fn clone(&self) -> #name {
                assert!(!self.was_deleted());                
                #name { inner: self.inner, tracker: self.tracker }
            }
        }

        unsafe impl WidgetExt for #name {
            fn set_pos(&mut self, x: i32, y: i32) {
                self.resize(x, y, self.width(), self.height());
            }

            fn set_size(&mut self, width: i32, height: i32) {
                self.resize(self.x(), self.y(), width, height);
            }

            fn set_label(&mut self, title: &str) {
                assert!(!self.was_deleted());
                let temp = CString::safe_new(title);
                unsafe {
                    #set_label(
                        self.inner,
                        temp.as_ptr(),
                    )
                }
            }

            fn redraw(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #redraw(self.inner);
                }
            }

            fn show(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #show(self.inner) }
            }

            fn hide(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #hide(self.inner) }
            }

            fn x(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #x(self.inner)}
            }

            fn y(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #y(self.inner) }
            }

            fn width(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #width(self.inner) }
            }

            fn height(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #height(self.inner) }
            }

            fn w(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #width(self.inner) }
            }

            fn h(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #height(self.inner) }
            }

            fn label(&self) -> String {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #label(self.inner) as *mut raw::c_char;
                    if ptr.is_null() {
                        String::from("")
                    } else {
                        CStr::from_ptr(ptr).to_string_lossy().to_string()
                    }
                }
            }

            fn measure_label(&self) -> (i32, i32) {
                assert!(!self.was_deleted());
                let mut x = 0;
                let mut y = 0;
                unsafe {
                    #measure_label(self.inner, &mut x, &mut y);
                }
                (x, y)
            }

            unsafe fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
                self.inner as *mut fltk_sys::widget::Fl_Widget
            }

            fn activate(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #activate(self.inner) }
            }

            fn deactivate(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #deactivate(self.inner) }
            }

            fn redraw_label(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #redraw_label(self.inner) }
            }

            fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                assert!(!self.was_deleted());
                unsafe { #resize(self.inner, x, y, width, height) }
            }

            fn tooltip(&self) -> Option<String> {
                assert!(!self.was_deleted());
                unsafe {
                    let tooltip_ptr = #tooltip(self.inner);
                    if tooltip_ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(
                            tooltip_ptr as *mut raw::c_char).to_string_lossy().to_string())
                    }
                }
            }

            fn set_tooltip(&mut self, txt: &str) {
                assert!(!self.was_deleted());
                let txt = CString::safe_new(txt);
                unsafe {
                    #set_tooltip(
                        self.inner,
                        txt.as_ptr() as *mut raw::c_char,
                    )
                }
            }

            fn color(&self) -> Color {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#color(self.inner)) }
            }

            fn set_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe { #set_color(self.inner, color.bits() as u32) }
            }

            fn label_color(&self) -> Color {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_color(self.inner)) }
            }

            fn set_label_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe { #set_label_color(self.inner, color.bits() as u32) }
            }

            fn label_font(&self) -> Font {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_font(self.inner)) }
            }

            fn set_label_font(&mut self, font: Font) {
                assert!(!self.was_deleted());
                unsafe { #set_label_font(self.inner, font.bits() as i32) }
            }

            fn label_size(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #label_size(self.inner) }
            }

            fn set_label_size(&mut self, sz: i32) {
                assert!(!self.was_deleted());
                let sz = if sz < 1 { 1 } else { sz };
                unsafe { #set_label_size(self.inner, sz) }
            }

            fn label_type(&self) -> LabelType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_type(self.inner)) }
            }

            fn set_label_type(&mut self, typ: LabelType) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_label_type(self.inner, typ as i32);
                }
            }

            fn frame(&self) -> FrameType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#frame(self.inner)) }
            }

            fn set_frame(&mut self, typ: FrameType) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_frame(self.inner, typ as i32);
                }
            }

            fn changed(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #changed(self.inner)  != 0
                }
            }

            fn set_changed(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #set_changed(self.inner) }
            }

            fn clear_changed(&mut self) {
                assert!(!self.was_deleted());
                unsafe {#clear_changed(self.inner) }
            }

            fn align(&self) -> Align {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#align(self.inner)) }
            }

            fn set_align(&mut self, align: Align) {
                assert!(!self.was_deleted());
                unsafe { #set_align(self.inner, align.bits() as i32) }
            }

            fn set_trigger(&mut self, trigger: CallbackTrigger) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_trigger(self.inner, trigger.bits() as i32)
                }
            }

            fn trigger(&self) -> CallbackTrigger {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#trigger(self.inner))
                }
            }

            fn parent(&self) -> Option<Box<dyn GroupExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let x = #parent(self.inner);
                    if x.is_null() {
                        None
                    } else {
                        Some(Box::new(crate::group::Group::from_widget_ptr(x as *mut _)))
                    }
                }
            }

            fn selection_color(&mut self) -> Color {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#selection_color(self.inner))
                }
            }

            fn set_selection_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_selection_color(self.inner, color.bits() as u32);
                }
            }

            fn do_callback(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #do_callback(self.inner);
                }
            }

            fn window(&self) -> Option<Box<dyn WindowExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let wind_ptr = #window(self.inner);
                    if wind_ptr.is_null() {
                        None
                    } else {
                        Some(Box::new(crate::window::Window::from_widget_ptr(wind_ptr as *mut fltk_sys::widget::Fl_Widget)))
                    }
                }
            }

            fn top_window(&self) -> Option<Box<dyn WindowExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let wind_ptr = #top_window(self.inner);
                    if wind_ptr.is_null() {
                        None
                    } else {
                        Some(Box::new(crate::window::Window::from_widget_ptr(wind_ptr as *mut fltk_sys::widget::Fl_Widget)))
                    }
                }
            }

            fn takes_events(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #takes_events(self.inner)  != 0
                }
            }

            unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
                let ptr = #user_data(self.inner);
                if ptr.is_null() {
                    None
                } else {
                    let x = ptr as *mut Box<dyn FnMut()>;
                    let x = Box::from_raw(x);
                    #set_callback(self.inner, None, std::ptr::null_mut());
                    Some(*x)
                }
            }

            fn take_focus(&mut self) -> Result<(), FltkError> {
                assert!(!self.was_deleted());
                unsafe {
                    match #take_focus(self.inner) {
                        0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                        _ => Ok(()),
                    }
                }
            }


            fn set_visible_focus(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_visible_focus(self.inner)
                }
            }


            fn clear_visible_focus(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #clear_visible_focus(self.inner)
                }
            }


            fn visible_focus(&mut self, v: bool) {
                assert!(!self.was_deleted());
                unsafe {
                    #visible_focus(self.inner, v as i32)
                }
            }


            fn has_visible_focus(&mut self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #has_visible_focus(self.inner) != 0
                }
            }

            fn was_deleted(&self) -> bool {
                unsafe {
                    self.inner.is_null() || self.tracker.is_null() || fltk_sys::fl::Fl_Widget_Tracker_deleted(self.tracker) != 0
                }
            }

            fn damage(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #damage(self.inner) != 0
                }
            }

            fn set_damage(&mut self, flag: bool) {
                assert!(!self.was_deleted());
                let flag = if flag { 10 } else { 0 };
                unsafe {
                    #set_damage(self.inner, flag)
                }
            }

            fn damage_type(&self) -> Damage {
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#damage(self.inner))
                }
            }

            fn set_damage_type(&mut self, mask: Damage) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_damage(self.inner, mask.bits())
                }
            }

            fn clear_damage(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #clear_damage(self.inner)
                }
            }

            fn as_window(&self) -> Option<Box<dyn WindowExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #as_window(self.inner);
                    if ptr.is_null() {
                        return None;
                    }
                    Some(Box::new(crate::window::Window::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)))
                }
            }

            fn as_group(&self) -> Option<Box<dyn GroupExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #as_group(self.inner);
                    if ptr.is_null() {
                        return None;
                    }
                    Some(Box::new(crate::group::Group::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)))
                }
            }

            fn below_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "below_of requires the size of the widget to be known!");
                self.resize(w.x(), w.y() + w.height() + padding, self.width(), self.height());
                self
            }

            fn above_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "above_of requires the size of the widget to be known!");
                self.resize(w.x(), w.y() - padding - self.height(), self.width(), self.height());
                self
            }

            fn right_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "right_of requires the size of the widget to be known!");
                self.resize(w.x() + w.width() + padding, w.y(), self.width(), self.height());
                self
            }

            fn left_of<W: WidgetExt>(mut self, w: &W, padding: i32) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "left_of requires the size of the widget to be known!");
                self.resize(w.x() - self.width() - padding, w.y(), self.width(), self.height());
                self
            }

            fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(w.width() != 0 && w.height() != 0, "center_of requires the size of the widget to be known!");
                let sw = self.width() as f64;
                let sh = self.height() as f64;
                let ww = w.width() as f64;
                let wh = w.height() as f64;
                let sx = (ww - sw) / 2.0;
                let sy = (wh - sh) / 2.0;
                let wx = if w.as_window().is_some() { 0 } else { w.x() };
                let wy = if w.as_window().is_some() { 0 } else { w.y() };
                self.resize(sx as i32 + wx, sy as i32 + wy, self.width(), self.height());
                self.redraw();
                self
            }

            fn center_of_parent(mut self) -> Self {
                assert!(!self.was_deleted());
                if let Some(w) = self.parent() {
                    debug_assert!(w.width() != 0 && w.height() != 0, "center_of requires the size of the widget to be known!");
                    let sw = self.width() as f64;
                    let sh = self.height() as f64;
                    let ww = w.width() as f64;
                    let wh = w.height() as f64;
                    let sx = (ww - sw) / 2.0;
                    let sy = (wh - sh) / 2.0;
                    let wx = if w.as_window().is_some() { 0 } else { w.x() };
                    let wy = if w.as_window().is_some() { 0 } else { w.y() };
                    self.resize(sx as i32 + wx, sy as i32 + wy, self.width(), self.height());
                    self.redraw();
                }
                self
            }

            fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(w.width() != 0 && w.height() != 0, "size_of requires the size of the widget to be known!");
                self.resize(self.x(), self. y(), w.width(), w.height());
                self
            }

            fn size_of_parent(mut self) -> Self {
                assert!(!self.was_deleted());
                if let Some(parent) = self.parent() {
                    let w = parent.width();
                    let h = parent.height();
                    self.resize(self.x(), self.y(), w, h);
                }
                self
            }

            fn inside<W: WidgetExt>(&self, wid: &W) -> bool {
                assert!(!self.was_deleted());
                assert!(!wid.was_deleted());
                unsafe {
                    #inside(self.inner, wid.as_widget_ptr() as *mut raw::c_void)  != 0
                }
            }

            fn with_pos(mut self, x: i32, y: i32) -> Self {
                self.resize(x, y, self.width(), self.height());
                self
            }

            fn with_size(mut self, width: i32, height: i32) -> Self {
                let x = self.x();
                let y = self.y();
                let w = self.width();
                let h = self.height();
                if w == 0 || h == 0 {
                    unsafe { #widget_resize(self.inner, x, y, width, height); }
                } else {
                    self.resize(x, y, width, height);
                }
                self
            }

            fn with_label(mut self, title: &str) -> Self {
                self.set_label(title);
                self
            }

            fn with_align(mut self, align: Align) -> Self {
                self.set_align(align);
                self
            }

            fn get_type<T: WidgetType>(&self) -> T {
                assert!(!self.was_deleted());
                unsafe { T::from_i32(#get_type(self.inner)) }
            }

            fn set_type<T: WidgetType>(&mut self, typ: T) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_type(self.inner, typ.to_i32());
                }
            }

            fn set_image<I: ImageExt>(&mut self, image: Option<I>) {
                assert!(!self.was_deleted());
                if let Some(image) = image {
                    assert!(!image.was_deleted());
                    unsafe { #set_image(self.inner, image.as_image_ptr() as *mut _) }
                } else {
                    unsafe { #set_image(self.inner, std::ptr::null_mut() as *mut raw::c_void) }
                }
            }

            fn image(&self) -> Option<Box<dyn ImageExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let image_ptr = #image(self.inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        let mut img = Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                        Some(Box::new(img))
                    }
                }
            }

            fn set_deimage<I: ImageExt>(&mut self, image: Option<I>) {
                assert!(!self.was_deleted());
                if let Some(image) = image {
                    assert!(!image.was_deleted());
                    unsafe { #set_deimage(self.inner, image.as_image_ptr() as *mut _) }
                } else {
                    unsafe { #set_deimage(self.inner, std::ptr::null_mut() as *mut raw::c_void) }
                }
            }

            fn deimage(&self) -> Option<Box<dyn ImageExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let image_ptr = #deimage(self.inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        let mut img = Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                        Some(Box::new(img))
                    }
                }
            }

            fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
                assert!(!self.was_deleted());
                unsafe {
                    unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                        let mut wid = #name::from_widget_ptr(wid as *mut _);
                        let a = data as *mut Box<dyn FnMut(&mut #name)>;
                        let f: &mut (dyn FnMut(&mut #name)) = &mut **a;
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                    }
                    let _old_data = self.user_data();
                    let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                    let data: *mut raw::c_void = a as *mut raw::c_void;
                    let callback: Fl_Callback = Some(shim);
                    #set_callback(self.inner, callback, data);
                }
            }

            fn emit<T: 'static + Clone + Send + Sync>(&mut self, sender: crate::app::Sender<T>, msg: T) {
                assert!(!self.was_deleted());
                self.set_callback(move |_| sender.send(msg.clone()))
            }

            unsafe fn into_widget<W: WidgetBase>(&self) -> W where Self: Sized {
                W::from_widget_ptr(self.as_widget_ptr() as *mut _)
            }

            fn visible(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #visible(self.inner) != 0
                }
            }

            fn visible_r(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #visible_r(self.inner) != 0
                }
            }
        }
    };
    gen.into()
}

pub fn impl_widget_type(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl WidgetType for #name {
            fn to_i32(self) -> i32 {
                self as i32
            }

            fn from_i32(val: i32) -> #name {
                unsafe { mem::transmute(val) }
            }
        }
    };
    gen.into()
}
