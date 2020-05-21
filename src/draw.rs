use crate::image::RgbImage;
pub use crate::prelude::*;
use fltk_sys::draw::*;
use std::os::raw;
use std::mem;
use std::ffi::{CStr, CString};

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
pub fn capture_window<Win: WindowExt>(win: &mut Win) -> Option<RgbImage> {
    let cp = win.width() as u32 * win.height() as u32 * 3;
    win.show();
    unsafe {
        let x = cfl_read_image(std::ptr::null_mut(), 0, 0, win.width(), win.height(), 0);
        if x.is_null() {
            None
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize);
            Some(RgbImage::new(&x.to_vec(), win.width(), win.height(), 3))
        }
    }
}

// /// Captures part of the window, returns a raw RGB data
// pub fn capture_window_part<Win: WindowExt>(
//     win: &Win,
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
//         let cp = w as u32 * h as u32 * 3;
//         let x = std::slice::from_raw_parts(x, cp as usize);
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
pub fn write_to_png_file(rgb_image: RgbImage, path: &std::path::Path) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_png(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to jpg file
pub fn write_to_jpg_file(rgb_image: RgbImage, path: &std::path::Path) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_jpg(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to bmp file
pub fn write_to_bmp_file(rgb_image: RgbImage, path: &std::path::Path) -> Result<(), FltkError> {
    let (data, w, h) = rgb_image.into_parts();
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_bmp(
            data.as_ptr() as *mut u8,
            path.as_ptr() as *const raw::c_char,
            w,
            h,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Shows a color map
pub fn show_colormap(old_color: Color) -> Color {
    unsafe { mem::transmute(Fl_show_colormap(old_color as u32)) }
}

pub fn set_color_rgb(r: u8, g: u8, b: u8) {
    unsafe { cfl_set_color_rgb(r, g, b) }
}

pub fn get_color() -> Color {
    unsafe { mem::transmute(cfl_get_color()) }
}

pub fn push_no_clip() {
    unsafe { cfl_push_no_clip() }
}

pub fn not_clipped(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe {
        match cfl_not_clipped(x, y, w, h) {
            0 => false,
            _ => true,
        }
    }
}

pub fn restore_clip() {
    unsafe { cfl_restore_clip() }
}

pub fn rectf(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_rectf(x, y, w, h) }
}

pub fn rectf_with_rgb(x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
    unsafe { cfl_rectf_with_rgb(x, y, w, h, r, g, b) }
}

pub fn line2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { cfl_line2(x, y, x1, y1, x2, y2) }
}

pub fn loop2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe { cfl_loop2(x, y, x1, y1, x2, y2, x3, y3) }
}

pub fn polygon(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { cfl_polygon(x, y, x1, y1, x2, y2) }
}

pub fn polygon2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe { cfl_polygon2(x, y, x1, y1, x2, y2, x3, y3) }
}

pub fn xyline(x: i32, y: i32, x1: i32) {
    unsafe { cfl_xyline(x, y, x1) }
}

pub fn xyline2(x: i32, y: i32, x2: i32, y2: i32) {
    unsafe { cfl_xyline2(x, y, x2, y2) }
}

pub fn xyline3(x: i32, y: i32, x2: i32, y2: i32, x3: i32) {
    unsafe { cfl_xyline3(x, y, x2, y2, x3) }
}

pub fn yxline(x: i32, y: i32, y1: i32) {
    unsafe { cfl_yxline(x, y, y1) }
}

pub fn yxline2(x: i32, y: i32, y1: i32, x2: i32) {
    unsafe { cfl_yxline2(x, y, y1, x2) }
}

pub fn yxline3(x: i32, y: i32, y1: i32, x2: i32, y3: i32) {
    unsafe { cfl_yxline3(x, y, y1, x2, y3) }
}

pub fn push_matrix() {
    unsafe { cfl_push_matrix() }
}

pub fn pop_matrix() {
    unsafe { cfl_pop_matrix() }
}

pub fn scale(x: f64, y: f64) {
    unsafe { cfl_scale(x, y) }
}

pub fn scale2(x: f64) {
    unsafe { cfl_scale2(x) }
}

pub fn translate(x: f64, y: f64) {
    unsafe { cfl_translate(x, y) }
}

pub fn rotate(d: f64) {
    unsafe { cfl_rotate(d) }
}

pub fn mult_matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) {
    unsafe { cfl_mult_matrix(a, b, c, d, x, y) }
}

pub fn begin_points() {
    unsafe { cfl_begin_points() }
}

pub fn begin_line() {
    unsafe { cfl_begin_line() }
}

pub fn begin_loop() {
    unsafe { cfl_begin_loop() }
}

pub fn begin_polygon() {
    unsafe { cfl_begin_polygon() }
}

pub fn vertex(x: f64, y: f64) {
    unsafe { cfl_vertex(x, y) }
}

pub fn curve(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe { cfl_curve(x0, y0, x1, y1, x2, y2, x3, y3) }
}

pub fn arc2(x: f64, y: f64, r: f64, start: f64, end: f64) {
    unsafe { cfl_arc2(x, y, r, start, end) }
}

pub fn end_points() {
    unsafe { cfl_end_points() }
}

pub fn end_line() {
    unsafe { cfl_end_line() }
}

pub fn end_loop() {
    unsafe { cfl_end_loop() }
}

pub fn end_polygon() {
    unsafe { cfl_end_polygon() }
}

pub fn begin_complex_polygon() {
    unsafe { cfl_begin_complex_polygon() }
}

pub fn gap() {
    unsafe { cfl_gap() }
}

pub fn end_complex_polygon() {
    unsafe { cfl_end_complex_polygon() }
}

pub fn transform_x(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_x(x, y) }
}

pub fn transform_y(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_y(x, y) }
}

pub fn transform_dx(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_dx(x, y) }
}

pub fn transform_dy(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_dy(x, y) }
}

pub fn transformed_vertex(xf: f64, yf: f64) {
    unsafe { cfl_transformed_vertex(xf, yf) }
}

pub fn end_offscreen() {
    unsafe { cfl_end_offscreen() }
}

pub fn set_font(face: Font, fsize: u32) {
    unsafe { cfl_set_font(face as i32, fsize as i32) }
}

pub fn font() -> Font {
    unsafe { mem::transmute(cfl_font()) }
}

pub fn size() -> u32 {
    unsafe { cfl_size() as u32 }
}

pub fn height() -> i32 {
    unsafe { cfl_height() }
}

pub fn set_height(font: Font, size: u32) {
    unsafe { cfl_set_height(font as i32, size as i32); }
}

pub fn descent() -> i32 {
    unsafe { cfl_descent() }
}

pub fn width(txt: &str) -> f64 {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_width(txt.into_raw()) }
}

pub fn width2(txt: &str, n: i32) -> f64 {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_width2(txt.into_raw(), n) }
}

pub fn width3(c: Color) -> f64 {
    unsafe { cfl_width3(c as u32) }
}

pub fn latin1_to_local(txt: &str, n: i32) -> String {
    let txt = CString::new(txt).unwrap();
    unsafe {
        let x = cfl_latin1_to_local(txt.into_raw(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

pub fn local_to_latin1(txt: &str, n: i32) -> String {
    let txt = CString::new(txt).unwrap();
    unsafe {
        let x = cfl_local_to_latin1(txt.into_raw(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

pub fn draw(txt: &str, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw(txt.into_raw(), x, y) }
}

pub fn draw2(angle: i32, txt: &str, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw2(angle, txt.into_raw(), x, y) }
}

pub fn draw3(txt: &str, n: i32, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw3(txt.into_raw(), n, x, y) }
}

pub fn draw4(angle: i32, txt: &str, n: i32, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw4(angle, txt.into_raw(), n, x, y) }
}

pub fn rtl_draw(txt: &str, n: i32, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_rtl_draw(txt.into_raw(), n, x, y) }
}

pub fn frame7(s: &str, x: i32, y: i32, w: i32, h: i32) {
    let s = CString::new(s).unwrap();
    unsafe { cfl_frame7(s.into_raw(), x, y, w, h) }
}

pub fn frame2(s: &str, x: i32, y: i32, w: i32, h: i32) {
    let s = CString::new(s).unwrap();
    unsafe { cfl_frame2(s.into_raw(), x, y, w, h) }
}

pub fn draw_box(box_type: FrameType, x: i32, y: i32, w: i32, h: i32, arg1: i32) {
    unsafe { cfl_draw_box(box_type as i32, x, y, w, h, arg1) }
}

pub fn can_do_alpha_blending() -> bool {
    unsafe {
        match cfl_can_do_alpha_blending() {
            0 => false,
            _ => true,
        }
    }
}

pub fn shortcut_label(shortcut: Color) -> String {
    unsafe {
        let x = cfl_shortcut_label(shortcut as u32);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

pub fn old_shortcut(s: &str) -> Color {
    let s = CString::new(s).unwrap();
    unsafe { mem::transmute(cfl_old_shortcut(s.into_raw())) }
}

pub fn overlay_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_overlay_rect(x, y, w, h) }
}

pub fn overlay_clear() {
    unsafe { cfl_overlay_clear() }
}

pub fn set_cursor(cursor: CursorStyle) {
    unsafe { cfl_set_cursor(cursor as i32) }
}

pub fn set_cursor2(cursor: CursorStyle, fg: i32, bg: i32) {
    unsafe { cfl_set_cursor2(cursor as i32, fg, bg) }
}

pub fn set_status(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_set_status(x, y, w, h) }
}

pub fn set_spot<Win: WindowExt>(font: Font, size: u32, x: i32, y: i32, w: i32, h: i32, win: Win) {
    unsafe { cfl_set_spot(font as i32, size as i32, x, y, w, h, win.as_widget_ptr() as *mut raw::c_void) }
}

pub fn reset_spot() {
    unsafe { cfl_reset_spot() }
}
