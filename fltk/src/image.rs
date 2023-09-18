use crate::enums::ColorDepth;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::image::*;
use std::{ffi::CString, mem};

#[cfg(feature = "single-threaded")]
type ImageRC<T> = std::rc::Rc<T>;

#[cfg(not(feature = "single-threaded"))]
type ImageRC<T> = std::sync::Arc<T>;

/// The scaling algorithm to use for raster images
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RgbScaling {
    /// Default RGB image scaling algorithm
    Nearest = 0,
    /// More accurate, but slower RGB image scaling algorithm
    Bilinear,
}

/// Wrapper around `Fl_Image`, used to wrap other image types
#[derive(Debug)]
pub struct Image {
    inner: ImageRC<*mut Fl_Image>,
}

crate::macros::image::impl_image_ext!(Image, Fl_Image);

impl Image {
    /// Sets the scaling algorithm
    pub fn set_scaling_algorithm(algorithm: RgbScaling) {
        unsafe { Fl_Image_set_scaling_algorithm(algorithm as i32) }
    }

    /// Gets the scaling algorithm
    pub fn scaling_algorithm() -> RgbScaling {
        unsafe { mem::transmute(Fl_Image_scaling_algorithm()) }
    }
}

/// Creates a struct holding a shared image
#[derive(Debug)]
pub struct SharedImage {
    inner: ImageRC<*mut Fl_Shared_Image>,
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
                    inner: ImageRC::from(x),
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
                    inner: ImageRC::from(x),
                })
            }
        }
    }
}

/// Creates a struct holding a Jpeg image
#[derive(Debug)]
pub struct JpegImage {
    inner: ImageRC<*mut Fl_JPEG_Image>,
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
                    inner: ImageRC::from(image_ptr),
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
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding a PNG image
#[derive(Debug)]
pub struct PngImage {
    inner: ImageRC<*mut Fl_PNG_Image>,
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
                    inner: ImageRC::from(image_ptr),
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
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding an SVG image
#[derive(Debug)]
pub struct SvgImage {
    inner: ImageRC<*mut Fl_SVG_Image>,
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
                    inner: ImageRC::from(image_ptr),
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
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }

    /// Rasterize an SvgImage
    pub fn normalize(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_SVG_Image_normalize(*self.inner) }
    }
}

/// Creates a struct holding a BMP image
#[derive(Debug)]
pub struct BmpImage {
    inner: ImageRC<*mut Fl_BMP_Image>,
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
                    inner: ImageRC::from(image_ptr),
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
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }
}

/// Creates a struct holding a GIF image
#[derive(Debug)]
pub struct GifImage {
    inner: ImageRC<*mut Fl_GIF_Image>,
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
                    inner: ImageRC::from(image_ptr),
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
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }
}

bitflags::bitflags! {
    /// Defines AnimGifImage flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AnimGifImageFlags: u16 {
        /// No Event
        const None = 0;
        /// This flag indicates to the loader that it should not start
        /// the animation immediately after successful load, which is
        /// the default.
        /// It can be started later using the \ref start() method.
        const DONT_START = 1;
        /// This flag indicates to the loader that it should not
        /// resize the canvas widget of the animation to the dimensions
        /// of the animation, which is the default.
        /// Needed for special use cases.
        const DONT_RESIZE_CANVAS = 2;
        /// This flag indicates to the loader that it should not
        /// set the animation as \ref image() member of the canvas widget,
        /// which is the default.
        /// Needed for special use cases.
        const DONT_SET_AS_IMAGE = 4;
        /// Often frames change just a small area of the animation canvas.
        /// This flag indicates to the loader to try using less memory
        /// by storing frame data not as canvas-sized images but use the
        /// sizes defined in the GIF file.
        /// The drawbacks are higher cpu usage during playback and maybe
        //// minor artefacts when resized.\
        const OPTIMIZE_MEMORY = 8;
        /// This flag can be used to print informations about the decoding process to the console.
        const LOG_FLAG = 64;
        /// This flag can be used to print even more informations about the decoding process to the console.
        const DEBUG_FLAG = 128;
    }
}

/// Creates a struct holding an animated GIF image
#[derive(Debug)]
pub struct AnimGifImage {
    inner: ImageRC<*mut Fl_Anim_GIF_Image>,
}

crate::macros::image::impl_image_ext!(AnimGifImage, Fl_Anim_GIF_Image);

impl AnimGifImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>, W: WidgetExt>(
        path: P,
        w: &mut W,
        flags: AnimGifImageFlags,
    ) -> Result<AnimGifImage, FltkError> {
        Self::load_(path.as_ref(), w, flags)
    }

    fn load_<W: WidgetExt>(
        path: &std::path::Path,
        w: &mut W,
        flags: AnimGifImageFlags,
    ) -> Result<AnimGifImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr =
                Fl_Anim_GIF_Image_new(temp.as_ptr(), w.as_widget_ptr() as _, flags.bits());
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_Anim_GIF_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(AnimGifImage {
                    inner: ImageRC::from(image_ptr),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
    pub fn from_data<W: WidgetExt>(
        data: &[u8],
        w: &mut W,
        flags: AnimGifImageFlags,
    ) -> Result<AnimGifImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_Anim_GIF_Image_from(
                    std::ptr::null(),
                    data.as_ptr(),
                    data.len() as _,
                    w.as_widget_ptr() as _,
                    flags.bits(),
                );
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_Anim_GIF_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(AnimGifImage {
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }

    /// Returns the delay of the frame
    pub fn delay(&self, frame: i32) -> f64 {
        unsafe { Fl_Anim_GIF_Image_delay(*self.inner, frame) }
    }

    /// Sets the delay of the frame
    pub fn set_delay(&mut self, frame: i32, delay: f64) {
        unsafe { Fl_Anim_GIF_Image_set_delay(*self.inner, frame, delay) }
    }

    /// Returns whether the Gif is animated
    pub fn is_animated(&self) -> bool {
        unsafe { Fl_Anim_GIF_Image_is_animated(*self.inner) != 0 }
    }

    /// Sets the animation speed
    pub fn set_speed(&mut self, speed: f64) {
        unsafe { Fl_Anim_GIF_Image_set_speed(*self.inner, speed) }
    }

    /// Returns the animation speed
    pub fn speed(&mut self) -> f64 {
        unsafe { Fl_Anim_GIF_Image_speed(*self.inner) }
    }

    /// Starts the animation
    pub fn start(&mut self) -> bool {
        unsafe { Fl_Anim_GIF_Image_start(*self.inner) != 0 }
    }

    /// Stops the animation
    pub fn stop(&mut self) -> bool {
        unsafe { Fl_Anim_GIF_Image_stop(*self.inner) != 0 }
    }

    /// Show the next frame if the animation is stopped. Errors if the Gif has no more frames
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<(), FltkError> {
        unsafe {
            if Fl_Anim_GIF_Image_next(*self.inner) != 0 {
                Ok(())
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }

    /// Returns whether the Gif is playing
    pub fn playing(&self) -> bool {
        unsafe { Fl_Anim_GIF_Image_playing(*self.inner) != 0 }
    }
}

/// Creates a struct holding a XPM image
#[derive(Debug)]
pub struct XpmImage {
    inner: ImageRC<*mut Fl_XPM_Image>,
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
                    inner: ImageRC::from(image_ptr),
                })
            }
        }
    }
}

/// Creates a struct holding a XBM image
#[derive(Debug)]
pub struct XbmImage {
    inner: ImageRC<*mut Fl_XBM_Image>,
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
                    inner: ImageRC::from(image_ptr),
                })
            }
        }
    }
}

/// Creates a struct holding a PNM image
#[derive(Debug)]
pub struct PnmImage {
    inner: ImageRC<*mut Fl_PNM_Image>,
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
                    inner: ImageRC::from(image_ptr),
                })
            }
        }
    }
}

/// Creates a struct holding a tiled image
#[derive(Debug)]
pub struct TiledImage {
    inner: ImageRC<*mut Fl_Tiled_Image>,
}

crate::macros::image::impl_image_ext!(TiledImage, Fl_Tiled_Image);

impl TiledImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    pub fn new<Img: ImageExt>(img: Img, w: i32, h: i32) -> TiledImage {
        unsafe {
            assert!(!img.was_deleted());
            let ptr = Fl_Tiled_Image_new(img.as_image_ptr(), w, h);
            assert!(!ptr.is_null());
            TiledImage {
                inner: ImageRC::from(ptr),
            }
        }
    }
}

/// Creates a struct holding a pixmap image
#[derive(Debug)]
pub struct Pixmap {
    inner: ImageRC<*mut Fl_Pixmap>,
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
                    inner: ImageRC::from(image_ptr as *mut Fl_Pixmap),
                })
            }
        }
    }
}

/// Creates a struct holding a raw RGB image
#[derive(Debug)]
pub struct RgbImage {
    pub(crate) inner: ImageRC<*mut Fl_RGB_Image>,
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
                    inner: ImageRC::from(img),
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
                inner: ImageRC::from(img),
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
                inner: ImageRC::from(img),
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
                inner: ImageRC::from(img),
            })
        }
    }

    /// Creates an RgbImage from a pixmap
    pub fn from_pixmap(image: &Pixmap) -> RgbImage {
        unsafe { RgbImage::from_image_ptr(Fl_RGB_Image_from_pixmap(*image.inner as _) as _) }
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
            let data = self.to_rgb_data();
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
                        for (_, pixel) in data.chunks_exact(3).enumerate() {
                            temp.push(
                                (pixel[0] as f32 * 0.299
                                    + pixel[1] as f32 * 0.587
                                    + pixel[2] as f32 * 0.114)
                                    as u8,
                            );
                        }
                        assert!(temp.len() as i32 == w * h);
                        RgbImage::new(&temp, w, h, ColorDepth::L8)
                    }
                    2 => {
                        for (_, pixel) in data.chunks_exact(3).enumerate() {
                            temp.push(
                                (pixel[0] as f32 * 0.299
                                    + pixel[1] as f32 * 0.587
                                    + pixel[2] as f32 * 0.114)
                                    as u8,
                            );
                            temp.push(255);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    4 => {
                        for (_, pixel) in data.chunks_exact(3).enumerate() {
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
                        for (_, pixel) in data.chunks_exact(4).enumerate() {
                            temp.push(
                                (pixel[0] as f32 * 0.299
                                    + pixel[1] as f32 * 0.587
                                    + pixel[2] as f32 * 0.114)
                                    as u8,
                            );
                        }
                        assert!(temp.len() as i32 == w * h);
                        RgbImage::new(&temp, w, h, ColorDepth::L8)
                    }
                    2 => {
                        for (_, pixel) in data.chunks_exact(4).enumerate() {
                            temp.push(
                                (pixel[0] as f32 * 0.299
                                    + pixel[1] as f32 * 0.587
                                    + pixel[2] as f32 * 0.114)
                                    as u8,
                            );
                            temp.push(pixel[3]);
                        }
                        assert!(temp.len() as i32 == w * h * 2);
                        RgbImage::new(&temp, w, h, ColorDepth::La8)
                    }
                    3 => {
                        for (_, pixel) in data.chunks_exact(4).enumerate() {
                            temp.push(pixel[0]);
                            temp.push(pixel[1]);
                            temp.push(pixel[2]);
                        }
                        assert!(temp.len() as i32 == w * h * 3);
                        RgbImage::new(&temp, w, h, ColorDepth::Rgb8)
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }

    /// Convert black pixels to transparent
    pub fn convert_transparent(&self) -> Result<RgbImage, FltkError> {
        let depth = self.depth() as i32;
        if depth == 2 || depth == 4 {
            Ok(self.copy())
        } else {
            let w = self.w();
            let h = self.h();
            let data = self.to_rgb_data();
            let mut temp = Vec::new();
            match depth {
                1 => {
                    for i in data {
                        temp.push(i);
                        if i == 0 {
                            temp.push(0);
                        } else {
                            temp.push(255);
                        }
                    }
                    assert!(temp.len() as i32 == w * h * 2);
                    RgbImage::new(&temp, w, h, ColorDepth::La8)
                }
                3 => {
                    for (_, pixel) in data.chunks_exact(3).enumerate() {
                        let r = pixel[0];
                        let g = pixel[1];
                        let b = pixel[2];
                        temp.push(r);
                        temp.push(g);
                        temp.push(b);
                        if r == 0 && g == 0 && b == 0 {
                            temp.push(0);
                        } else {
                            temp.push(255);
                        }
                    }
                    assert!(temp.len() as i32 == w * h * 4);
                    RgbImage::new(&temp, w, h, ColorDepth::Rgba8)
                }
                _ => unreachable!(),
            }
        }
    }

    /// Blur the image
    /// Adapted from <https://www.cairographics.org/cookbook/blur.c/>
    pub fn blur(&self, radius: u32) -> Result<RgbImage, FltkError> {
        assert!(self.depth() == ColorDepth::Rgba8);
        let radius = radius as i32;
        let mut src = self.to_rgb_data();
        let width = self.w();
        let height = self.h();
        let depth = self.depth();
        let mut dst = vec![0u8; (width * height * depth as i32) as usize];
        let mut kernel = [0u8; 17];
        let size = kernel.len() as i32;
        let half = size / 2;
        let src_stride = width * depth as i32;
        let dst_stride = src_stride;

        let mut x: u32;
        let mut y: u32;
        let mut z: u32;
        let mut w: u32;
        let mut p: u32;

        let mut a: u32 = 0;
        for i in 0..size {
            let f = i - half;
            let f = f as f32;
            kernel[i as usize] = ((-f * f / 30.0).exp() * 80.0) as u8;
            a += kernel[i as usize] as u32;
        }

        // Horizontally blur from surface -> temp
        for i in 0..height {
            let s: &[u32] = unsafe { src[(i * src_stride) as usize..].align_to::<u32>().1 };
            let d: &mut [u32] = unsafe { dst[(i * dst_stride) as usize..].align_to_mut::<u32>().1 };
            for j in 0..width {
                if radius < j && j < width - radius {
                    let j = j as usize;
                    d[j] = s[j];
                    continue;
                }

                x = 0;
                y = 0;
                z = 0;
                w = 0;
                for k in 0..size {
                    if j - half + k < 0 || j - half + k >= width {
                        continue;
                    }

                    p = s[(j - half + k) as usize];
                    let k = k as usize;

                    x += ((p >> 24) & 0xff) * kernel[k] as u32;
                    y += ((p >> 16) & 0xff) * kernel[k] as u32;
                    z += ((p >> 8) & 0xff) * kernel[k] as u32;
                    w += (p & 0xff) * kernel[k] as u32;
                }
                d[j as usize] = (x / a) << 24 | (y / a) << 16 | (z / a) << 8 | (w / a);
            }
        }

        // Then vertically blur from tmp -> surface
        for i in 0..height {
            let mut s: &mut [u32] =
                unsafe { dst[(i * dst_stride) as usize..].align_to_mut::<u32>().1 };
            let d: &mut [u32] = unsafe { src[(i * src_stride) as usize..].align_to_mut::<u32>().1 };
            for j in 0..width {
                if radius < i && i < height - radius {
                    let j = j as usize;
                    d[j] = s[j];
                    continue;
                }

                x = 0;
                y = 0;
                z = 0;
                w = 0;
                for k in 0..size {
                    if i - half + k < 0 || i - half + k >= height {
                        continue;
                    }

                    s = unsafe {
                        dst[((i - half + k) * dst_stride) as usize..]
                            .align_to_mut::<u32>()
                            .1
                    };
                    p = s[j as usize];
                    let k = k as usize;
                    x += ((p >> 24) & 0xff) * kernel[k] as u32;
                    y += ((p >> 16) & 0xff) * kernel[k] as u32;
                    z += ((p >> 8) & 0xff) * kernel[k] as u32;
                    w += (p & 0xff) * kernel[k] as u32;
                }
                d[j as usize] = (x / a) << 24 | (y / a) << 16 | (z / a) << 8 | (w / a);
            }
        }
        RgbImage::new(&dst, width, height, depth)
    }

    /// Returns an SRGB image
    pub fn to_srgb_image(&self) -> Result<RgbImage, FltkError> {
        assert!(self.depth() as i32 >= 3);
        let depth = self.depth() as i32;
        let w = self.w();
        let h = self.h();
        fn correct_gamma(v: f32) -> f32 {
            if v <= 0.0031308 {
                v * 12.92
            } else {
                1.055 * v.powf(1.0 / 2.4) - 0.055
            }
        }
        let mut temp = vec![];
        let data = self.to_rgb_data();
        match depth {
            3 => {
                for (_, pixel) in data.chunks_exact(3).enumerate() {
                    let r = (correct_gamma(pixel[0] as f32 / 255.0) * 255.0) as u8;
                    let g = (correct_gamma(pixel[1] as f32 / 255.0) * 255.0) as u8;
                    let b = (correct_gamma(pixel[2] as f32 / 255.0) * 255.0) as u8;
                    temp.push(r);
                    temp.push(g);
                    temp.push(b);
                }
                assert!(temp.len() as i32 == w * h * 3);
                RgbImage::new(&temp, w, h, ColorDepth::Rgb8)
            }
            4 => {
                for (_, pixel) in data.chunks_exact(4).enumerate() {
                    let r = (correct_gamma(pixel[0] as f32 / 255.0) * 255.0) as u8;
                    let g = (correct_gamma(pixel[1] as f32 / 255.0) * 255.0) as u8;
                    let b = (correct_gamma(pixel[2] as f32 / 255.0) * 255.0) as u8;
                    temp.push(r);
                    temp.push(g);
                    temp.push(b);
                    temp.push(pixel[3]);
                }
                assert!(temp.len() as i32 == w * h * 4);
                RgbImage::new(&temp, w, h, ColorDepth::Rgba8)
            }
            _ => unreachable!(),
        }
    }

    /// Sets the scaling algorithm
    pub fn set_scaling_algorithm(algorithm: RgbScaling) {
        unsafe { Fl_RGB_Image_set_scaling_algorithm(algorithm as i32) }
    }

    /// Gets the scaling algorithm
    pub fn scaling_algorithm() -> RgbScaling {
        unsafe { mem::transmute(Fl_RGB_Image_scaling_algorithm()) }
    }
}

/// Creates a struct holding a Windows icon (.ico) image
#[derive(Debug)]
pub struct IcoImage {
    inner: ImageRC<*mut Fl_ICO_Image>,
}

crate::macros::image::impl_image_ext!(IcoImage, Fl_ICO_Image);

impl IcoImage {
    /// Loads the image from a filesystem path, doesn't check for the validity of the data
    /// # Errors
    /// Errors on non-existent path or invalid format
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<IcoImage, FltkError> {
        Self::load_(path.as_ref())
    }

    fn load_(path: &std::path::Path) -> Result<IcoImage, FltkError> {
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        unsafe {
            let temp = path.to_str().ok_or_else(|| {
                FltkError::Unknown(String::from("Failed to convert path to string"))
            })?;
            let temp = CString::new(temp)?;
            let image_ptr = Fl_ICO_Image_new(temp.as_ptr(), -1);
            if image_ptr.is_null() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                if Fl_ICO_Image_fail(image_ptr) < 0 {
                    return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                }
                Ok(IcoImage {
                    inner: ImageRC::from(image_ptr),
                })
            }
        }
    }

    /// Loads the image from data/memory
    /// # Errors
    /// Errors on invalid format
    pub fn from_data(data: &[u8]) -> Result<IcoImage, FltkError> {
        unsafe {
            if data.is_empty() {
                Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
            } else {
                let x = Fl_ICO_Image_from_data(data.as_ptr(), data.len() as _, -1);
                if x.is_null() {
                    Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
                } else {
                    if Fl_ICO_Image_fail(x) < 0 {
                        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
                    }
                    Ok(IcoImage {
                        inner: ImageRC::from(x),
                    })
                }
            }
        }
    }

    /// Get the icon directory entry
    pub fn icon_dir_entry(&self) -> Vec<IconDirEntry> {
        unsafe {
            let mut size = 0;
            let ret = Fl_ICO_Image_icondirentry(*self.inner, &mut size) as *mut IconDirEntry;
            std::slice::from_raw_parts(ret, size as _).to_owned()
        }
    }
}

use std::os::raw::c_int;

/// Icon directory entry
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IconDirEntry {
    ///  Image width
    b_width: c_int,
    ///  Image height
    b_height: c_int,
    ///  Number of colors (0 if  ≥ 8bpp)
    b_color_count: c_int,
    ///  Reserved
    b_reserve: c_int,
    ///  Color Planes
    w_planes: c_int,
    ///  Bits per pixel
    w_bit_count: c_int,
    ///  Resource size in bytes
    dw_bytes_in_res: c_int,
    ///  Offset to the image
    dw_image_offset: c_int,
}
