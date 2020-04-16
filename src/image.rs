pub use crate::prelude::*;
use crate::draw::*;
use fltk_sys::image::*;
use std::{ffi::CString, mem, os::raw};

/// Wrapper around Fl_Image, used to wrap other image types
#[derive(Debug, Clone)]
pub struct Image {
    _inner: *mut Fl_Image,
}

/// A conversion function for internal use
impl<I: ImageTrait> From<I> for Image {
    fn from(s: I) -> Self {
        let img: *mut Fl_Image = s.as_image_ptr();
        Image { _inner: img }
    }
}

/// A conversion function for internal use
impl Image {
    /// Returns the internal pointer of Image
    pub fn as_ptr(&mut self) -> *mut Fl_Image {
        self._inner
    }
    /// Initialize an Image base from a raw pointer
    pub fn from_raw(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
        Image { _inner: ptr }
    }
    /// Transforms an Image base into another Image
    pub fn downcast_into<I: ImageTrait>(&mut self) -> I {
        I::from_image_ptr(self._inner)
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

/// Creates a struct holding a raw RGB image
#[derive(Debug, Clone)]
pub struct RgbImage {
    _inner: Vec<u8>,
    _w: i32,
    _h: i32,
}

impl RgbImage {
    /// Initializes a new raw RgbImage
    pub fn new(data: Vec<u8>, w: i32, h: i32) -> RgbImage {
        RgbImage {
            _inner: data,
            _w: w,
            _h: h,
        }
    }
    /// Deconstructs a raw RgbImage into parts
    pub fn into_parts(self) -> (Vec<u8>, i32, i32) {
        (self._inner, self._w, self._h)
    }
    /// Transforms the RgbImage to a PngImage
    pub fn into_png_image(self) -> Result<PngImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.png");
        let _ = write_to_png_file(self, &path)?;
        let ret = PngImage::new(&path).copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }
    /// Transforms the RgbImage to a JpegImage
    pub fn into_jpg_image(self) -> Result<JpegImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.jpg");
        let _ = write_to_jpg_file(self, &path)?;
        let ret = JpegImage::new(&path).copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }
    /// Transforms the RgbImage to a BmpImage
    pub fn into_bmp_image(&self) -> Result<BmpImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.bmp");
        let _ = write_to_bmp_file(self.clone(), &path)?;
        let ret = BmpImage::new(&path).copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }
}