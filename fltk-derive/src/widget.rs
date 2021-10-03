#[macro_export]
macro_rules! impl_widget_ext {
    ($name: ident, $flname: ident) => {
        #[cfg(not(feature = "single-threaded"))]
        unsafe impl Send for $name {}
        #[cfg(not(feature = "single-threaded"))]
        unsafe impl Sync for $name {}

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }
        impl Eq for $name {}
        impl Clone for $name {
            fn clone(&self) -> $name {
                assert!(!self.was_deleted());
                $name {
                    inner: self.inner,
                    tracker: self.tracker,
                    is_derived: self.is_derived,
                }
            }
        }

        paste! {
            unsafe impl WidgetExt for $name {
                fn with_pos(mut self, x: i32, y: i32) -> Self {
                    let w = self.w();
                    let h = self.h();
                    self.resize(x, y, w, h);
                    self
                }
    
                fn with_size(mut self, width: i32, height: i32) -> Self {
                    let x = self.x();
                    let y = self.y();
                    let w = self.width();
                    let h = self.height();
                    if w == 0 || h == 0 {
                        self.widget_resize(x, y, width, height);
                    } else {
                        self.resize(x, y, width, height);
                    }
                    self
                }
    
                fn with_label(mut self, title: &str) -> Self {
                    self.set_label(title);
                    self
                }
    
                fn with_align(mut self, align: crate::enums::Align) -> Self {
                    self.set_align(align);
                    self
                }
    
                fn with_type<T: WidgetType>(mut self, typ: T) -> Self {
                    assert!(!self.was_deleted());
                    self.set_type(typ);
                    self
                }
    
                fn below_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                    assert!(!wid.was_deleted());
                    assert!(!self.was_deleted());
                    let w = self.w();
                    let h = self.h();
                    debug_assert!(
                        w != 0 && h != 0,
                        "below_of requires the size of the widget to be known!"
                    );
                    self.resize(wid.x(), wid.y() + wid.h() + padding, w, h);
                    self
                }
    
                fn above_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                    assert!(!wid.was_deleted());
                    assert!(!self.was_deleted());
                    let w = self.w();
                    let h = self.h();
                    debug_assert!(
                        w != 0 && h != 0,
                        "above_of requires the size of the widget to be known!"
                    );
                    self.resize(wid.x(), wid.y() - padding - h, w, h);
                    self
                }
    
                fn right_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                    assert!(!wid.was_deleted());
                    assert!(!self.was_deleted());
                    let w = self.w();
                    let h = self.h();
                    debug_assert!(
                        w != 0 && h != 0,
                        "right_of requires the size of the widget to be known!"
                    );
                    self.resize(wid.x() + wid.width() + padding, wid.y(), w, h);
                    self
                }
    
                fn left_of<W: WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                    assert!(!wid.was_deleted());
                    assert!(!self.was_deleted());
                    let w = self.w();
                    let h = self.h();
                    debug_assert!(
                        w != 0 && h != 0,
                        "left_of requires the size of the widget to be known!"
                    );
                    self.resize(wid.x() - w - padding, wid.y(), w, h);
                    self
                }
    
                fn center_of<W: WidgetExt>(mut self, w: &W) -> Self {
                    assert!(!w.was_deleted());
                    assert!(!self.was_deleted());
                    debug_assert!(
                        w.width() != 0 && w.height() != 0,
                        "center_of requires the size of the widget to be known!"
                    );
                    let sw = self.width() as f64;
                    let sh = self.height() as f64;
                    let ww = w.width() as f64;
                    let wh = w.height() as f64;
                    let sx = (ww - sw) / 2.0;
                    let sy = (wh - sh) / 2.0;
                    let wx = if w.as_window().is_some() { 0 } else { w.x() };
                    let wy = if w.as_window().is_some() { 0 } else { w.y() };
                    self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                    self.redraw();
                    self
                }
    
                fn center_of_parent(mut self) -> Self {
                    assert!(!self.was_deleted());
                    if let Some(w) = self.parent() {
                        debug_assert!(
                            w.width() != 0 && w.height() != 0,
                            "center_of requires the size of the widget to be known!"
                        );
                        let sw = self.width() as f64;
                        let sh = self.height() as f64;
                        let ww = w.width() as f64;
                        let wh = w.height() as f64;
                        let sx = (ww - sw) / 2.0;
                        let sy = (wh - sh) / 2.0;
                        let wx = if w.as_window().is_some() { 0 } else { w.x() };
                        let wy = if w.as_window().is_some() { 0 } else { w.y() };
                        self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                        self.redraw();
                    }
                    self
                }
    
                fn size_of<W: WidgetExt>(mut self, w: &W) -> Self {
                    assert!(!w.was_deleted());
                    assert!(!self.was_deleted());
                    debug_assert!(
                        w.width() != 0 && w.height() != 0,
                        "size_of requires the size of the widget to be known!"
                    );
                    let x = self.x();
                    let y = self.y();
                    self.resize(x, y, w.width(), w.height());
                    self
                }
    
                fn size_of_parent(mut self) -> Self {
                    assert!(!self.was_deleted());
                    if let Some(parent) = self.parent() {
                        let w = parent.width();
                        let h = parent.height();
                        let x = self.x();
                        let y = self.y();
                        self.resize(x, y, w, h);
                    }
                    self
                }
    
                fn set_pos(&mut self, x: i32, y: i32) {
                    self.resize(x, y, self.width(), self.height());
                }
    
                fn set_size(&mut self, width: i32, height: i32) {
                    self.resize(self.x(), self.y(), width, height);
                }
    
                fn set_label(&mut self, title: &str) {
                    assert!(!self.was_deleted());
                    unsafe {
                        let temp = CString::safe_new(title);
                        [<$flname _set_label>](self.inner, temp.as_ptr());
                    }
                }
    
                fn redraw(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _redraw>](self.inner);
                    }
                }
    
                fn show(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _show>](self.inner) }
                }
    
                fn hide(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _hide>](self.inner) }
                }
    
                fn x(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _x>](self.inner) }
                }
    
                fn y(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _y>](self.inner) }
                }
    
                fn width(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _width>](self.inner) }
                }
    
                fn height(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _height>](self.inner) }
                }
    
                fn w(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _width>](self.inner) }
                }
    
                fn h(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _height>](self.inner) }
                }
    
                fn label(&self) -> String {
                    assert!(!self.was_deleted());
                    unsafe {
                        fltk_sys::fl::Fl_lock();
                        let ptr = [<$flname _label>](self.inner) as *mut raw::c_char;
                        let s = if ptr.is_null() {
                            String::from("")
                        } else {
                            CStr::from_ptr(ptr).to_string_lossy().to_string()
                        };
                        fltk_sys::fl::Fl_unlock();
                        s
                    }
                }
    
                fn measure_label(&self) -> (i32, i32) {
                    assert!(!self.was_deleted());
                    let mut x = 0;
                    let mut y = 0;
                    unsafe {
                        [<$flname _measure_label>](self.inner, &mut x, &mut y);
                    }
                    (x, y)
                }
    
                unsafe fn as_widget_ptr(&self) -> *mut fltk_sys::widget::Fl_Widget {
                    self.inner as *mut fltk_sys::widget::Fl_Widget
                }
    
                fn activate(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _activate>](self.inner) }
                }
    
                fn deactivate(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _deactivate>](self.inner) }
                }
    
                fn redraw_label(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _redraw_label>](self.inner) }
                }
    
                fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _resize>](self.inner, x, y, width, height) }
                }
    
                fn widget_resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _widget_resize>](self.inner, x, y, width, height) }
                }
    
                fn tooltip(&self) -> Option<String> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let tooltip_ptr = [<$flname _tooltip>](self.inner);
                        fltk_sys::fl::Fl_lock();
                        let s = if tooltip_ptr.is_null() {
                            None
                        } else {
                            Some(
                                CStr::from_ptr(tooltip_ptr as *mut raw::c_char)
                                    .to_string_lossy()
                                    .to_string(),
                            )
                        };
                        fltk_sys::fl::Fl_unlock();
                        s
                    }
                }
    
                fn set_tooltip(&mut self, txt: &str) {
                    assert!(!self.was_deleted());
                    let txt = CString::safe_new(txt);
                    unsafe {
                        [<$flname _set_tooltip>](
                            self.inner,
                            txt.as_ptr() as *mut raw::c_char,
                        )
                    }
                }
    
                fn color(&self) -> Color {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _color>](self.inner)) }
                }
    
                fn set_color(&mut self, color: Color) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_color>](self.inner, color.bits() as u32) }
                }
    
                fn label_color(&self) -> Color {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _label_color>](self.inner)) }
                }
    
                fn set_label_color(&mut self, color: Color) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _set_label_color>](self.inner, color.bits() as u32)
                    }
                }
    
                fn label_font(&self) -> Font {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _label_font>](self.inner)) }
                }
    
                fn set_label_font(&mut self, font: Font) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_label_font>](self.inner, font.bits() as i32) }
                }
    
                fn label_size(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _label_size>](self.inner) }
                }
    
                fn set_label_size(&mut self, sz: i32) {
                    assert!(!self.was_deleted());
                    let sz = if sz < 1 { 1 } else { sz };
                    unsafe { [<$flname _set_label_size>](self.inner, sz) }
                }
    
                fn label_type(&self) -> LabelType {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _label_type>](self.inner)) }
                }
    
                fn set_label_type(&mut self, typ: LabelType) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _set_label_type>](self.inner, typ as i32);
                    }
                }
    
                fn frame(&self) -> FrameType {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _box>](self.inner)) }
                }
    
                fn set_frame(&mut self, typ: FrameType) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _set_box>](self.inner, typ as i32);
                    }
                }
    
                fn changed(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _changed>](self.inner) != 0 }
                }
    
                fn set_changed(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_changed>](self.inner) }
                }
    
                fn clear_changed(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _clear_changed>](self.inner) }
                }
    
                fn align(&self) -> Align {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _align>](self.inner)) }
                }
    
                fn set_align(&mut self, align: Align) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_align>](self.inner, align.bits() as i32) }
                }
    
                fn set_trigger(&mut self, trigger: CallbackTrigger) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_when>](self.inner, trigger.bits() as i32) }
                }
    
                fn trigger(&self) -> CallbackTrigger {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _when>](self.inner)) }
                }
    
                fn parent(&self) -> Option<crate::group::Group> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let x = [<$flname _parent>](self.inner);
                        if x.is_null() {
                            None
                        } else {
                            Some(crate::group::Group::from_widget_ptr(x as *mut _))
                        }
                    }
                }
    
                fn selection_color(&mut self) -> Color {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _selection_color>](self.inner)) }
                }
    
                fn set_selection_color(&mut self, color: Color) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _set_selection_color>](self.inner, color.bits() as u32);
                    }
                }
    
                fn do_callback(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _do_callback>](self.inner);
                    }
                }
    
                fn window(&self) -> Option<Box<dyn WindowExt>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let wind_ptr = [<$flname _window>](self.inner);
                        if wind_ptr.is_null() {
                            None
                        } else {
                            Some(Box::new(crate::window::Window::from_widget_ptr(
                                wind_ptr as *mut fltk_sys::widget::Fl_Widget,
                            )))
                        }
                    }
                }
    
                fn top_window(&self) -> Option<Box<dyn WindowExt>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let wind_ptr = [<$flname _top_window>](self.inner);
                        if wind_ptr.is_null() {
                            None
                        } else {
                            Some(Box::new(crate::window::Window::from_widget_ptr(
                                wind_ptr as *mut fltk_sys::widget::Fl_Widget,
                            )))
                        }
                    }
                }
    
                fn takes_events(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _takes_events>](self.inner) != 0 }
                }
    
                unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
                    let ptr = [<$flname _user_data>](self.inner);
                    if ptr.is_null() {
                        None
                    } else {
                        let x = ptr as *mut Box<dyn FnMut()>;
                        let x = Box::from_raw(x);
                        [<$flname _set_callback>](self.inner, None, std::ptr::null_mut());
                        Some(*x)
                    }
                }
    
                fn take_focus(&mut self) -> Result<(), FltkError> {
                    assert!(!self.was_deleted());
                    unsafe {
                        match [<$flname _take_focus>](self.inner) {
                            0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
                            _ => Ok(()),
                        }
                    }
                }
    
                fn set_visible_focus(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_visible_focus>](self.inner) }
                }
    
                fn clear_visible_focus(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _clear_visible_focus>](self.inner) }
                }
    
                fn visible_focus(&mut self, v: bool) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _visible_focus>](self.inner, v as i32) }
                }
    
                fn has_visible_focus(&mut self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _has_visible_focus>](self.inner) != 0 }
                }
    
                fn has_focus(&mut self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { fltk_sys::fl::Fl_focus() == self.inner as _ }
                }
    
                fn was_deleted(&self) -> bool {
                    unsafe {
                        self.inner.is_null()
                            || self.tracker.is_null()
                            || fltk_sys::fl::Fl_Widget_Tracker_deleted(self.tracker) != 0
                    }
                }
    
                fn damage(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _damage>](self.inner) != 0 }
                }
    
                fn set_damage(&mut self, flag: bool) {
                    assert!(!self.was_deleted());
                    let flag = if flag { 10 } else { 0 };
                    unsafe { [<$flname _set_damage>](self.inner, flag) }
                }
    
                fn damage_type(&self) -> Damage {
                    assert!(!self.was_deleted());
                    unsafe { mem::transmute([<$flname _damage>](self.inner)) }
                }
    
                fn set_damage_type(&mut self, mask: Damage) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_damage>](self.inner, mask.bits()) }
                }
    
                fn clear_damage(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _clear_damage>](self.inner) }
                }
    
                fn as_window(&self) -> Option<Box<dyn WindowExt>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _as_window>](self.inner);
                        if ptr.is_null() {
                            return None;
                        }
                        Some(Box::new(crate::window::Window::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )))
                    }
                }
    
                fn as_group(&self) -> Option<crate::group::Group> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _as_group>](self.inner);
                        if ptr.is_null() {
                            return None;
                        }
                        Some(crate::group::Group::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        ))
                    }
                }
    
                fn inside<W: WidgetExt>(&self, wid: &W) -> bool {
                    assert!(!self.was_deleted());
                    assert!(!wid.was_deleted());
                    unsafe {
                        [<$flname _inside>](
                            self.inner,
                            wid.as_widget_ptr() as *mut raw::c_void,
                        ) != 0
                    }
                }
    
                fn get_type<T: WidgetType>(&self) -> T {
                    assert!(!self.was_deleted());
                    unsafe { T::from_i32([<$flname _get_type>](self.inner)) }
                }
    
                fn set_type<T: WidgetType>(&mut self, typ: T) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _set_type>](self.inner, typ.to_i32());
                    }
                }
    
                fn set_image<I: ImageExt>(&mut self, image: Option<I>) {
                    assert!(!self.was_deleted());
                    if let Some(image) = image {
                        assert!(!image.was_deleted());
                        unsafe {
                            [<$flname _set_image>](
                                self.inner,
                                image.as_image_ptr() as *mut _,
                            )
                        }
                    } else {
                        unsafe {
                            [<$flname _set_image>](
                                self.inner,
                                std::ptr::null_mut() as *mut raw::c_void,
                            )
                        }
                    }
                }
    
                fn set_image_scaled<I: ImageExt>(&mut self, image: Option<I>) {
                    assert!(!self.was_deleted());
                    if let Some(mut image) = image {
                        assert!(!image.was_deleted());
                        image.scale(self.w(), self.h(), false, true);
                        unsafe {
                            [<$flname _set_image>](
                                self.inner,
                                image.as_image_ptr() as *mut _,
                            )
                        }
                    } else {
                        unsafe {
                            [<$flname _set_image>](
                                self.inner,
                                std::ptr::null_mut() as *mut raw::c_void,
                            )
                        }
                    }
                }
    
                fn image(&self) -> Option<Box<dyn ImageExt>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let image_ptr = [<$flname _image>](self.inner);
                        if image_ptr.is_null() {
                            None
                        } else {
                            let mut img =
                                Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                            Some(Box::new(img))
                        }
                    }
                }
    
                fn set_deimage<I: ImageExt>(&mut self, image: Option<I>) {
                    assert!(!self.was_deleted());
                    if let Some(image) = image {
                        assert!(!image.was_deleted());
                        unsafe {
                            [<$flname _set_deimage>](
                                self.inner,
                                image.as_image_ptr() as *mut _,
                            )
                        }
                    } else {
                        unsafe {
                            [<$flname _set_deimage>](
                                self.inner,
                                std::ptr::null_mut() as *mut raw::c_void,
                            )
                        }
                    }
                }
    
                fn set_deimage_scaled<I: ImageExt>(&mut self, image: Option<I>) {
                    assert!(!self.was_deleted());
                    if let Some(mut image) = image {
                        assert!(!image.was_deleted());
                        image.scale(self.w(), self.h(), false, true);
                        unsafe {
                            [<$flname _set_deimage>](
                                self.inner,
                                image.as_image_ptr() as *mut _,
                            )
                        }
                    } else {
                        unsafe {
                            [<$flname _set_deimage>](
                                self.inner,
                                std::ptr::null_mut() as *mut raw::c_void,
                            )
                        }
                    }
                }
    
                fn deimage(&self) -> Option<Box<dyn ImageExt>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let image_ptr = [<$flname _deimage>](self.inner);
                        if image_ptr.is_null() {
                            None
                        } else {
                            let mut img =
                                Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                            Some(Box::new(img))
                        }
                    }
                }
    
                fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
                    assert!(!self.was_deleted());
                    unsafe {
                        unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let a = data as *mut Box<dyn FnMut(&mut $name)>;
                            let f: &mut (dyn FnMut(&mut $name)) = &mut **a;
                            let _ =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                        }
                        let _old_data = self.user_data();
                        let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut raw::c_void = a as *mut raw::c_void;
                        let callback: Fl_Callback = Some(shim);
                        [<$flname _set_callback>](self.inner, callback, data);
                    }
                }
    
                fn emit<T: 'static + Clone + Send + Sync>(
                    &mut self,
                    sender: crate::app::Sender<T>,
                    msg: T,
                ) {
                    assert!(!self.was_deleted());
                    self.set_callback(move |_| sender.send(msg.clone()))
                }
    
                unsafe fn into_widget<W: WidgetBase>(&self) -> W {
                    W::from_widget_ptr(self.as_widget_ptr() as *mut _)
                }
    
                fn visible(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _visible>](self.inner) != 0 }
                }
    
                fn visible_r(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _visible_r>](self.inner) != 0 }
                }
    
                fn is_same<W: WidgetExt>(&self, other: &W) -> bool {
                    unsafe { self.as_widget_ptr() == other.as_widget_ptr() }
                }
    
                fn active(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _active>](self.inner) != 0 }
                }
    
                fn active_r(&self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _active_r>](self.inner) != 0 }
                }
    
                fn callback(&self) -> Option<Box<dyn FnMut()>> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let cb = [<$flname _callback>](self.inner);
                        let data = [<$flname _user_data>](self.inner);
                        if !data.is_null() {
                            return None;
                        }
                        let s = self.clone();
                        if let Some(cb) = cb {
                            let cb_1 = Box::new(move || {
                                cb(s.as_widget_ptr() as _, std::ptr::null_mut());
                            });
                            Some(cb_1)
                        } else {
                            None
                        }
                    }
                }
    
                fn handle_event(&mut self, event: Event) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _handle_event>](self.inner, event.bits()) }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_widget_base {
    ($name: ident, $flname: ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self::new(0, 0, 0, 0, None)
            }
        }

        paste! {
            unsafe impl WidgetBase for $name {
                fn new<T: Into<Option<&'static str>>>(
                    x: i32,
                    y: i32,
                    width: i32,
                    height: i32,
                    title: T,
                ) -> $name {
                    let temp = if let Some(title) = title.into() {
                        CString::safe_new(title).into_raw()
                    } else {
                        std::ptr::null_mut()
                    };
                    unsafe {
                        let widget_ptr = [<$flname _new>](x, y, width, height, temp);
                        assert!(!widget_ptr.is_null());
                        let tracker = fltk_sys::fl::Fl_Widget_Tracker_new(
                            widget_ptr as *mut fltk_sys::fl::Fl_Widget,
                        );
                        assert!(!tracker.is_null());
                        unsafe extern "C" fn shim(data: *mut raw::c_void) {
                            if !data.is_null() {
                                let x = data as *mut Box<dyn FnMut()>;
                                let _x = Box::from_raw(x);
                            }
                        }
                        [<$flname _set_deleter>](widget_ptr, Some(shim));
                        $name {
                            inner: widget_ptr,
                            tracker: tracker,
                            is_derived: true,
                        }
                    }
                }
    
                fn default_fill() -> Self {
                    Self::default().size_of_parent().center_of_parent()
                }
    
                fn delete(mut wid: Self) {
                    assert!(!wid.was_deleted());
                    unsafe {
                        fltk_sys::fl::Fl_delete_widget(
                            wid.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget
                        );
                        wid.inner = std::ptr::null_mut() as *mut _;
                        wid.tracker = std::ptr::null_mut() as *mut fltk_sys::fl::Fl_Widget_Tracker;
                    }
                }
    
                unsafe fn from_widget_ptr(ptr: *mut fltk_sys::widget::Fl_Widget) -> Self {
                    assert!(!ptr.is_null());
                    fltk_sys::fl::Fl_lock();
                    let tracker =
                        fltk_sys::fl::Fl_Widget_Tracker_new(ptr as *mut fltk_sys::fl::Fl_Widget);
                    assert!(!tracker.is_null());
                    let temp = $name {
                        inner: ptr as *mut $flname,
                        tracker: tracker,
                        is_derived: false,
                    };
                    fltk_sys::fl::Fl_unlock();
                    temp
                }
    
                unsafe fn from_widget<W: WidgetExt>(w: W) -> Self {
                    Self::from_widget_ptr(w.as_widget_ptr() as *mut _)
                }
    
                fn handle<F: FnMut(&mut Self, Event) -> bool + 'static>(&mut self, cb: F) {
                    assert!(!self.was_deleted());
                    assert!(self.is_derived);
                    unsafe {
                        unsafe extern "C" fn shim(
                            wid: *mut Fl_Widget,
                            ev: std::os::raw::c_int,
                            data: *mut raw::c_void,
                        ) -> i32 {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let ev: Event = mem::transmute(ev);
                            let a: *mut Box<dyn FnMut(&mut $name, Event) -> bool> =
                                data as *mut Box<dyn FnMut(&mut $name, Event) -> bool>;
                            let f: &mut (dyn FnMut(&mut $name, Event) -> bool) = &mut **a;
                            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                match f(&mut wid, ev) {
                                    true => return 1,
                                    false => return 0,
                                }
                            }));
                            if let Ok(ret) = result {
                                ret
                            } else {
                                0
                            }
                        }
                        let _old_data = self.handle_data();
                        let a: *mut Box<dyn FnMut(&mut Self, Event) -> bool> =
                            Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut raw::c_void = a as *mut raw::c_void;
                        let callback: custom_handler_callback = Some(shim);
                        [<$flname _handle>](self.inner, callback, data);
                    }
                }
    
                fn draw<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
                    assert!(!self.was_deleted());
                    assert!(self.is_derived);
                    unsafe {
                        unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let a: *mut Box<dyn FnMut(&mut $name)> =
                                data as *mut Box<dyn FnMut(&mut $name)>;
                            let f: &mut (dyn FnMut(&mut $name)) = &mut **a;
                            let _ =
                                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                        }
                        let _old_data = self.draw_data();
                        let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut raw::c_void = a as *mut raw::c_void;
                        let callback: custom_draw_callback = Some(shim);
                        [<$flname _draw>](self.inner, callback, data);
                    }
                }
    
                unsafe fn draw_data(&mut self) -> Option<Box<dyn FnMut()>> {
                    let ptr = [<$flname _draw_data>](self.inner);
                    if ptr.is_null() {
                        return None;
                    }
                    let data = ptr as *mut Box<dyn FnMut()>;
                    let data = Box::from_raw(data);
                    [<$flname _draw>](self.inner, None, std::ptr::null_mut());
                    Some(*data)
                }
    
                unsafe fn handle_data(&mut self) -> Option<Box<dyn FnMut(Event) -> bool>> {
                    let ptr = [<$flname _handle_data>](self.inner);
                    if ptr.is_null() {
                        return None;
                    }
                    let data = ptr as *mut Box<dyn FnMut(Event) -> bool>;
                    let data = Box::from_raw(data);
                    [<$flname _handle>](self.inner, None, std::ptr::null_mut());
                    Some(*data)
                }
    
                fn resize_callback<F: FnMut(&mut Self, i32, i32, i32, i32) + 'static>(
                    &mut self,
                    cb: F,
                ) {
                    assert!(!self.was_deleted());
                    assert!(self.is_derived);
                    unsafe {
                        unsafe extern "C" fn shim(
                            wid: *mut Fl_Widget,
                            x: i32,
                            y: i32,
                            w: i32,
                            h: i32,
                            data: *mut raw::c_void,
                        ) {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let a: *mut Box<dyn FnMut(&mut $name, i32, i32, i32, i32)> =
                                data as *mut Box<dyn FnMut(&mut $name, i32, i32, i32, i32)>;
                            let f: &mut (dyn FnMut(&mut $name, i32, i32, i32, i32)) = &mut **a;
                            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                f(&mut wid, x, y, w, h)
                            }));
                        }
                        let a: *mut Box<dyn FnMut(&mut Self, i32, i32, i32, i32)> =
                            Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut raw::c_void = a as *mut raw::c_void;
                        let callback: Option<
                            unsafe extern "C" fn(*mut Fl_Widget, i32, i32, i32, i32, *mut raw::c_void),
                        > = Some(shim);
                        [<$flname _resize_callback>](self.inner, callback, data);
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_widget_type {
    ($name: ident) => {
        impl WidgetType for $name {
            fn to_i32(self) -> i32 {
                self as i32
            }

            fn from_i32(val: i32) -> $name {
                unsafe { mem::transmute(val) }
            }
        }
    };
}
