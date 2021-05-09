use crate::enums::ColorDepth;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::image::*;
use std::{
    ffi::CString,
    mem,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Wrapper around `Fl_Image`, used to wrap other image types
#[derive(ImageExt, Debug)]
pub struct Image {
    inner: *mut Fl_Image,
    refcount: AtomicUsize,
}

/// Creates a struct holding a shared image
#[derive(ImageExt, Debug)]
pub struct SharedImage {
    inner: *mut Fl_Shared_Image,
    refcount: AtomicUsize,
}

impl SharedImage {
    /// Loads a `SharedImage` from a path
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<SharedImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<SharedImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let x = Fl_Shared_Image_get(temp.as_ptr(), 0, 0);
            if x.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_Shared_Image_fail(x) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SharedImage {
                    inner: x,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads a `SharedImage` from an image
    /// # Errors
    /// Errors on unsupported `SharedImage` types
    pub fn from_image<I: ImageExt>(image: I) -> Result<SharedImage, FltkError> {
        unsafe {
            let x = Fl_Shared_Image_from_rgb(image.as_image_ptr() as *mut Fl_RGB_Image, 0);
            if x.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_Shared_Image_fail(x) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SharedImage {
                    inner: x,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }
}

/// Creates a struct holding a Jpeg image
#[derive(ImageExt, Debug)]
pub struct JpegImage {
    inner: *mut Fl_JPEG_Image,
    refcount: AtomicUsize,
}

impl JpegImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<JpegImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<JpegImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_JPEG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_JPEG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(JpegImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
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
                    Ok(JpegImage {
                        inner: x,
                        refcount: AtomicUsize::new(1),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding a PNG image
#[derive(ImageExt, Debug)]
pub struct PngImage {
    inner: *mut Fl_PNG_Image,
    refcount: AtomicUsize,
}

impl PngImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<PngImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<PngImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_PNG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_PNG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(PngImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
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
                    Ok(PngImage {
                        inner: x,
                        refcount: AtomicUsize::new(1),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding an SVG image
#[derive(ImageExt, Debug)]
pub struct SvgImage {
    inner: *mut Fl_SVG_Image,
    refcount: AtomicUsize,
}

impl SvgImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<SvgImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<SvgImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_SVG_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_SVG_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(SvgImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
    pub fn from_data(data: &str) -> Result<SvgImage, FltkError> {
        if data.is_empty() {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        } else {
            let data = CString::safe_new(data);
            unsafe {
                let x = Fl_SVG_Image_from(data.as_ptr());
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_SVG_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(SvgImage {
                        inner: x,
                        refcount: AtomicUsize::new(1),
                    })
                }
            }
        }
    }

    /// Rasterize an SvgImage
    pub fn normalize(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_SVG_Image_normalize(self.inner)
        }
    }
}

/// Creates a struct holding a BMP image
#[derive(ImageExt, Debug)]
pub struct BmpImage {
    inner: *mut Fl_BMP_Image,
    refcount: AtomicUsize,
}

impl BmpImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<BmpImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<BmpImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_BMP_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_BMP_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(BmpImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
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
                    Ok(BmpImage {
                        inner: x,
                        refcount: AtomicUsize::new(1),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding a GIF image
#[derive(ImageExt, Debug)]
pub struct GifImage {
    inner: *mut Fl_GIF_Image,
    refcount: AtomicUsize,
}

impl GifImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<GifImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<GifImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_GIF_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_GIF_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(GifImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
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
                    Ok(GifImage {
                        inner: x,
                        refcount: AtomicUsize::new(1),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding a XPM image
#[derive(ImageExt, Debug)]
pub struct XpmImage {
    inner: *mut Fl_XPM_Image,
    refcount: AtomicUsize,
}

impl XpmImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<XpmImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<XpmImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_XPM_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_XPM_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(XpmImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }
}

/// Creates a struct holding a XBM image
#[derive(ImageExt, Debug)]
pub struct XbmImage {
    inner: *mut Fl_XBM_Image,
    refcount: AtomicUsize,
}

impl XbmImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<XbmImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<XbmImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_XBM_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_XBM_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(XbmImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }
}

/// Creates a struct holding a PNM image
#[derive(ImageExt, Debug)]
pub struct PnmImage {
    inner: *mut Fl_PNM_Image,
    refcount: AtomicUsize,
}

impl PnmImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<PnmImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<PnmImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_PNM_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_PNM_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(PnmImage {
                    inner: image_ptr,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }
}

/// Creates a struct holding a tiled image
#[derive(ImageExt, Debug)]
pub struct TiledImage {
    inner: *mut Fl_Tiled_Image,
    refcount: AtomicUsize,
}

impl TiledImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn new<Img: ImageExt>(img: Img, w: i32, h: i32) -> TiledImage {
        unsafe {
            let ptr = Fl_Tiled_Image_new(img.as_image_ptr(), w, h);
            assert!(!ptr.is_null());
            TiledImage {
                inner: ptr,
                refcount: AtomicUsize::new(1),
            }
        }
    }
}

/// Creates a struct holding a pixmap image
#[derive(ImageExt, Debug)]
pub struct Pixmap {
    inner: *mut Fl_Pixmap,
    refcount: AtomicUsize,
}

impl Pixmap {
    /// Creates a new Pixmap image
    /// # Errors
    /// Errors on invalid or unsupported image format
    pub fn new(data: &[&str]) -> Result<Pixmap, FltkError> {
        let mut temp_file = std::env::temp_dir();
        temp_file.push("_internal_temp_fltk_file.xpm");
        let mut temp = String::from("/* XPM */\nstatic char *_613589117910[] = {\n");
        for elem in data {
            temp.push('\"');
            temp.push_str(elem);
            temp.push_str("\",\n");
        }
        temp.push_str("};\n");
        std::fs::write(&temp_file, temp)?;
        unsafe {
            let temp = CString::new(temp_file.to_string_lossy().as_bytes())?;
            let image_ptr = Fl_XPM_Image_new(temp.as_ptr());
            if image_ptr.is_null() {
                std::fs::remove_file(temp_file)?;
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_XPM_Image_fail(image_ptr) < 0 {
                    std::fs::remove_file(temp_file)?;
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                std::fs::remove_file(temp_file)?;
                Ok(Pixmap {
                    inner: image_ptr as _,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }
}

/// Creates a struct holding a raw RGB image
#[derive(ImageExt, Debug)]
pub struct RgbImage {
    inner: *mut Fl_RGB_Image,
    refcount: AtomicUsize,
}

impl RgbImage {
    /// Initializes a new raw `RgbImage`, copies the data and handles its lifetime.
    /// If you need to work with RGB data,
    /// # Errors
    /// Errors on invalid or unsupported image format
    pub fn new(data: &[u8], w: i32, h: i32, depth: ColorDepth) -> Result<RgbImage, FltkError> {
        let sz = w * h * depth as i32;
        if sz > data.len() as i32 {
            return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
        }
        unsafe {
            let img = Fl_RGB_Image_new(data.as_ptr(), w, h, depth as i32, 0);
            if img.is_null() || Fl_RGB_Image_fail(img) < 0 {
                Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
            } else {
                Ok(RgbImage {
                    inner: img,
                    refcount: AtomicUsize::new(1),
                })
            }
        }
    }

    /// Initializes a new raw `RgbImage` from shared data, doesn't handle the data's lifetime
    /// # Safety
    /// The data must be valid for the lifetime of the image
    /// # Errors
    /// Errors on invalid or unsupported image format
    pub unsafe fn from_data(
        data: &[u8],
        w: i32,
        h: i32,
        depth: ColorDepth,
    ) -> Result<RgbImage, FltkError> {
        let sz = w * h * depth as i32;
        if sz > data.len() as i32 {
            return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
        }
        let img = Fl_RGB_Image_from_data(data.as_ptr(), w, h, depth as i32, 0);
        if img.is_null() || Fl_RGB_Image_fail(img) < 0 {
            Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
        } else {
            Ok(RgbImage {
                inner: img,
                refcount: AtomicUsize::new(1),
            })
        }
    }

    /// Initializes a new raw `RgbImage`, copies the data and handles its lifetime.
    /// If you need to work with RGB data,
    /// # Errors
    /// Errors on invalid or unsupported image format
    /// # Safety
    /// Passing wrong line data can read to over or underflow
    pub unsafe fn new2(data: &[u8], w: i32, h: i32, depth: i32, line_data: i32) -> Result<RgbImage, FltkError> {
        let img = Fl_RGB_Image_new(data.as_ptr(), w, h, depth, line_data);
        if img.is_null() || Fl_RGB_Image_fail(img) < 0 {
            Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
        } else {
            Ok(RgbImage {
                inner: img,
                refcount: AtomicUsize::new(1),
            })
        }
    }

    /// Initializes a new raw `RgbImage` from shared data, doesn't handle the data's lifetime
    /// # Safety
    /// The data must be valid for the lifetime of the image
    /// # Errors
    /// Errors on invalid or unsupported image format
    /// # Safety
    /// Passing wrong line data can read to over or underflow
    pub unsafe fn from_data2(
        data: &[u8],
        w: i32,
        h: i32,
        depth: i32,
        line_data: i32
    ) -> Result<RgbImage, FltkError> {
        let img = Fl_RGB_Image_from_data(data.as_ptr(), w, h, depth, line_data);
        if img.is_null() || Fl_RGB_Image_fail(img) < 0 {
            Err(FltkError::Internal(FltkErrorKind::ImageFormatError))
        } else {
            Ok(RgbImage {
                inner: img,
                refcount: AtomicUsize::new(1),
            })
        }
    }

    /// Deconstructs a raw `RgbImage` into parts
    /// # Safety
    /// Destructures the image into its raw elements
    pub unsafe fn into_parts(self) -> (Vec<u8>, i32, i32) {
        let w = self.data_w();
        let h = self.data_h();
        (self.to_rgb_data(), w, h)
    }
}
