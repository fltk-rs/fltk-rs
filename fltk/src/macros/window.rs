#[doc(hidden)]
#[macro_export]
/// Implements WindowExt
macro_rules! impl_window_ext {
    ($name: ident, $flname: ident) => {
        #[cfg(any(feature = "raw-window-handle", feature = "rwh05"))]
        unsafe impl HasRawWindowHandle for $name {
            fn raw_window_handle(&self) -> RawWindowHandle {
                #[cfg(target_os = "windows")]
                {
                    #[cfg(feature = "rwh05")]
                    type Handle = Win32WindowHandle;
                    #[cfg(feature = "raw-window-handle")]
                    type Handle = Win32Handle;
                    let mut handle = Handle::empty();
                    handle.hwnd = self.raw_handle();
                    handle.hinstance = $crate::app::display();
                    return RawWindowHandle::Win32(handle);
                }

                #[cfg(target_os = "macos")]
                {
                    let raw = self.raw_handle();
                    unsafe extern "C" {
                        pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
                    }
                    let cv = unsafe { cfltk_getContentView(raw) };
                    #[cfg(feature = "rwh05")]
                    type Handle = AppKitWindowHandle;
                    #[cfg(feature = "raw-window-handle")]
                    type Handle = AppKitHandle;
                    let mut handle = Handle::empty();
                    handle.ns_window = raw;
                    handle.ns_view = cv as _;
                    return RawWindowHandle::AppKit(handle);
                }

                #[cfg(target_os = "android")]
                {
                    #[cfg(feature = "rwh05")]
                    type Handle = AndroidNdkWindowHandle;
                    #[cfg(feature = "raw-window-handle")]
                    type Handle = AndroidNdkHandle;
                    let mut handle = Handle::empty();
                    handle.a_native_window = self.raw_handle();
                    return RawWindowHandle::AndroidNdk(handle);
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                {
                    if !$crate::app::using_wayland() {
                        #[cfg(feature = "rwh05")]
                        type Handle = XlibWindowHandle;
                        #[cfg(feature = "raw-window-handle")]
                        type Handle = XlibHandle;
                        let mut handle = Handle::empty();
                        handle.window = self.raw_handle() as RawXlibHandle;
                        return RawWindowHandle::Xlib(handle);
                    } else {
                        #[cfg(feature = "rwh05")]
                        type Handle = WaylandWindowHandle;
                        #[cfg(feature = "raw-window-handle")]
                        type Handle = WaylandHandle;
                        let mut handle = Handle::empty();
                        handle.surface = unsafe { resolve_raw_handle(self.raw_handle() as *mut raw::c_void) };
                        return RawWindowHandle::Wayland(handle);
                    }
                }
            }
        }

        #[cfg(feature = "rwh05")]
        unsafe impl HasRawDisplayHandle for $name {
            fn raw_display_handle(&self) -> RawDisplayHandle {
                #[cfg(target_os = "windows")]
                {
                    type Handle = WindowsDisplayHandle;
                    let handle = Handle::empty();
                    return RawDisplayHandle::Windows(handle);
                }

                #[cfg(target_os = "macos")]
                {
                    type Handle = AppKitDisplayHandle;
                    let handle = Handle::empty();
                    return RawDisplayHandle::AppKit(handle);
                }

                #[cfg(target_os = "android")]
                {
                    type Handle = AndroidDisplayHandle;
                    let handle = Handle::empty();
                    return RawDisplayHandle::Android(handle);
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                {
                    if !$crate::app::using_wayland() {
                        type Handle = XlibDisplayHandle;
                        let mut handle = Handle::empty();
                        handle.display = $crate::app::display();
                        handle.screen = self.screen_num();
                        return RawDisplayHandle::Xlib(handle);
                    } else {
                        type Handle = WaylandDisplayHandle;
                        let mut handle = Handle::empty();
                        handle.display = $crate::app::display();
                        return RawDisplayHandle::Wayland(handle);
                    }
                }
            }
        }

        #[cfg(feature = "rwh06")]
        impl HasWindowHandle for $name {
            fn window_handle(&self) ->  Result<WindowHandle<'_>, HandleError> {
                #[cfg(target_os = "windows")]
                {
                    let mut handle = Win32WindowHandle::new(std::num::NonZeroIsize::new(self.raw_handle() as isize).unwrap());
                    handle.hinstance = std::num::NonZeroIsize::new($crate::app::display() as isize);
                    return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Win32(handle)) });
                }

                #[cfg(target_os = "macos")]
                {
                    let raw = self.raw_handle();
                    unsafe extern "C" {
                        pub fn cfltk_getContentView(xid: *mut raw::c_void) -> *mut raw::c_void;
                    }
                    let cv = unsafe { cfltk_getContentView(raw) };
                    let handle = AppKitWindowHandle::new(std::ptr::NonNull::new(cv).unwrap());
                    return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AppKit(handle)) });
                }

                #[cfg(target_os = "android")]
                {
                    let handle = AndroidNdkWindowHandle::new(std::ptr::NonNull::new(self.raw_handle()).unwrap());
                    return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(handle)) });
                }

                #[cfg(target_os = "emscripten")]
                {
                    let handle = WebCanvasWindowHandle::new(std::ptr::NonNull::new(unsafe { resolve_raw_handle(self.raw_handle() as *mut raw::c_void) }).unwrap());
                    return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::WebCanvas(handle)) });
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                {
                    if !$crate::app::using_wayland() {
                        let handle = XlibWindowHandle::new(self.raw_handle() as RawXlibHandle);
                        return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Xlib(handle)) });
                    } else {
                        let handle = WaylandWindowHandle::new(std::ptr::NonNull::new(unsafe { resolve_raw_handle(self.raw_handle() as *mut raw::c_void) }).unwrap());
                        return Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Wayland(handle)) });
                    }
                }
            }
        }

        #[cfg(feature = "rwh06")]
        impl HasDisplayHandle for $name {
            fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
                #[cfg(target_os = "windows")]
                {
                    let handle = WindowsDisplayHandle::new();
                    return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Windows(handle)) });
                }

                #[cfg(target_os = "macos")]
                {
                    let handle = AppKitDisplayHandle::new();
                    return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::AppKit(handle)) });
                }

                #[cfg(target_os = "android")]
                {
                    let handle = AndroidDisplayHandle::new();
                    return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Android(handle)) });
                }

                #[cfg(target_os = "emscripten")]
                {
                    let handle = WebDisplayHandle::new();
                    return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Web(handle)) });
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                {
                    if !$crate::app::using_wayland() {
                        let display = std::ptr::NonNull::new($crate::app::display());
                        let screen = self.screen_num();
                        let handle = XlibDisplayHandle::new(display, screen);
                        return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Xlib(handle)) });
                    } else {
                        let handle = WaylandDisplayHandle::new(std::ptr::NonNull::new($crate::app::display()).unwrap());
                        return Ok(unsafe { DisplayHandle::borrow_raw(RawDisplayHandle::Wayland(handle)) });
                    }
                }
            }
        }

        paste::paste! {
            unsafe impl WindowExt for $name {
                fn set_center_screen(&mut self) {
                    debug_assert!(
                        self.w() != 0 && self.h() != 0,
                        "center_screen requires the size of the widget to be known!"
                    );
                    let (mut x, mut y) = screen_size();
                    x -= self.w();
                    y -= self.h();
                    self.resize(
                        x / 2,
                        y / 2,
                        self.w(),
                        self.h(),
                    );
                }

                fn make_modal(&mut self, val: bool) {
                    unsafe { [<$flname _make_modal>](self.inner.widget() as _, val as u32) }
                }

                fn fullscreen(&mut self, val: bool) {
                    unsafe { [<$flname _fullscreen>](self.inner.widget() as _, val as u32) }
                }

                fn make_current(&mut self) {
                    unsafe { [<$flname _make_current>](self.inner.widget() as _) }
                }

                fn icon(&self) -> Option<Box<dyn ImageExt>> {
                    unsafe {
                        let image_ptr = [<$flname _icon>](self.inner.widget() as _);
                        if image_ptr.is_null() {
                            None
                        } else {
                            let img =
                                $crate::image::Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                            Some(Box::new(img))
                        }
                    }
                }

                fn set_icon<T: ImageExt>(&mut self, image: Option<T>) {
                    assert!(
                        std::any::type_name::<T>()
                            != std::any::type_name::<$crate::image::SharedImage>(),
                        "SharedImage icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::Pixmap>(),
                        "Pixmap icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::XpmImage>(),
                        "Xpm icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::XbmImage>(),
                        "Xbm icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::PnmImage>(),
                        "Pnm icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::GifImage>(),
                        "Gif icons are not supported!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::Image>(),
                        "Icon images can't be generic!"
                    );
                    assert!(
                        std::any::type_name::<T>() != std::any::type_name::<$crate::image::TiledImage>(),
                        "TiledImage icons are not supported!"
                    );
                    if let Some(image) = image {
                        assert!(!image.was_deleted());
                        if std::any::type_name::<T>() == std::any::type_name::<$crate::image::SvgImage>()
                        {
                            unsafe {
                                [<$flname _set_icon>](
                                    self.inner.widget() as _,
                                    image.as_image_ptr() as *mut _,
                                )
                            }
                        } else {
                            // Shouldn't fail after the previous asserts!
                            unsafe {
                                [<$flname _set_icon>](
                                    self.inner.widget() as _,
                                    image.to_rgb().unwrap().as_image_ptr() as *mut _,
                                )
                            }
                        }
                    } else {
                    unsafe {
                            [<$flname _set_icon>](
                                self.inner.widget() as _,
                                std::ptr::null_mut() as *mut raw::c_void,
                            )
                        }
                    }
                }

                fn set_cursor(&mut self, cursor: Cursor) {
                    unsafe { [<$flname _set_cursor>](self.inner.widget() as _, cursor as i32) }
                }

                fn shown(&self) -> bool {
                    unsafe { [<$flname _shown>](self.inner.widget() as _) != 0 }
                }

                fn set_border(&mut self, flag: bool) {
                    assert!($crate::app::is_ui_thread());
                    unsafe { [<$flname _set_border>](self.inner.widget() as _, flag as i32) }
                }

                fn border(&self) -> bool {
                    unsafe { [<$flname _border>](self.inner.widget() as _) != 0 }
                }

                fn free_position(&mut self) {
                    unsafe { [<$flname _free_position>](self.inner.widget() as _) }
                }

                fn raw_handle(&self) -> RawHandle {
                    unsafe {
                        let ptr = [<$flname _raw_handle>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        return ptr as RawHandle;
                    }
                }

                unsafe fn set_raw_handle(&mut self, handle: RawHandle) {
                    assert!(handle as isize != 0);
                    Fl_Window_set_raw_handle(self.inner.widget() as *mut Fl_Window, &handle as *const _ as *mut _);
                }

                fn region(&self) -> $crate::draw::Region {
                    unsafe {
                        let ptr = [<$flname _region>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        $crate::draw::Region(ptr)
                    }
                }

                unsafe fn set_region(&mut self, region: $crate::draw::Region) {
                    assert!(!region.0.is_null());
                    [<$flname _set_region>](self.inner.widget() as _, region.0)
                }

                fn iconize(&mut self) {
                    unsafe { [<$flname _iconize>](self.inner.widget() as _) }
                }

                fn fullscreen_active(&self) -> bool {
                    unsafe { [<$flname _fullscreen_active>](self.inner.widget() as _) != 0 }
                }

                fn decorated_w(&self) -> i32 {
                    unsafe { [<$flname _decorated_w>](self.inner.widget() as _) }
                }

                fn decorated_h(&self) -> i32 {
                    unsafe { [<$flname _decorated_h>](self.inner.widget() as _) }
                }

                fn size_range(&mut self, min_w: i32, min_h: i32, max_w: i32, max_h: i32) {
                    let max_w = if max_w > u16::MAX as i32 {
                        0
                    } else {
                        max_w
                    };
                    let max_h = if max_h > u16::MAX as i32 {
                        0
                    } else {
                        max_h
                    };
                    unsafe {
                        [<$flname _size_range>](self.inner.widget() as _, min_w, min_h, max_w, max_h);
                    }
                }

                fn hotspot<W: WidgetExt>(&mut self, w: &W) {
                    unsafe { [<$flname _hotspot>](self.inner.widget() as _, w.as_widget_ptr() as _) }
                }

                fn set_shape<I: ImageExt>(&mut self, image: Option<I>) {
                    assert!(self.w() != 0);
                    assert!(self.h() != 0);
                    assert!(
                        std::any::type_name::<I>()
                            != std::any::type_name::<$crate::image::SharedImage>(),
                        "SharedImage is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::XbmImage>(),
                        "Xbm is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::PnmImage>(),
                        "Pnm is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::GifImage>(),
                        "Gif is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::JpegImage>(),
                        "Jpeg is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::SvgImage>(),
                        "Svg is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::PngImage>(),
                        "Png is not supported!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::Image>(),
                        "Images can't be generic!"
                    );
                    assert!(
                        std::any::type_name::<I>() != std::any::type_name::<$crate::image::TiledImage>(),
                        "TiledImage is not supported!"
                    );
                    unsafe {
                        if let Some(image) = image {
                            assert!(!image.was_deleted());
                            assert!(image.w() == image.data_w() as i32);
                            assert!(image.h() == image.data_h() as i32);
                            [<$flname _set_shape>](self.inner.widget() as _, image.as_image_ptr() as _)
                        };
                    }
                }

                fn shape(&self) -> Option<Box<dyn ImageExt>> {
                    unsafe {
                        let image_ptr = [<$flname _shape>](self.inner.widget() as _);
                        if image_ptr.is_null() {
                            None
                        } else {
                            let img =
                            $crate::image::Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                            Some(Box::new(img))
                        }
                    }
                }

                fn x_root(&self) -> i32 {
                    unsafe { [<$flname _x_root>](self.inner.widget() as _) }
                }

                fn y_root(&self) -> i32 {
                    unsafe { [<$flname _y_root>](self.inner.widget() as _) }
                }

                fn set_cursor_image(
                    &mut self,
                    mut image: $crate::image::RgbImage,
                    hot_x: i32,
                    hot_y: i32,
                ) {
                    if image.data_w() != image.w() || image.data_h() == image.h() {
                        image.scale(image.data_w(), image.data_h(), false, true);
                    }
                    unsafe {
                        assert!(!image.was_deleted());
                        [<$flname _set_cursor_image>](
                            self.inner.widget() as _,
                            image.as_image_ptr() as _,
                            hot_x,
                            hot_y,
                        )
                    }
                }

                fn default_cursor(&mut self, cursor: Cursor) {
                    unsafe { [<$flname _default_cursor>](self.inner.widget() as _, cursor as i32) }
                }

                fn screen_num(&self) -> i32 {
                    unsafe { [<$flname _screen_num>](self.inner.widget() as _) }
                }

                fn set_screen_num(&mut self, n: i32) {
                    unsafe { [<$flname _set_screen_num>](self.inner.widget() as _, n) }
                }

                fn wait_for_expose(&self) {
                    unsafe { [<$flname _wait_for_expose>](self.inner.widget() as _) }
                }

                fn opacity(&self) -> f64 {
                    assert!(self.is_derived);
                    unsafe { [<$flname _alpha>](self.inner.widget() as _) as f64 / 255.0 }
                }

                fn set_opacity(&mut self, val: f64) {
                    assert!(self.is_derived);
                    if self.shown() {
                        self.wait_for_expose();
                        let val: u8 = if val > 1.0 {
                            255
                        } else if val < 0.0 {
                            0
                        } else {
                            (val * 255.0).round() as u8
                        };
                        unsafe { [<$flname _set_alpha>](self.inner.widget() as _, val) }
                    }
                }

                fn xclass(&self) -> Option<String> {
                    unsafe {
                        let ptr = [<$flname _xclass>](self.inner.widget() as _);
                        if ptr.is_null() {
                            None
                        } else {
                            Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
                        }
                    }
                }

                fn set_xclass(&mut self, s: &str) {
                    let s = CString::safe_new(s);
                    unsafe { [<$flname _set_xclass>](self.inner.widget() as _, s.as_ptr()) }
                }

                fn clear_modal_states(&mut self) {
                    unsafe { [<$flname _clear_modal_states>](self.inner.widget() as _) }
                }

                fn force_position(&mut self, flag: bool) {
                    assert!(self.is_derived);
                    unsafe { [<$flname _force_position>](self.inner.widget() as _, flag as _) }
                }

                fn set_override(&mut self) {
                    unsafe { [<$flname _set_override>](self.inner.widget() as _) }
                }

                fn is_override(&self) -> bool {
                    unsafe { [<$flname _override>](self.inner.widget() as _) != 0 }
                }

                fn set_icon_label(&mut self, label: &str) {
                    let label = CString::safe_new(label);
                    unsafe { [<$flname _set_icon_label>](self.inner.widget() as _, label.as_ptr()) }
                }

                fn icon_label(&self) -> Option<String> {
                    unsafe {
                        let label_ptr = [<$flname _icon_label>](self.inner.widget() as _);
                        if label_ptr.is_null() {
                            None
                        } else {
                            Some(CStr::from_ptr(label_ptr as *mut std::os::raw::c_char)
                            .to_string_lossy()
                            .to_string())
                        }
                    }
                }
            }
        }
    };
}

pub use impl_window_ext;
