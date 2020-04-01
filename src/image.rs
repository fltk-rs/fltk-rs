pub use crate::prelude::*;
use fltk_sys::image::*;
use std::{ffi::CString, mem, os::raw};

/// Wrapper around Fl_Image, used to wrap other image types
#[derive(Debug, Clone)]
pub struct Image {
    _inner: *mut Fl_Image,
}

impl Image {
    pub fn as_ptr(&mut self) -> *mut Fl_Image {
        self._inner
    }
}

/// A conversion function for internal use
impl<I: ImageTrait> From<I> for Image {
    fn from(s: I) -> Self {
        let img: *mut Fl_Image = s.as_image_ptr();
        Image {
            _inner: img,
        }
    }
}

/// Creates a struct holding a Jpeg image
#[derive(ImageTrait, Debug, Clone)]
pub struct JpegImage {
    _inner: *mut Fl_JPEG_Image,
}

/// Creates a struct holding a PNG image
#[derive(ImageTrait, Debug, Clone)]
pub struct PngImage {
    _inner: *mut Fl_PNG_Image,
}

/// Creates a struct holding an SVG image
#[derive(ImageTrait, Debug, Clone)]
pub struct SvgImage {
    _inner: *mut Fl_SVG_Image,
}

/// Creates a struct holding a BMP image
#[derive(ImageTrait, Debug, Clone)]
pub struct BmpImage {
    _inner: *mut Fl_BMP_Image,
}

/// Creates a struct holding a GIF image
#[derive(ImageTrait, Debug, Clone)]
pub struct GifImage {
    _inner: *mut Fl_GIF_Image,
}