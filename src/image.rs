pub use crate::prelude::*;
use fltk_sys::image::*;
use std::os::raw;

#[derive(ImageTrait, Debug, Clone)]
pub struct JpegImage {
    _inner: *mut Fl_JPEG_Image,
}

#[derive(ImageTrait, Debug, Clone)]
pub struct PngImage {
    _inner: *mut Fl_PNG_Image,
}

#[derive(ImageTrait, Debug, Clone)]
pub struct SvgImage {
    _inner: *mut Fl_SVG_Image,
}

#[derive(ImageTrait, Debug, Clone)]
pub struct BmpImage {
    _inner: *mut Fl_BMP_Image,
}

#[derive(ImageTrait, Debug, Clone)]
pub struct GifImage {
    _inner: *mut Fl_GIF_Image,
}