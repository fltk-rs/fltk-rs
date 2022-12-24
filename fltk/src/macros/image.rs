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
                let x = self.refcount.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                $name {
                    inner: self.inner,
                    refcount: std::sync::atomic::AtomicUsize::new(x + 1),
                }
            }
        }

        paste::paste! {
            impl Drop for $name {
                fn drop(&mut self) {
                    if !self.was_deleted() {
                        self.refcount.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        if *self.refcount.get_mut() < 1 {
                            unsafe {
                                [<$flname _delete>](self.inner);
                            }
                        }
                    }
                }
            }

            unsafe impl ImageExt for $name {
                fn copy(&self) -> Self {
                    assert!(!self.was_deleted());
                    unsafe {
                        let img = [<$flname _copy>](self.inner);
                        assert!(!img.is_null());
                        $name {
                            inner: img,
                            refcount: std::sync::atomic::AtomicUsize::new(1),
                        }
                    }
                }

                fn draw(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _draw>](self.inner, arg2, arg3, arg4, arg5) }
                }

                fn draw_ext(&mut self, arg2: i32, arg3: i32, arg4: i32, arg5: i32, cx: i32, cy: i32) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _draw_ext>](self.inner, arg2, arg3, arg4, arg5, cx, cy)
                    }
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

                fn as_image_ptr(&self) -> *mut fltk_sys::image::Fl_Image {
                    assert!(!self.was_deleted());
                    self.inner as *mut fltk_sys::image::Fl_Image
                }

                unsafe fn from_image_ptr(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
                    assert!(!ptr.is_null());
                    $name {
                        inner: ptr as *mut $flname,
                        refcount: std::sync::atomic::AtomicUsize::new(1),
                    }
                }

                fn to_rgb_data(&self) -> Vec<u8> {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _data>](self.inner);
                        assert!(!ptr.is_null());
                        assert!(!(*ptr).is_null());
                        let cnt = self.data_w() * self.data_h() * self.depth() as i32;
                        let ret: &[u8] = std::slice::from_raw_parts(*ptr as *const u8, cnt as usize);
                        ret.to_vec()
                    }
                }

                fn to_raw_data(&self) -> *const *const u8 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _data>](self.inner) as *const *const u8 }
                }

                fn to_rgb(&self) -> Result<$crate::image::RgbImage, FltkError> {
                    assert!(!self.was_deleted());
                    if self.count() != 1 {
                        Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
                    } else {
                        let data = self.to_rgb_data();
                        let mut img = $crate::image::RgbImage::new(&data, self.data_w(), self.data_h(), self.depth())?;
                        img.scale(self.w(), self.h(), false, true);
                        Ok(img)
                    }
                }

                fn to_rgb_image(&self) -> Result<$crate::image::RgbImage, FltkError> {
                    self.to_rgb()
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
                            self.inner,
                            width,
                            height,
                            proportional as i32,
                            can_expand as i32,
                        )
                    }
                }

                fn count(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _count>](self.inner) }
                }

                fn data_w(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _data_w>](self.inner) }
                }

                fn data_h(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _data_h>](self.inner) }
                }

                fn depth(&self) -> $crate::enums::ColorDepth {
                    assert!(!self.was_deleted());
                    unsafe { std::mem::transmute([<$flname _d>](self.inner) as u8) }
                }

                fn ld(&self) -> i32 {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _ld>](self.inner) }
                }

                fn inactive(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _inactive>](self.inner) }
                }

                unsafe fn delete(mut img: Self) {
                    assert!(!img.inner.is_null());
                    [<$flname _delete>](img.inner);
                    img.inner = std::ptr::null_mut() as *mut $flname;
                }

                unsafe fn increment_arc(&mut self) {
                    assert!(!self.was_deleted());
                    self.refcount.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }

                unsafe fn decrement_arc(&mut self) {
                    assert!(!self.was_deleted());
                    self.refcount.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                    assert!(
                        *self.refcount.get_mut() > 1,
                        "The image should outlive the widget!"
                    );
                }

                fn was_deleted(&self) -> bool {
                    self.inner.is_null()
                }

                unsafe fn into_image<I: ImageExt>(self) -> I {
                    I::from_image_ptr(self.inner as *mut _)
                }

                fn from_dyn_image_ptr(p: *mut fltk_sys::image::Fl_Image) -> Option<Self> {
                    unsafe {
                        let ptr = [<$flname _from_dyn_ptr>](p);
                        if ptr.is_null() {
                            None
                        } else {
                            Some($name {
                                inner: ptr as *mut $flname,
                                refcount: std::sync::atomic::AtomicUsize::new(1),
                            })
                        }
                    }
                }

                fn from_dyn_image<I: ImageExt>(i: &I) -> Option<Self> {
                    Self::from_dyn_image_ptr(i.as_image_ptr())
                }
            }
        }
    };
}

pub use impl_image_ext;
