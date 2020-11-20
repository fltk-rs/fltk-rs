pub use crate::prelude::*;
use fltk_sys::surface::*;
use std::ffi::CString;
use std::path;

/// An image surface object
pub struct ImageSurface {
    _inner: *mut Fl_Image_Surface,
}

impl SurfaceDevice for ImageSurface {
    fn is_current(&self) -> bool {
        unsafe { Fl_Surface_Device_is_current(self._inner as *mut _) != 0 }
    }

    fn surface() -> Self {
        unsafe {
            let ptr = Fl_Surface_Device_surface();
            assert!(!ptr.is_null());
            Self {
                _inner: ptr as *mut _,
            }
        }
    }

    fn push_current(new_current: &ImageSurface) {
        unsafe { Fl_Surface_Device_push_current(new_current._inner as *mut _) }
    }

    fn pop_current() {
        unsafe {
            Fl_Surface_Device_pop_current();
        }
    }
}

impl ImageSurface {
    /// Creates a new image surface
    pub fn new(w: i32, h: i32, high_res: bool) -> ImageSurface {
        unsafe {
            let ptr = Fl_Image_Surface_new(w, h, high_res as i32);
            assert!(!ptr.is_null());
            ImageSurface { _inner: ptr }
        }
    }

    /// Gets the image of an image surface as an rgb image
    pub fn image(&self) -> Option<crate::image::RgbImage> {
        unsafe {
            let ptr = Fl_Image_Surface_image(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::image::RgbImage::from_image_ptr(ptr as *mut _))
            }
        }
    }

    /// Gets the high resolution image of an image surface as a shared image
    pub fn highres_image(&self) -> Option<crate::image::SharedImage> {
        unsafe {
            let ptr = Fl_Image_Surface_highres_image(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::image::SharedImage::from_image_ptr(ptr as *mut _))
            }
        }
    }

    /// Gets the origin coordinates of an image surface
    pub fn origin(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_Image_Surface_origin(self._inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Set the origin coordinates of an image surface
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_Image_Surface_set_origin(self._inner, x, y) }
    }

    /// Rescale an image surface
    pub fn rescale(&mut self) {
        unsafe { Fl_Image_Surface_rescale(self._inner) }
    }

    /// Draw a widget on the image surface
    pub fn draw<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_Image_Surface_draw(
                self._inner,
                widget.as_widget_ptr() as *mut _,
                delta_x,
                delta_y,
            )
        }
    }

    /// draw a decorated window
    pub fn draw_decorated_window<W: WindowExt>(&self, win: &W, x_offset: i32, y_offset: i32) {
        unsafe {
            Fl_Image_Surface_draw_decorated_window(
                self._inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }
}

impl Drop for ImageSurface {
    fn drop(&mut self) {
        unsafe { Fl_Image_Surface_delete(self._inner) }
    }
}

/// An image surface object
pub struct SvgFileSurface {
    _inner: *mut Fl_SVG_File_Surface,
}

impl SurfaceDevice for SvgFileSurface {
    fn is_current(&self) -> bool {
        unsafe { Fl_Surface_Device_is_current(self._inner as *mut _) != 0 }
    }

    fn surface() -> Self {
        unsafe {
            let ptr = Fl_Surface_Device_surface();
            assert!(!ptr.is_null());
            Self {
                _inner: ptr as *mut _,
            }
        }
    }

    fn push_current(new_current: &SvgFileSurface) {
        unsafe { Fl_Surface_Device_push_current(new_current._inner as *mut _) }
    }

    fn pop_current() {
        unsafe {
            Fl_Surface_Device_pop_current();
        }
    }
}

impl SvgFileSurface {
    /// Returns a new SvgFileSurface
    pub fn new<P: AsRef<path::Path>>(width: i32, height: i32, path: P) -> SvgFileSurface {
        let path = CString::safe_new(path.as_ref().to_str().unwrap());
        unsafe { 
            let ptr = Fl_SVG_File_Surface_new(width, height, path.as_ptr());
            assert!(!ptr.is_null());
            SvgFileSurface {
                _inner: ptr,
            }
        }
    }

    /// Sets the origin of the SvgFileSurface
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_SVG_File_Surface_origin(self._inner, x, y) }
    }

    /// Returns the width and height of the printable rect
    pub fn printable_rect(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_SVG_File_Surface_printable_rect(self._inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Draw a widget in an svg file surface
    /// the .svg file is not complete until the destructor was run
    pub fn draw<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_SVG_File_Surface_draw(
                self._inner,
                widget.as_widget_ptr() as *mut _,
                delta_x,
                delta_y,
            )
        }
    }

    /// draw a decorated window
    pub fn draw_decorated_window<W: WindowExt>(&self, win: &W, x_offset: i32, y_offset: i32) {
        unsafe {
            Fl_SVG_File_Surface_draw_decorated_window(
                self._inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }
}

impl Drop for SvgFileSurface {
    fn drop(&mut self) {
        unsafe { Fl_SVG_File_Surface_delete(self._inner) }
    }
}
