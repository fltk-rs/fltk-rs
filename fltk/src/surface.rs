use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::surface::*;
use std::ffi::CString;
use std::path;

/// An image surface object.
/// Example usage:
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// let but = button::Button::new(0, 0, 80, 40, "Click");
/// let sur = surface::ImageSurface::new(but.width(), but.height(), false);
/// surface::ImageSurface::push_current(&sur);
/// draw::set_draw_color(enums::Color::White);
/// draw::draw_rectf(0, 0, but.width(), but.height());
/// sur.draw(&but, 0, 0);
/// let img = sur.image().unwrap();
/// surface::ImageSurface::pop_current();
/// ```
pub struct ImageSurface {
    inner: *mut Fl_Image_Surface,
}

impl SurfaceDevice for ImageSurface {
    fn is_current(&self) -> bool {
        unsafe { Fl_Surface_Device_is_current(self.inner as *mut _) != 0 }
    }

    fn surface() -> Self {
        unsafe {
            let ptr = Fl_Surface_Device_surface();
            assert!(!ptr.is_null());
            Self {
                inner: ptr as *mut _,
            }
        }
    }

    fn push_current(new_current: &ImageSurface) {
        unsafe { Fl_Surface_Device_push_current(new_current.inner as *mut _) }
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
            ImageSurface { inner: ptr }
        }
    }

    /// Gets the image of an image surface as an rgb image
    pub fn image(&self) -> Option<crate::image::RgbImage> {
        unsafe {
            let ptr = Fl_Image_Surface_image(self.inner);
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
            let ptr = Fl_Image_Surface_highres_image(self.inner);
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
            Fl_Image_Surface_origin(self.inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Set the origin coordinates of an image surface
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_Image_Surface_set_origin(self.inner, x, y) }
    }

    /// Rescale an image surface
    pub fn rescale(&mut self) {
        unsafe { Fl_Image_Surface_rescale(self.inner) }
    }

    /// Draw a widget on the image surface
    pub fn draw<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_Image_Surface_draw(
                self.inner,
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
                self.inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }
}

impl Drop for ImageSurface {
    fn drop(&mut self) {
        unsafe { Fl_Image_Surface_delete(self.inner) }
    }
}

/// An SVG image surface object
/// Example usage:
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// let but = button::Button::new(0, 0, 80, 40, "Click");
/// // We need the destructor of SvgFileSurface to actually create the image
/// {
///     let sur = surface::SvgFileSurface::new(but.width(), but.height(), "temp.svg");
///     surface::SvgFileSurface::push_current(&sur);
///     draw::set_draw_color(enums::Color::White);
///     draw::draw_rectf(0, 0, but.width(), but.height());
///     sur.draw(&but, 0, 0);
///     surface::SvgFileSurface::pop_current();
/// }
/// ```
pub struct SvgFileSurface {
    inner: *mut Fl_SVG_File_Surface,
}

impl SurfaceDevice for SvgFileSurface {
    fn is_current(&self) -> bool {
        unsafe { Fl_Surface_Device_is_current(self.inner as *mut _) != 0 }
    }

    fn surface() -> Self {
        unsafe {
            let ptr = Fl_Surface_Device_surface();
            assert!(!ptr.is_null());
            Self {
                inner: ptr as *mut _,
            }
        }
    }

    fn push_current(new_current: &SvgFileSurface) {
        unsafe { Fl_Surface_Device_push_current(new_current.inner as *mut _) }
    }

    fn pop_current() {
        unsafe {
            Fl_Surface_Device_pop_current();
        }
    }
}

impl SvgFileSurface {
    /// Returns a new `SvgFileSurface`
    pub fn new<P: AsRef<path::Path>>(width: i32, height: i32, path: P) -> SvgFileSurface {
        let path = CString::safe_new(path.as_ref().to_str().unwrap());
        unsafe {
            let ptr = Fl_SVG_File_Surface_new(width, height, path.as_ptr());
            assert!(!ptr.is_null());
            SvgFileSurface { inner: ptr }
        }
    }

    /// Sets the origin of the `SvgFileSurface`
    pub fn set_origin(&mut self, x: i32, y: i32) {
        unsafe { Fl_SVG_File_Surface_origin(self.inner, x, y) }
    }

    /// Returns the width and height of the printable rect
    pub fn printable_rect(&self) -> (i32, i32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            Fl_SVG_File_Surface_printable_rect(self.inner, &mut x, &mut y);
            (x, y)
        }
    }

    /// Draw a widget in an svg file surface.
    /// The .svg file is not complete until the destructor was run
    pub fn draw<W: WidgetExt>(&self, widget: &W, delta_x: i32, delta_y: i32) {
        unsafe {
            Fl_SVG_File_Surface_draw(
                self.inner,
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
                self.inner,
                win.as_widget_ptr() as *mut _,
                x_offset,
                y_offset,
            )
        }
    }
}

impl Drop for SvgFileSurface {
    fn drop(&mut self) {
        unsafe { Fl_SVG_File_Surface_delete(self.inner) }
    }
}
