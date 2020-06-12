use crate::draw::*;
pub use crate::prelude::*;
use fltk_sys::image::*;
use std::{ffi::CString, mem, os::raw};

/// Wrapper around Fl_Image, used to wrap other image types
#[derive(ImageExt, Debug)]
pub struct Image {
    _inner: *mut Fl_Image,
}

// /// A conversion function for internal use
// impl<I: ImageExt> From<I> for Image {
//     fn from(s: I) -> Self {
//         let img: *mut Fl_Image = s.as_image_ptr();
//         Image { _inner: img }
//     }
// }

/// A conversion function for internal use
impl Image {
    /// Returns the internal pointer of Image
    pub unsafe fn as_ptr(&self) -> *mut Fl_Image {
        self._inner
    }

    /// Initialize an Image base from a raw pointer
    pub unsafe fn from_raw(ptr: *mut fltk_sys::image::Fl_Image) -> Self {
        Image { _inner: ptr }
    }

    /// Transforms an Image base into another Image
    pub unsafe fn into<I: ImageExt>(self) -> I {
        I::from_image_ptr(self._inner)
    }
}

/// Creates a struct holding a shared image
#[derive(ImageExt, Debug)]
pub struct SharedImage {
    _inner: *mut Fl_Shared_Image,
}

impl SharedImage {
    /// Loads a SharedImage from a path
    pub fn load(path: &std::path::Path) -> Result<SharedImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let x = Fl_Shared_Image_get(temp.as_ptr(), 0, 0);
            if x.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_Shared_Image_fail(x) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SharedImage { _inner: x })
            }
        }
    }

    /// Loads a SharedImage from an RgbImage
    pub fn from_rgb(rgb: RgbImage, own_it: bool) -> Result<SharedImage, FltkError> {
        unsafe {
            let x = Fl_Shared_Image_from_rgb(rgb._inner, own_it as i32);
            if x.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_Shared_Image_fail(x) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SharedImage { _inner: x })
            }
        }
    }
}

/// Creates a struct holding a Jpeg image
#[derive(ImageExt, Debug)]
pub struct JpegImage {
    _inner: *mut Fl_JPEG_Image,
}

impl JpegImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn load(path: &std::path::Path) -> Result<JpegImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let image_ptr = Fl_JPEG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
            } else {
                if Fl_JPEG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(JpegImage { _inner: image_ptr })
            }
        }
    }

    /// Loads the image from data/memory
    pub fn from_data(data: &[u8]) -> Result<JpegImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_JPEG_Image_from(data.as_ptr());
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_JPEG_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(JpegImage { _inner: x })
                }
            }
        }
    }
}

/// Creates a struct holding a PNG image
#[derive(ImageExt, Debug)]
pub struct PngImage {
    _inner: *mut Fl_PNG_Image,
}

impl PngImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn load(path: &std::path::Path) -> Result<PngImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let image_ptr = Fl_PNG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
            } else {
                if Fl_PNG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(PngImage { _inner: image_ptr })
            }
        }
    }

    /// Loads the image from data/memory
    pub fn from_data(data: &[u8]) -> Result<PngImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_PNG_Image_from(data.as_ptr(), data.len() as i32);
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_PNG_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(PngImage { _inner: x })
                }
            }
        }
    }
}

/// Creates a struct holding an SVG image
#[derive(ImageExt, Debug)]
pub struct SvgImage {
    _inner: *mut Fl_SVG_Image,
}

impl SvgImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn load(path: &std::path::Path) -> Result<SvgImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let image_ptr = Fl_SVG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
            } else {
                if Fl_SVG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SvgImage { _inner: image_ptr })
            }
        }
    }

    /// Loads the image from data/memory
    pub fn from_data(data: &str) -> Result<SvgImage, FltkError> {
        if data.is_empty() {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        } else {
            let data = CString::new(data).unwrap();
            unsafe {
                let x = Fl_SVG_Image_from(data.as_ptr());
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_SVG_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(SvgImage { _inner: x })
                }
            }
        }
    }
}

/// Creates a struct holding a BMP image
#[derive(ImageExt, Debug)]
pub struct BmpImage {
    _inner: *mut Fl_BMP_Image,
}

impl BmpImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn load(path: &std::path::Path) -> Result<BmpImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let image_ptr = Fl_BMP_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
            } else {
                if Fl_BMP_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(BmpImage { _inner: image_ptr })
            }
        }
    }

    /// Loads the image from data/memory
    pub fn from_data(data: &[u8]) -> Result<BmpImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_BMP_Image_from(data.as_ptr());
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_BMP_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(BmpImage { _inner: x })
                }
            }
        }
    }
}

/// Creates a struct holding a GIF image
#[derive(ImageExt, Debug)]
pub struct GifImage {
    _inner: *mut Fl_GIF_Image,
}

impl GifImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn load(path: &std::path::Path) -> Result<GifImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().unwrap();
            let temp = CString::new(temp)?;
            let image_ptr = Fl_GIF_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
            } else {
                if Fl_GIF_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(GifImage { _inner: image_ptr })
            }
        }
    }

    /// Loads the image from data/memory
    pub fn from_data(data: &[u8]) -> Result<GifImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_GIF_Image_from(data.as_ptr());
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_GIF_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(GifImage { _inner: x })
                }
            }
        }
    }
}

/// Creates a struct holding a raw RGB image
#[derive(ImageExt, Debug)]
pub struct RgbImage {
    _inner: *mut Fl_RGB_Image,
}

impl RgbImage {
    /// Initializes a new raw RgbImage
    pub fn new(data: &Vec<u8>, w: i32, h: i32, depth: u32) -> Result<RgbImage, FltkError> {
        if depth > 4 {
            return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
        }
        let mut sz = w * h;
        if depth > 0 {
            sz = sz * depth as i32;
        }
        if sz > data.len() as i32 {
            return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
        }
        let img = unsafe { Fl_RGB_Image_new(data.as_ptr(), w, h, depth as i32) };
        if img.is_null() {
            return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
        } else {
            unsafe {
                if Fl_RGB_Image_fail(img) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
            }
            Ok(RgbImage { _inner: img })
        }
    }

    /// Deconstructs a raw RgbImage into parts
    pub fn into_parts(self) -> (Vec<u8>, i32, i32) {
        let w = self.width();
        let h = self.height();
        (self.to_rgb(), w, h)
    }

    /// Transforms the RgbImage to a PngImage
    pub fn into_png_image(self) -> Result<PngImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.png");
        let _ = write_to_png_file(self, &path)?;
        let ret = PngImage::load(&path)?.copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }

    /// Transforms the RgbImage to a JpegImage
    pub fn into_jpg_image(self) -> Result<JpegImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.jpg");
        let _ = write_to_jpg_file(self, &path)?;
        let ret = JpegImage::load(&path)?.copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }

    /// Transforms the RgbImage to a BmpImage
    pub fn into_bmp_image(self) -> Result<BmpImage, FltkError> {
        let path = std::path::PathBuf::from("_internal_temp_fltk_file.bmp");
        let _ = write_to_bmp_file(self, &path)?;
        let ret = BmpImage::load(&path)?.copy();
        std::fs::remove_file(&path)?;
        Ok(ret)
    }
}
