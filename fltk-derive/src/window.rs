use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_window_trait(ast: &DeriveInput) -> TokenStream {
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
    let icon = Ident::new(format!("{}_{}", name_str, "icon").as_str(), name.span());
    let set_icon = Ident::new(format!("{}_{}", name_str, "set_icon").as_str(), name.span());
    let set_border = Ident::new(
        format!("{}_{}", name_str, "set_border").as_str(),
        name.span(),
    );
    let border = Ident::new(format!("{}_{}", name_str, "border").as_str(), name.span());
    let free_position = Ident::new(
        format!("{}_{}", name_str, "free_position").as_str(),
        name.span(),
    );
    let set_cursor = Ident::new(
        format!("{}_{}", name_str, "set_cursor").as_str(),
        name.span(),
    );
    let shown = Ident::new(format!("{}_{}", name_str, "shown").as_str(), name.span());
    let raw_handle = Ident::new(
        format!("{}_{}", name_str, "raw_handle").as_str(),
        name.span(),
    );
    let region = Ident::new(format!("{}_{}", name_str, "region").as_str(), name.span());
    let set_region = Ident::new(
        format!("{}_{}", name_str, "set_region").as_str(),
        name.span(),
    );
    let iconize = Ident::new(format!("{}_{}", name_str, "iconize").as_str(), name.span());
    let fullscreen_active = Ident::new(
        format!("{}_{}", name_str, "fullscreen_active").as_str(),
        name.span(),
    );
    let decorated_w = Ident::new(
        format!("{}_{}", name_str, "decorated_w").as_str(),
        name.span(),
    );
    let decorated_h = Ident::new(
        format!("{}_{}", name_str, "decorated_h").as_str(),
        name.span(),
    );
    let size_range = Ident::new(
        format!("{}_{}", name_str, "size_range").as_str(),
        name.span(),
    );
    let hotspot = Ident::new(format!("{}_{}", name_str, "hotspot").as_str(), name.span());
    let shape = Ident::new(format!("{}_{}", name_str, "shape").as_str(), name.span());
    let set_shape = Ident::new(
        format!("{}_{}", name_str, "set_shape").as_str(),
        name.span(),
    );
    let x_root = Ident::new(format!("{}_{}", name_str, "x_root").as_str(), name.span());
    let y_root = Ident::new(format!("{}_{}", name_str, "y_root").as_str(), name.span());
    let set_cursor_image = Ident::new(
        format!("{}_{}", name_str, "set_cursor_image").as_str(),
        name.span(),
    );
    let default_cursor = Ident::new(
        format!("{}_{}", name_str, "default_cursor").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl HasRawWindowHandle for #name {
            fn raw_window_handle(&self) -> RawWindowHandle {
                #[cfg(target_os = "windows")]
                {
                    return RawWindowHandle::Windows(windows::WindowsHandle {
                        hwnd: self.raw_handle(),
                        hinstance: crate::app::display(),
                        ..windows::WindowsHandle::empty()
                    });
                }

                #[cfg(target_os = "macos")]
                {
                    let raw = self.raw_handle();
                    let cv: *mut objc::runtime::Object = unsafe {
                        msg_send![raw as *mut objc::runtime::Object, contentView]
                    };
                    return RawWindowHandle::MacOS(macos::MacOSHandle {
                        ns_window: raw,
                        ns_view: cv as _,
                        ..macos::MacOSHandle::empty()
                    });
                }

                #[cfg(target_os = "android")]
                {
                    return RawWindowHandle::Android(android::AndroidHandle {
                        a_native_window: self.raw_handle(),
                        ..android::AndroidHandle::empty()
                    });
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                {
                    return RawWindowHandle::Xlib(unix::XlibHandle {
                        window: self.raw_handle(),
                        display: crate::app::display(),
                        ..unix::XlibHandle::empty()
                    });
                }
            }
        }

        unsafe impl WindowExt for #name {
            fn center_screen(mut self) -> Self {
                assert!(!self.was_deleted());
                debug_assert!(self.width() != 0 && self.height() != 0, "center_screen requires the size of the widget to be known!");
                let (mut x, mut y) = crate::app::screen_size();
                x -= self.width() as f64;
                y -= self.height() as f64;
                self.resize((x / 2.0) as i32, (y / 2.0) as i32, self.width(), self.height());
                self
            }

            fn make_modal(&mut self, val: bool) {
                assert!(!self.was_deleted());
                unsafe { #make_modal(self.inner, val as u32) }
            }

            fn fullscreen(&mut self, val: bool) {
                assert!(!self.was_deleted());
                unsafe { #fullscreen(self.inner, val as u32) }
            }

            fn make_current(&mut self) {
                assert!(!self.was_deleted());
                unsafe { #make_current(self.inner) }
            }

            fn icon(&self) -> Option<Box<dyn ImageExt>> {
                unsafe {
                    assert!(!self.was_deleted());
                    let image_ptr = #icon(self.inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        let mut img = Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                        Some(Box::new(img))
                    }
                }
            }

            fn set_icon<T: ImageExt>(&mut self, image: Option<T>) {
                assert!(!self.was_deleted());
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::Pixmap>(), "Pixmap icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::XpmImage>(), "Xpm icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::XbmImage>(), "Xbm icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::PnmImage>(), "Pnm icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::GifImage>(), "Gif icons are not supported!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::Image>(), "Icon images can't be generic!");
                assert!(std::any::type_name::<T>() != std::any::type_name::<crate::image::TiledImage>(), "TiledImage icons are not supported!");
                if let Some(mut image) = image {
                    assert!(!image.was_deleted());
                    if std::any::type_name::<T>() == std::any::type_name::<crate::image::SvgImage>() {
                        unsafe { image.increment_arc(); #set_icon(self.inner, image.as_image_ptr() as *mut _) }
                    } else {
                        // Shouldn't fail after the previous asserts!
                        unsafe { #set_icon(self.inner, image.to_rgb().unwrap().as_image_ptr() as *mut _) }
                    }
                } else {
                    unsafe { #set_icon(self.inner, std::ptr::null_mut() as *mut raw::c_void) }
                }
            }

            fn set_cursor(&mut self, cursor: Cursor) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_cursor(self.inner, cursor as i32)
                }
            }

            fn shown(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #shown(self.inner)  != 0
                }
            }

            fn set_border(&mut self, flag: bool) {
                assert!(!self.was_deleted());
                unsafe {
                    #set_border(self.inner, flag as i32)
                }
            }

            fn border(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #border(self.inner) != 0
                }
            }

            fn free_position(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #free_position(self.inner)
                }
            }

            fn raw_handle(&self) -> RawHandle {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #raw_handle(self.inner);
                    assert!(!ptr.is_null());
                    let winid = resolve_raw_handle(ptr);

                    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "android", target_os = "ios"))]
                    return winid.opaque;

                    #[cfg(any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ))]
                    return winid.x_id as RawHandle;
                }
            }

            unsafe fn set_raw_handle(&mut self, handle: RawHandle) {
                assert!(!self.was_deleted());

                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "android", target_os = "ios"))]
                assert!(!handle.is_null());

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                assert!(handle != 0);

                Fl_Window_set_raw_handle(self.inner as *mut Fl_Window, mem::transmute(&handle));
            }

            fn region(&self) -> crate::draw::Region {
                assert!(!self.was_deleted());
                unsafe {
                    let ptr = #region(self.inner);
                    assert!(!ptr.is_null());
                    ptr
                }
            }

            unsafe fn set_region(&mut self, region: crate::draw::Region) {
                assert!(!self.was_deleted());
                assert!(!region.is_null());
                #set_region(self.inner, region)
            }

            fn iconize(&mut self) {
                assert!(!self.was_deleted());
                unsafe {
                    #iconize(self.inner)
                }
            }

            fn fullscreen_active(&self) -> bool {
                assert!(!self.was_deleted());
                unsafe {
                    #fullscreen_active(self.inner) != 0
                }
            }

            fn decorated_w(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #decorated_w(self.inner)
                }
            }

            fn decorated_h(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #decorated_h(self.inner)
                }
            }

            fn size_range(&mut self, min_w: i32, min_h: i32, max_w: i32, max_h: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    #size_range(self.inner, min_w, min_h, max_w, max_h);
                }
            }

            fn hotspot<W: WidgetExt>(&mut self, w: &W) {
                assert!(!self.was_deleted());
                assert!(!w.was_deleted());
                unsafe {
                    #hotspot(self.inner, w.as_widget_ptr() as _)
                }
            }

            fn set_shape<I: ImageExt>(&mut self, image: Option<I>) {
                assert!(!self.was_deleted());
                assert!(self.w() != 0);
                assert!(self.h() != 0);
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::XbmImage>(), "Xbm is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::PnmImage>(), "Pnm is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::GifImage>(), "Gif is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::GifImage>(), "Jpeg is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(), "Svg is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(), "Png is not supported!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::Image>(), "Images can't be generic!");
                assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::TiledImage>(), "TiledImage is not supported!");
                unsafe {
                    if let Some(image) = image {
                        assert!(image.w() == image.data_w() as i32);
                        assert!(image.h() == image.data_h() as i32);
                        #set_shape(self.inner, image.as_image_ptr() as _)
                    };
                }
            }

            fn shape(&self) -> Option<Box<dyn ImageExt>> {
                assert!(!self.was_deleted());
                unsafe {
                    let image_ptr = #shape(self.inner);
                    if image_ptr.is_null() {
                        None
                    } else {
                        let mut img = Image::from_image_ptr(image_ptr as *mut fltk_sys::image::Fl_Image);
                        Some(Box::new(img))
                    }
                }
            }

            fn x_root(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #x_root(self.inner)
                }
            }

            fn y_root(&self) -> i32 {
                assert!(!self.was_deleted());
                unsafe {
                    #y_root(self.inner)
                }
            }

            fn set_cursor_image(&mut self, mut image: crate::image::RgbImage, hot_x: i32, hot_y: i32) {
                assert!(!self.was_deleted());
                unsafe {
                    image.increment_arc();
                    #set_cursor_image(self.inner, image.as_image_ptr() as _, hot_x, hot_y)
                }
            }

            fn default_cursor(&mut self, cursor: Cursor) {
                assert!(!self.was_deleted());
                unsafe {
                    #default_cursor(self.inner, cursor as i32)
                }
            }
        }
    };
    gen.into()
}
