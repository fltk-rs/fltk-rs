pub use crate::prelude::*;
use crate::image::RgbImage;
use fltk_sys::draw::*;
use std::os::raw;

/// Draws a line
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        cfl_line(x1, y1, x2, y2);
    }
}

/// Draws a point
pub fn draw_point(x: i32, y: i32) {
    unsafe { cfl_point(x, y) }
}

/// Draws a rectangle
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_rect(x, y, w, h) }
}

/// Draws a rectangle with border color
pub fn draw_rect_with_color(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { cfl_rect_with_color(x, y, w, h, color as u32) }
}

/// Draws a loop
pub fn draw_loop(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe {
        cfl_loop(x1, y1, x2, y2, x3, y3);
    }
}

/// Draws a filled rectangle
pub fn draw_rect_fill(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { cfl_rectf_with_color(x, y, w, h, color as u32) }
}

/// Draws a focus rectangle
pub fn draw_focus_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_focus_rect(x, y, w, h) }
}

/// Sets the drawing color
pub fn set_draw_color(color: Color) {
    unsafe { cfl_set_color_int(color.to_u32()) }
}

/// Draws a circle
pub fn draw_circle(x: f64, y: f64, r: f64) {
    unsafe {
        cfl_circle(x, y, r);
    }
}

/// Draws an arc
pub fn draw_arc(x: i32, y: i32, w: i32, h: i32, a: f64, b: f64) {
    unsafe {
        cfl_arc(x, y, w, h, a, b);
    }
}

/// Draws a filled pie
pub fn draw_pie(x: i32, y: i32, w: i32, h: i32, a: f64, b: f64) {
    unsafe {
        cfl_pie(x, y, w, h, a, b);
    }
}

/// Captures part of the window and returns raw data
pub fn capture_window<Window: WindowTrait>(win: &mut Window) -> RgbImage {
    let cp = win.width() as usize * win.height() as usize * 3;
    win.show();
    unsafe {
        let x = cfl_read_image(std::ptr::null_mut(), 0, 0, win.width(), win.height(), 0);
        assert!(!x.is_null(), "Failed to read image from region!");
        let x = std::slice::from_raw_parts(x, cp);
        RgbImage::new(x.to_vec(), win.width(), win.height())
    }
}

// /// Captures part of the window, returns a raw RGB data
// pub fn capture_window_part<Window: WindowTrait>(
//     win: &Window,
//     x: i32,
//     y: i32,
//     w: i32,
//     h: i32,
// ) -> Vec<u8> {
//     assert!(
//         x + w <= win.width() && y + h <= win.height(),
//         "Captures must be less than the parent window's size!"
//     );
//     unsafe {
//         let x = cfl_capture_window_part(win.as_widget_ptr() as *mut raw::c_void, x, y, w, h);
//         assert!(!x.is_null());
//         let cp = w as usize * h as usize * 3;
//         let x = std::slice::from_raw_parts(x, cp);
//         x.to_vec()
//     }
// }

/// Sets the line style
pub fn set_line_style(style: LineStyle, width: i32) {
    unsafe {
        cfl_line_style(
            style as i32,
            width,
            std::ptr::null_mut() as *mut std::os::raw::c_char,
        );
    }
}

/// Limits drawing to a region
pub fn push_clip(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        cfl_push_clip(x, y, w, h);
    }
}

/// Puts the drawing back
pub fn pop_clip() {
    unsafe {
        cfl_pop_clip();
    }
}

/// Transforms raw data to png file
pub fn write_to_png_file(
    rgb_image: RgbImage,
    path: &std::path::Path,
) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().unwrap();
    let path = std::ffi::CString::new(path).unwrap();
    unsafe {
        match cfl_raw_image_to_png(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to jpg file
pub fn write_to_jpg_file(
    rgb_image: RgbImage,
    path: &std::path::Path,
) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().unwrap();
    let path = std::ffi::CString::new(path).unwrap();
    unsafe {
        match cfl_raw_image_to_jpg(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to bmp file
pub fn write_to_bmp_file(
    rgb_image: RgbImage,
    path: &std::path::Path,
) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().unwrap();
    let path = std::ffi::CString::new(path).unwrap();
    unsafe {
        match cfl_raw_image_to_bmp(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}
