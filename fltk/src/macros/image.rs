#[doc(hidden)]
#[macro_export]
/// Implements ImageExt
macro_rules! impl_image_ext {
    ($name: ident, $flname: ident) => {
        #[cfg(not(feature = "single-threaded"))]
        unsafe impl Sync for $name {}
        #[cfg(not(feature = "single-threaded"))]
        unsafe impl Send for $name {}

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }

        impl Eq for $name {}

        impl Clone for $name {
            fn clone(&self) -> Self {
                assert!(!self.was_deleted());
                $name {
                    inner: ImageRC::clone(&self.inner),
                }
            }
        }

        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    if std::any::type_name::<$name>() != std::any::type_name::<$crate::image::Image>() {
                        if !self.was_deleted() {
                            if ImageRC::strong_count(&self.inner) == 1 {
                                unsafe {
                                    [<$flname _delete>](*self.inner);
                                }
                            }
                        }
                    }
                }
            }

            unsafe impl ImageExt for $name {
                fn copy(&self) -> Self {
                    assert!(!self.was_deleted());
                    unsafe {
                        let img = [<$flname _copy>](*self.inner);
                        assert!(!img.is_null());
                        $name {
                            inner: ImageRC::from(img),
                        }
                    }
                }

                fn copy_sized(&self, w: i32, h: i32) -> Self {
                    assert!(!self.was_deleted());
                    unsafe {
                        let img = [<$flname _copy_sized>](*self.inner, w, h);
                        assert!(!img.is_null());
                        $name {
                            inner: ImageRC::from(img),
                        }
                    }
                }

                fn draw(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _draw>](*self.inner, arg2, arg3, arg4, arg5) }
                }

                fn draw_ext(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32, cx: i32, cy: i32) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _draw_ext>](*self.inner, arg2, arg3, arg4, arg5, cx, cy)
                    }
                }

                fn w(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _width>](*self.inner) }
                }

                fn h(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _height>](*self.inner) }
                }

                fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image {
                    assert!(!self.was_deleted());
                    let ptr = std::rc::Rc::into_raw(self.inner.clone());
                    unsafe {
                        *ptr as _
                    }
                }

                unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
                    assert!(!ptr.is_null());
                    $name {
                        inner: ImageRC::from(ptr as *mut $flname),
                    }
                }

                fn as_rgb_data(&self) -> Vec<u8> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _data>](*self.inner);
                        assert!(!ptr.is_null());
                        assert!(!(*ptr).is_null());
                        let cnt = self.data_w() * self.data_h() * self.depth() as i32;
                        let ret: &[u8] = std::slice::from_raw_parts(*ptr as *const u8, cnt as usize);
                        ret.to_vec()
                    }
                }

                fn as_rgb_image(&self) -> Result<$crate::image::RgbImage, FltkError> {
                    assert!(!self.was_deleted());
                    if self.count() != 1 {
                        Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
                    } else {
                        let data = self.as_rgb_data();
                        let mut img = $crate::image::RgbImage::new(&data, self.data_w(), self.data_h(), self.depth())?;
                        img.scale(self.w(), self.h(), false, true);
                        Ok(img)
                    }
                }

                fn scale(&mut self, width: i32, height: i32, proportional: bool, can_expand: bool) {
                    assert!(!self.was_deleted());
                    let width = if width < 1 {
                        1
                    } else {
                        width
                    };
                    let height = if height < 1 {
                        1
                    } else {
                        height
                    };
                    unsafe {
                        [<$flname _scale>](
                            *self.inner,
                            width,
                            height,
                            proportional as i32,
                            can_expand as i32,
                        )
                    }
                }

                fn count(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _count>](*self.inner) }
                }

                fn data_w(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _data_w>](*self.inner) }
                }

                fn data_h(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _data_h>](*self.inner) }
                }

                fn depth(&self) -> $crate::enums::ColorDepth {
                    assert!(!self.was_deleted());
                    unsafe { std::mem::transmute([<$flname _d>](*self.inner) as u8) }
                }

                fn ld(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _ld>](*self.inner) }
                }

                fn inactive(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _inactive>](*self.inner) }
                }

                unsafe fn delete(img: Self) { unsafe {
                    assert!(!img.inner.is_null());
                    [<$flname _delete>](*img.inner);
                }}

                fn was_deleted(&self) -> bool {
                    self.inner.is_null()
                }

                unsafe fn into_image<I: ImageExt>(self) -> I { unsafe {
                    let ptr = ImageRC::into_raw(ImageRC::clone(&self.inner));
                    ImageRC::increment_strong_count(ptr);
                    let image = ImageRC::from_raw(ptr);
                    I::from_image_ptr(*image as *mut _)
                }}

                fn color_average(&mut self, c: $crate::enums::Color, i: f32) {
                    assert!(!self.was_deleted());
                    unsafe { Fl_Image_color_average(*self.inner as *mut _, c.bits(), i) }
                }

                fn desaturate(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { Fl_Image_desaturate(*self.inner as *mut _) }
                }

                fn uncache(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { Fl_Image_uncache(*self.inner as *mut _) }
                }

                // fn label_for_widget<W: $crate::prelude::WidgetExt>(&self, w: &mut W) {
                //     assert!(!self.was_deleted());
                //     unsafe { Fl_Image_label_widget(*self.inner as *mut _, w.as_widget_ptr() as *mut _) }
                // }

                fn label_for_menu_item(&self, item: &mut $crate::menu::MenuItem) {
                    assert!(!self.was_deleted());
                    unsafe { Fl_Image_label_menu_item(*self.inner as *mut _, item.as_ptr() as *mut _) }
                }
            }
        }
    };
}

pub use impl_image_ext;
