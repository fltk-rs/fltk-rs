use crate::enums::ColorDepth;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::image::*;
use std::{ffi::CString, mem, sync::atomic::AtomicUsize};

/// Wrapper around `Fl_Image`, used to wrap other image types
#[derive(Debug)]
pub struct Image {
    inner: *mut Fl_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(Image, Fl_Image);

/// Creates a struct holding a shared image
#[derive(Debug)]
pub struct SharedImage {
    inner: *mut Fl_Shared_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(SharedImage, Fl_Shared_Image);

impl SharedImage {
    /// Loads a `SharedImage` from a path
    /// # Errors
    /// Errors on non-existent path or invalid format, also errors if app::App was not initialized
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
            assert!(!image.was_deleted());
            let x = Fl_Shared_Image_from_rgb(image.as_image_ptr() as *mut Fl_RGB_Image, 1);
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
#[derive(Debug)]
pub struct JpegImage {
    inner: *mut Fl_JPEG_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(JpegImage, Fl_JPEG_Image);

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
#[derive(Debug)]
pub struct PngImage {
    inner: *mut Fl_PNG_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(PngImage, Fl_PNG_Image);

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
#[derive(Debug)]
pub struct SvgImage {
    inner: *mut Fl_SVG_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(SvgImage, Fl_SVG_Image);

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
        unsafe { Fl_SVG_Image_normalize(self.inner) }
    }
}

/// Creates a struct holding a BMP image
#[derive(Debug)]
pub struct BmpImage {
    inner: *mut Fl_BMP_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(BmpImage, Fl_BMP_Image);

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
                let x = Fl_BMP_Image_from(data.as_ptr(), data.len() as _);
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
#[derive(Debug)]
pub struct GifImage {
    inner: *mut Fl_GIF_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(GifImage, Fl_GIF_Image);

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
                let x = Fl_GIF_Image_from(data.as_ptr(), data.len() as _);
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
#[derive(Debug)]
pub struct XpmImage {
    inner: *mut Fl_XPM_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(XpmImage, Fl_XPM_Image);

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
#[derive(Debug)]
pub struct XbmImage {
    inner: *mut Fl_XBM_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(XbmImage, Fl_XBM_Image);

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
#[derive(Debug)]
pub struct PnmImage {
    inner: *mut Fl_PNM_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(PnmImage, Fl_PNM_Image);

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
#[derive(Debug)]
pub struct TiledImage {
    inner: *mut Fl_Tiled_Image,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(TiledImage, Fl_Tiled_Image);

impl TiledImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn new<Img: ImageExt>(mut img: Img, w: i32, h: i32) -> TiledImage {
        unsafe {
            assert!(!img.was_deleted());
            img.increment_arc();
            let ptr = Fl_Tiled_Image_new(img.as_image_ptr(), w, h);
            assert!(!ptr.is_null());
            TiledImage {
                inner: ptr,
                refcount: AtomicUsize::new(2),
            }
        }
    }
}

/// Creates a struct holding a pixmap image
#[derive(Debug)]
pub struct Pixmap {
    inner: *mut Fl_Pixmap,
    refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(Pixmap, Fl_Pixmap);

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

/// The scaling algorithm to use for RGB images
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RgbScaling {
    /// Default RGB image scaling algorithm
    Nearest = 0,
    /// More accurate, but slower RGB image scaling algorithm
    Bilinear,
}

/// Creates a struct holding a raw RGB image
#[derive(Debug)]
pub struct RgbImage {
    pub(crate) inner: *mut Fl_RGB_Image,
    pub(crate) refcount: AtomicUsize,
}

crate::macros::image::impl_image_ext!(RgbImage, Fl_RGB_Image);

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
    pub unsafe fn new2(
        data: &[u8],
        w: i32,
        h: i32,
        depth: i32,
        line_data: i32,
    ) -> Result<RgbImage, FltkError> {
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
        line_data: i32,
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

    /// Creates an RgbImage from a pixmap
    pub fn from_pixmap(image: &Pixmap) -> RgbImage {
        unsafe { RgbImage::from_image_ptr(Fl_RGB_Image_from_pixmap(image.inner as _) as _) }
    }

    /// Deconstructs a raw `RgbImage` into parts
    /// # Safety
    /// Destructures the image into its raw elements
    pub unsafe fn into_parts(self) -> (Vec<u8>, i32, i32) {
        let w = self.data_w();
        let h = self.data_h();
        (self.to_rgb_data(), w, h)
    }

    /// Convert from one ColorDepth to another ColorDepth
    pub fn convert(&self, new_depth: ColorDepth) -> Result<RgbImage, FltkError> {
        let depth = self.depth() as i32;
        let new_depth = new_depth as i32;
        if depth == new_depth {
            Ok(self.copy())
        } else {
            let w = self.w();
            let h = self.h();
            let mut data = self.to_rgb_data();
            let mut temp = Vec::new();
            match depth {
                1 => match new_depth {
                    2 => {
                        for i in data {
                            temp.push(i);
                            temp.push(255);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    3 => {
                        for i in data {
                            temp.push(i);
                            temp.push(i);
                            temp.push(i);
                        }
                        assert!(temp.len() as i32 == w * h * 3);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgb8)
                    }
                    4 => {
                        for i in data {
                            temp.push(i);
                            temp.push(i);
                            temp.push(i);
                            temp.push(255);
                        }
                        assert!(temp.len() as i32 == w * h * 4);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgba8)
                    }
                    _ => unreachable!(),
                },
                2 => match new_depth {
                    1 => {
                        for (i, item) in data.iter().enumerate() {
                            if i % 2 == 0 {
                                temp.push(*item);
                            } else {
                                // skip
                            }
                        }
                        assert!(temp.len() as i32 == w * h);
                        RgbImage::new(&temp, w, h, ColorDepth::L8)
                    }
                    3 => {
                        for (i, item) in data.iter().enumerate() {
                            if i % 2 == 0 {
                                temp.push(*item);
                                temp.push(*item);
                                temp.push(*item);
                            } else {
                                // skip
                            }
                        }
                        assert!(temp.len() as i32 == w * h * 3);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgb8)
                    }
                    4 => {
                        for (i, item) in data.iter().enumerate() {
                            temp.push(*item);
                            if i % 2 == 0 {
                                temp.push(*item);
                                temp.push(*item);
                            }
                        }
                        assert!(temp.len() as i32 == w * h * 4);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgba8)
                    }
                    _ => unreachable!(),
                },
                3 => match new_depth {
                    1 => {
                        for (_, pixel) in data.chunks_exact_mut(3).enumerate() {
                            temp.push(
                                (pixel[0] as f64 * 0.299
                                    + pixel[1] as f64 * 0.587
                                    + pixel[2] as f64 * 0.114)
                                    as u8,
                            );
                        }
                        assert!(temp.len() as i32 == w * h);
                        RgbImage::new(&temp, w, h, ColorDepth::L8)
                    }
                    2 => {
                        for (_, pixel) in data.chunks_exact_mut(3).enumerate() {
                            temp.push(
                                (pixel[0] as f64 * 0.299
                                    + pixel[1] as f64 * 0.587
                                    + pixel[2] as f64 * 0.114)
                                    as u8,
                            );
                            temp.push(255);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    4 => {
                        for (_, pixel) in data.chunks_exact_mut(3).enumerate() {
                            temp.push(pixel[0]);
                            temp.push(pixel[1]);
                            temp.push(pixel[2]);
                            temp.push(255);
                        }
                        assert!(temp.len() as i32 == w * h * 4);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgba8)
                    }
                    _ => unreachable!(),
                },
                4 => match new_depth {
                    1 => {
                        for (_, pixel) in data.chunks_exact_mut(4).enumerate() {
                            temp.push(
                                (pixel[0] as f64 * 0.299
                                    + pixel[1] as f64 * 0.587
                                    + pixel[2] as f64 * 0.114)
                                    as u8,
                            );
                        }
                        assert!(temp.len() as i32 == w * h);
                        RgbImage::new(&temp, w, h, ColorDepth::L8)
                    }
                    2 => {
                        for (_, pixel) in data.chunks_exact_mut(4).enumerate() {
                            temp.push(
                                (pixel[0] as f64 * 0.299
                                    + pixel[1] as f64 * 0.587
                                    + pixel[2] as f64 * 0.114)
                                    as u8,
                            );
                            temp.push(pixel[3]);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    3 => {
                        for (_, pixel) in data.chunks_exact_mut(4).enumerate() {
                            temp.push(pixel[0]);
                            temp.push(pixel[1]);
                            temp.push(pixel[2]);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    /// Sets the scaling algorithm
    pub fn set_scaling_algorithm(algorithm: RgbScaling) {
        unsafe { Fl_Image_set_scaling_algorithm(algorithm as i32) }
    }

    /// Gets the scaling algorithm
    pub fn scaling_algorithm() -> RgbScaling {
        unsafe { mem::transmute(Fl_Image_scaling_algorithm()) }
    }
}
