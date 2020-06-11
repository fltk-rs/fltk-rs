use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_widget_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = get_fl_name(name.to_string());
    let ptr_name = Ident::new(format!("{}", name_str).as_str(), name.span());
    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let delete = Ident::new(format!("{}_{}", name_str, "delete").as_str(), name.span());
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
    let user_data = Ident::new(
        format!("{}_{}", name_str, "user_data").as_str(),
        name.span(),
    );
    let handle_data = Ident::new(
        format!("{}_{}", name_str, "handle_data").as_str(),
        name.span(),
    );
    let draw_data = Ident::new(
        format!("{}_{}", name_str, "draw_data").as_str(),
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
    let set_user_data = Ident::new(
        format!("{}_{}", name_str, "set_user_data").as_str(),
        name.span(),
    );
    let set_draw_data = Ident::new(
        format!("{}_{}", name_str, "set_draw_data").as_str(),
        name.span(),
    );
    let set_handle_data = Ident::new(
        format!("{}_{}", name_str, "set_handle_data").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl Send for #name {}
        unsafe impl Sync for #name {}

        impl Clone for #name {
            fn clone(&self) -> #name {
                #name { _inner: self._inner, _tracker: self._tracker }
            }
        }

        unsafe impl WidgetExt for #name {
            fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> #name {
                let temp = CString::new(title).unwrap();
                unsafe {
                    let widget_ptr = #new(x, y, width, height, temp.into_raw() as *const raw::c_char);
                    assert!(!widget_ptr.is_null());
                    let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(widget_ptr as *mut fltk_sys::fl::Fl_Widget);
                    assert!(!tracker.is_null());
                    #name {
                        _inner: widget_ptr,
                        _tracker: tracker,
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
                        assert!(!widget_ptr.is_null());
                        let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(widget_ptr as *mut fltk_sys::fl::Fl_Widget);
                        assert!(!tracker.is_null());
                    #name {
                        _inner: widget_ptr,
                        _tracker: tracker,
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

            fn with_align(mut self, align: Align) -> Self {
                self.set_align(align);
                self
            }

            fn set_label(&mut self, title: &str) {
                assert!(!self.was_deleted());
                let temp = CString::new(title).unwrap();
                unsafe {
                    #set_label(
                        self._inner,
                        temp.as_ptr(),
                    )
                }
            }

            fn redraw(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #redraw(self._inner);
                }
            }

            fn show(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #show(self._inner) }
            }

            fn hide(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #hide(self._inner) }
            }

            fn x(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #x(self._inner)}
            }

            fn y(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #y(self._inner) }
            }

            fn width(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #width(self._inner) }
            }

            fn height(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #height(self._inner) }
            }

            fn label(&self) -> String {
                assert!(!self.was_deleted());
                unsafe {
                    CStr::from_ptr(#label(self._inner) as *mut raw::c_char).to_string_lossy().to_string()
                }
            }

            unsafe fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
                unsafe { mem::transmute(self._inner) }
            }

            unsafe fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self {
                assert!(!ptr.is_null());
                unsafe {
                    let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(ptr as *mut fltk_sys::fl::Fl_Widget);
                    assert!(!tracker.is_null());
                    #name {
                        _inner: mem::transmute(ptr),
                        _tracker: tracker,
                    }
                }
            }

            fn activate(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #activate(self._inner) }
            }

            fn deactivate(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #deactivate(self._inner) }
            }

            fn redraw_label(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #redraw_label(self._inner) }
            }

            fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                assert!(!self.was_deleted());
                unsafe { #resize(self._inner, x, y, width, height) }
            }

            fn tooltip(&self) -> Option<String> {
                assert!(!self.was_deleted());
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
                assert!(!self.was_deleted());
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #set_tooltip(
                        self._inner,
                        txt.as_ptr() as *mut raw::c_char,
                    )
                }
            }

            fn get_type<T: WidgetType>(&self) -> T {
                assert!(!self.was_deleted());
                unsafe { T::from_i32(#get_type(self._inner)) }
            }

            fn set_type<T: WidgetType>(&mut self, typ: T) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_type(self._inner, typ.to_int());
                }
            }

            fn color(&self) -> Color {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#color(self._inner)) }
            }

            fn set_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe { #set_color(self._inner, color as u32) }
            }

            fn label_color(&self) -> Color {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_color(self._inner)) }
            }

            fn set_label_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe { #set_label_color(self._inner, color as u32) }
            }

            fn label_font(&self) -> Font {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_font(self._inner)) }
            }

            fn set_label_font(&mut self, font: Font) {
                assert!(!self.was_deleted());
                unsafe { #set_label_font(self._inner, font as i32) }
            }

            fn label_size(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe { #label_size(self._inner) }
            }

            fn set_label_size(&mut self, sz: i32) {
                assert!(!self.was_deleted());
                unsafe { #set_label_size(self._inner, sz) }
            }

            fn label_type(&self) -> LabelType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#label_type(self._inner)) }
            }

            fn set_label_type(&mut self, typ: LabelType) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_label_type(self._inner, typ as i32);
                }
            }

            fn frame(&self) -> FrameType {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#frame(self._inner)) }
            }

            fn set_frame(&mut self, typ: FrameType) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_frame(self._inner, typ as i32);
                }
            }

            fn changed(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    match #changed(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_changed(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #set_changed(self._inner) }
            }

            fn clear_changed(&mut self) {
                assert!(!self.was_deleted());
                unsafe {#clear_changed(self._inner) }
            }

            fn align(&self) -> Align {
                assert!(!self.was_deleted());
                unsafe { mem::transmute(#align(self._inner)) }
            }

            fn set_align(&mut self, align: Align) {
                assert!(!self.was_deleted());
                unsafe { #set_align(self._inner, align as i32) }
            }

            fn set_image<Image: ImageExt>(&mut self, image: &Image) {
                assert!(!self.was_deleted());
                unsafe { #set_image(self._inner, image.as_ptr()) }
            }

            fn set_image_with_size<Image: ImageExt>(&mut self, image: &Image, w: i32, h: i32) {
                assert!(!self.was_deleted());
                unsafe { #set_image_with_size(self._inner, image.as_ptr(), w, h) }
            }

            fn image(&self) -> Option<Image> {
                assert!(!self.was_deleted());
                unsafe {
                    let image_ptr = #image(self._inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        Some(Image::from_raw(image_ptr as *mut fltk_sys::image::Fl_Image))
                    }
                }
            }

            fn set_callback(&mut self, cb: Box<dyn FnMut()>) {
                assert!(!self.was_deleted());
                // debug_assert!(
                //     self.top_window().unwrap().takes_events() && self.takes_events(),
                //     "Handling events requires that the window and widget be active!"
                // );
                unsafe {
                    unsafe extern "C" fn shim(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    self.unset_callback();
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: fltk_sys::widget::Fl_Callback = Some(shim);
                    fltk_sys::widget::Fl_Widget_callback_with_captures(self.as_widget_ptr(), callback, data);
                }
            }

            unsafe fn unset_callback(&mut self) {
                unsafe {
                    let old_data = self.user_data();
                    if old_data.is_some() {
                        self.set_user_data(0 as *mut raw::c_void);
                        let old_data = old_data.unwrap();
                    }
                    // let callback: Option<unsafe extern "C" fn(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void)> = None;
                    // fltk_sys::widget::Fl_Widget_callback_with_captures(self.as_widget_ptr(), callback, std::ptr::null_mut());
                    // self.set_callback(Box::new(move || {/* do nothing! */} ));
                }
            }

            fn handle(&mut self, cb: Box<dyn FnMut(Event) -> bool>) {
                assert!(!self.was_deleted());
                // debug_assert!(
                //     self.top_window().unwrap().takes_events() && self.takes_events(),
                //     "Handling events requires that the window and widget be active!"
                // );
                unsafe {
                    unsafe extern "C" fn shim(_ev: std::os::raw::c_int, data: *mut raw::c_void) -> i32 {
                        let ev: Event = mem::transmute(_ev);
                        let a: *mut Box<dyn FnMut(Event) -> bool> = mem::transmute(data);
                        let f: &mut (dyn FnMut(Event) -> bool) = &mut **a;
                        match f(ev) {
                            true => return 1,
                            false => return 0,
                        }
                    }
                    let a: *mut Box<dyn FnMut(Event) -> bool> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: custom_handler_callback = Some(shim);
                    #set_handler(self._inner, callback, data);
                }
            }

            fn draw(&mut self, cb: Box<dyn FnMut()>) {
                assert!(!self.was_deleted());
                // debug_assert!(
                //     self.top_window().unwrap().takes_events() && self.takes_events(),
                //     "Handling events requires that the window and widget be active!"
                // );
                unsafe {
                    unsafe extern "C" fn shim(data: *mut raw::c_void) {
                        let a: *mut Box<dyn FnMut()> = mem::transmute(data);
                        let f: &mut (dyn FnMut()) = &mut **a;
                        f();
                    }
                    self.unset_draw_callback();
                    let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
                    let data: *mut raw::c_void = mem::transmute(a);
                    let callback: custom_draw_callback = Some(shim);
                    #set_draw(self._inner, callback, data);
                }
            }

            fn set_trigger(&mut self, trigger: CallbackTrigger) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_trigger(self._inner, trigger as i32)
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
                self.resize(w.x() + self.width() + padding, w.y(), self.width(), self.height());
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
                assert!(!w.was_deleted());
                assert!(!self.was_deleted());
                debug_assert!(w.width() != 0 && w.height() != 0, "size_of requires the size of the widget to be known!");
                self.resize(self.x(), self. y(), w.width(), w.height());
                self
            }

            fn parent(&self) -> Option<crate::widget::Widget> {
                assert!(!self.was_deleted());
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
                assert!(!self.was_deleted());
                unsafe {
                    mem::transmute(#selection_color(self._inner))
                }
            }

            fn set_selection_color(&mut self, color: Color) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_selection_color(self._inner, color as u32);
                }
            }

            fn do_callback(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #do_callback(self._inner);
                }
            }

            fn inside(&self, wid: crate::widget::Widget) -> bool {
                assert!(!self.was_deleted());
                assert!(!wid.was_deleted());
                unsafe {
                    match #inside(self._inner, wid.as_ptr() as *mut raw::c_void) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn window(&self) -> Option<crate::window::Window> {
                assert!(!self.was_deleted());
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
                assert!(!self.was_deleted());
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
                assert!(!self.was_deleted());
                unsafe {
                    match #takes_events(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn emit<T: 'static + Copy + Send + Sync>(&mut self, sender: crate::app::Sender<T>, msg: T) {
                assert!(!self.was_deleted());
                self.set_callback(Box::new(move || sender.send(msg)))
            }

            unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
                let ptr = #user_data(self._inner);
                if ptr.is_null() {
                    None
                } else {
                    let x = ptr as *mut Box<dyn FnMut()>;
                    let x = Box::from_raw(x);
                    Some(*x)
                }
            }

            unsafe fn set_user_data(&mut self, data: *mut raw::c_void) {
                unsafe { #set_user_data(self._inner, data) }
            }

            unsafe fn raw_user_data(&self) -> *mut raw::c_void {
                #user_data(self._inner)
            }

            fn delete(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    crate::app::delete_widget(self);
                }
            }

            fn take_focus(&mut self) -> Result<(), FltkError> {
                assert!(!self.was_deleted());
                unsafe {
                    match #take_focus(self._inner) {
                        0 => Ok(()),
                        _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                    }
                }
            }


            fn set_visible_focus(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_visible_focus(self._inner)
                }
            }


            fn clear_visible_focus(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #clear_visible_focus(self._inner)
                }
            }


            fn visible_focus(&mut self, v: bool) {
                assert!(!self.was_deleted());
                unsafe {
                    #visible_focus(self._inner, v as i32)
                }
            }


            fn has_visible_focus(&mut self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #has_visible_focus(self._inner) != 0
                }
            }

            fn was_deleted(&self) -> bool {
                unsafe {
                    if self._inner.is_null() || self._tracker.is_null() {
                        return true;
                    } else {
                        return fltk_sys::fl::Fl_Widget_Tracker_deleted(self._tracker) != 0;
                    }
                }
            }

            unsafe fn cleanup(&mut self) {
                self._inner = std::ptr::null_mut() as *mut #ptr_name;
                fltk_sys::fl::Fl_Widget_Tracker_delete(self._tracker);
                self._tracker = std::ptr::null_mut() as *mut fltk_sys::fl::Fl_Widget_Tracker;
            }

            unsafe fn draw_data(&mut self) -> Option<Box<dyn FnMut()>> {
                unsafe {
                    let ptr = #draw_data(self._inner);
                    if ptr.is_null() {
                        return None;
                    }
                    let data = ptr as *mut Box<dyn FnMut()>;
                    let data = Box::from_raw(data);
                    Some(*data)
                }
            }

            unsafe fn set_draw_data(&mut self, data: *mut raw::c_void) {
                unsafe {
                    #set_draw_data(self._inner, data);
                }
            }
            
            unsafe fn unset_draw_callback(&mut self) {
                unsafe {
                    let old_data = self.draw_data();
                    if old_data.is_some() {
                        let old_data = old_data.unwrap();
                        self.set_draw_data(0 as *mut raw::c_void);
                    }
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
