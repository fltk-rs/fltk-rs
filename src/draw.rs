use crate::image::RgbImage;
pub use crate::prelude::*;
use fltk_sys::draw::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// Opaque type around Fl_Region
#[derive(Debug)]
pub struct Region {
    _inner: *mut raw::c_void,
}

unsafe impl Sync for Region {}

unsafe impl Send for Region {}

/// Opaque type around Fl_Offscreen
#[derive(Debug)]
pub struct Offscreen {
    _inner: *mut raw::c_void,
}

unsafe impl Sync for Offscreen {}

unsafe impl Send for Offscreen {}

impl Offscreen {
    /// Creates a new offscreen type
    pub fn new(w: i32, h: i32) -> Option<Offscreen> {
        unsafe {
            let x = cfl_create_offscreen(w, h);
            if x.is_null() {
                None
            } else {
                Some(Offscreen { _inner: x })
            }
        }
    }

    /// Creates an uninitialized offscreen type
    pub unsafe fn uninit() -> Offscreen {
        Offscreen {
            _inner: std::ptr::null_mut(),
        }
    }

    /// Begins drawing in the offscreen
    pub fn begin(&self) {
        assert!(!self._inner.is_null());
        unsafe { cfl_begin_offscreen(self._inner) }
    }

    /// Ends drawing in the offscreen
    pub fn end(&self) {
        assert!(!self._inner.is_null());
        unsafe { cfl_end_offscreen() }
    }

    /// Copies the offscreen
    pub fn copy(&self, x: i32, y: i32, w: i32, h: i32, srcx: i32, srcy: i32) {
        assert!(!self._inner.is_null());
        unsafe { cfl_copy_offscreen(x, y, w, h, self._inner, srcx, srcy) }
    }

    /// Rescales the offscreen
    pub fn rescale(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { cfl_rescale_offscreen(self._inner) }
    }

    /// Checks the validity of the offscreen
    pub fn is_valid(&self) -> bool {
        assert!(!self._inner.is_null());
        if self._inner.is_null() {
            false
        } else {
            true
        }
    }

    /// Performs a shallow copy of the offscreen
    pub unsafe fn memcpy(&self) -> Offscreen {
        assert!(!self._inner.is_null());
        Offscreen {
            _inner: self._inner,
        }
    }
}

impl Drop for Offscreen {
    fn drop(&mut self) {
        unsafe { cfl_delete_offscreen(self._inner) }
    }
}

/// Shows a color map
pub fn show_colormap(old_color: Color) -> Color {
    unsafe { mem::transmute(Fl_show_colormap(old_color as u32)) }
}

/// Sets the color using rgb values
pub fn set_color_rgb(r: u8, g: u8, b: u8) {
    unsafe { cfl_set_color_rgb(r, g, b) }
}

/// Gets the last used color
pub fn get_color() -> Color {
    unsafe { mem::transmute(cfl_get_color()) }
}

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

/// Sets the clip region
pub fn set_clip_region(r: &Region) {
    unsafe { cfl_set_clip_region(r._inner) }
}

/// Gets the clip region
pub fn clip_region() -> Option<Region> {
    unsafe {
        let x = cfl_clip_region();
        if x.is_null() {
            None
        } else {
            Some(Region { _inner: x })
        }
    }
}

/// Pushes an empty clip region onto the stack so nothing will be clipped
pub fn push_no_clip() {
    unsafe { cfl_push_no_clip() }
}

/// Returns whether the rectangle intersect with the current clip region
pub fn not_clipped(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe {
        match cfl_not_clipped(x, y, w, h) {
            0 => false,
            _ => true,
        }
    }
}

/// Restores the clip region
pub fn restore_clip() {
    unsafe { cfl_restore_clip() }
}

/// Copies the offscreen
#[allow(dead_code)]
fn copy_offscreen(x: i32, y: i32, w: i32, h: i32, pixmap: &Offscreen, srcx: i32, srcy: i32) {
    unsafe { cfl_copy_offscreen(x, y, w, h, pixmap._inner, srcx, srcy) }
}

/// Creates an offscreen
pub fn create_offscreen(w: i32, h: i32) -> Offscreen {
    unsafe {
        let x = cfl_create_offscreen(w, h);
        assert!(!x.is_null());
        Offscreen { _inner: x }
    }
}

/// Begins the offscreen
#[allow(dead_code)]
fn begin_offscreen(b: &Offscreen) {
    unsafe { cfl_begin_offscreen(b._inner) }
}

/// Ends the offscreen
pub fn end_offscreen() {
    unsafe { cfl_end_offscreen() }
}

/// Deletes the offscreen
#[allow(dead_code)]
fn delete_offscreen(bitmap: &mut Offscreen) {
    unsafe { cfl_delete_offscreen(bitmap._inner) }
}

/// Rescales the offscreen
#[allow(dead_code)]
fn rescale_offscreen(ctx: &mut Offscreen) {
    unsafe { cfl_rescale_offscreen(ctx._inner) }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_x(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_x(x, y) }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_y(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_y(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dx(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_dx(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dy(x: f64, y: f64) -> f64 {
    unsafe { cfl_transform_dy(x, y) }
}

/// Adds coordinate pair to the vertex list without further transformations
pub fn transformed_vertex(xf: f64, yf: f64) {
    unsafe { cfl_transformed_vertex(xf, yf) }
}

/// Draws a filled rectangle
pub fn draw_rectf(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_rectf(x, y, w, h) }
}

/// Draws a filled rectangle with specified RGB color
pub fn draw_rectf_with_rgb(x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
    unsafe { cfl_rectf_with_rgb(x, y, w, h, r, g, b) }
}

/// Draws a line from (x,y) to (x1,y1) and another from (x1,y1) to (x2,y2)
pub fn draw_line2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { cfl_line2(x, y, x1, y1, x2, y2) }
}

/// Outlines a 4-sided polygon with lines
pub fn draw_loop2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe { cfl_loop2(x, y, x1, y1, x2, y2, x3, y3) }
}

/// Fills a 3-sided polygon. The polygon must be convex
pub fn draw_polygon(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { cfl_polygon(x, y, x1, y1, x2, y2) }
}

/// Fills a 4-sided polygon. The polygon must be convex
pub fn draw_polygon2(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe { cfl_polygon2(x, y, x1, y1, x2, y2, x3, y3) }
}

/// Adds a series of points on a Bezier curve to the path
pub fn draw_curve(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe { cfl_curve(x0, y0, x1, y1, x2, y2, x3, y3) }
}

/// Draws an arc
pub fn draw_arc2(x: f64, y: f64, r: f64, start: f64, end: f64) {
    unsafe { cfl_arc2(x, y, r, start, end) }
}

/// Draws a horizontal line from (x,y) to (x1,y)
pub fn draw_xyline(x: i32, y: i32, x1: i32) {
    unsafe { cfl_xyline(x, y, x1) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then vertical from (x1,y) to (x1,y2)
pub fn draw_xyline2(x: i32, y: i32, x1: i32, y2: i32) {
    unsafe { cfl_xyline2(x, y, x1, y2) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then a vertical from (x1,y) to (x1,y2)
/// and then another horizontal from (x1,y2) to (x3,y2)
pub fn draw_xyline3(x: i32, y: i32, x1: i32, y2: i32, x3: i32) {
    unsafe { cfl_xyline3(x, y, x1, y2, x3) }
}

/// Draws a vertical line from (x,y) to (x,y1)
pub fn draw_yxline(x: i32, y: i32, y1: i32) {
    unsafe { cfl_yxline(x, y, y1) }
}

/// Draws a vertical line from (x,y) to (x,y1), then a horizontal from (x,y1) to (x2,y1)
pub fn draw_yxline2(x: i32, y: i32, y1: i32, x2: i32) {
    unsafe { cfl_yxline2(x, y, y1, x2) }
}

///  Draws a vertical line from (x,y) to (x,y1) then a horizontal from (x,y1)
/// to (x2,y1), then another vertical from (x2,y1) to (x2,y3)
pub fn draw_yxline3(x: i32, y: i32, y1: i32, x2: i32, y3: i32) {
    unsafe { cfl_yxline3(x, y, y1, x2, y3) }
}

/// Saves the current transformation matrix on the stack
pub fn push_matrix() {
    unsafe { cfl_push_matrix() }
}

/// Pops the current transformation matrix from the stack
pub fn pop_matrix() {
    unsafe { cfl_pop_matrix() }
}

/// Concatenates scaling transformation onto the current one
pub fn scale_xy(x: f64, y: f64) {
    unsafe { cfl_scale(x, y) }
}

/// Concatenates scaling transformation onto the current one
pub fn scale_x(x: f64) {
    unsafe { cfl_scale2(x) }
}

/// Concatenates translation transformation onto the current one
pub fn translate(x: f64, y: f64) {
    unsafe { cfl_translate(x, y) }
}

/// Concatenates rotation transformation onto the current one
pub fn rotate(d: f64) {
    unsafe { cfl_rotate(d) }
}

/// Concatenates another transformation onto the current one
pub fn mult_matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) {
    unsafe { cfl_mult_matrix(a, b, c, d, x, y) }
}

/// Starts drawing a list of points. Points are added to the list with fl_vertex()
pub fn begin_points() {
    unsafe { cfl_begin_points() }
}

/// Starts drawing a list of lines
pub fn begin_line() {
    unsafe { cfl_begin_line() }
}

/// Starts drawing a closed sequence of lines
pub fn begin_loop() {
    unsafe { cfl_begin_loop() }
}

/// Starts drawing a convex filled polygon
pub fn begin_polygon() {
    unsafe { cfl_begin_polygon() }
}

/// Adds a single vertex to the current path
pub fn vertex(x: f64, y: f64) {
    unsafe { cfl_vertex(x, y) }
}

/// Ends list of points, and draws
pub fn end_points() {
    unsafe { cfl_end_points() }
}

/// Ends list of lines, and draws
pub fn end_line() {
    unsafe { cfl_end_line() }
}

/// Ends closed sequence of lines, and draws
pub fn end_loop() {
    unsafe { cfl_end_loop() }
}

/// Ends closed sequence of lines, and draws
pub fn end_polygon() {
    unsafe { cfl_end_polygon() }
}

/// Starts drawing a complex filled polygon
pub fn begin_complex_polygon() {
    unsafe { cfl_begin_complex_polygon() }
}

/// Call gap() to separate loops of the path
pub fn gap() {
    unsafe { cfl_gap() }
}

/// Ends complex filled polygon, and draws
pub fn end_complex_polygon() {
    unsafe { cfl_end_complex_polygon() }
}

/// Sets the current font, which is then used in various drawing routines
pub fn set_font(face: Font, fsize: u32) {
    unsafe { cfl_set_font(face as i32, fsize as i32) }
}

/// Gets the current font, which is used in various drawing routines
pub fn font() -> Font {
    unsafe { mem::transmute(cfl_font()) }
}

/// Gets the current font size, which is used in various drawing routines
pub fn size() -> u32 {
    unsafe { cfl_size() as u32 }
}

/// Returns the recommended minimum line spacing for the current font
pub fn height() -> i32 {
    unsafe { cfl_height() }
}

/// Sets the line spacing for the current font
pub fn set_height(font: Font, size: u32) {
    unsafe {
        cfl_set_height(font as i32, size as i32);
    }
}

/// Returns the recommended distance above the bottom of a height() tall box to
/// draw the text at so it looks centered vertically in that box
pub fn descent() -> i32 {
    unsafe { cfl_descent() }
}

/// Returns the typographical width of a string
pub fn width(txt: &str) -> f64 {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_width(txt.as_ptr()) }
}

/// Returns the typographical width of a sequence of n characters
pub fn width2(txt: &str, n: i32) -> f64 {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_width2(txt.as_ptr(), n) }
}

/// Returns the typographical width of a single character
pub fn char_width(c: char) -> f64 {
    unsafe { cfl_width3(c as u32) }
}

/// Converts text from Windows/X11 latin1 character set to local encoding
pub fn latin1_to_local(txt: &str, n: i32) -> String {
    let txt = CString::new(txt).unwrap();
    unsafe {
        let x = cfl_latin1_to_local(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Converts text from local encoding to Windowx/X11 latin1 character set
pub fn local_to_latin1(txt: &str, n: i32) -> String {
    let txt = CString::new(txt).unwrap();
    unsafe {
        let x = cfl_local_to_latin1(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a string starting at the given x, y location
pub fn draw_text(txt: &str, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw(txt.as_ptr(), x, y) }
}

/// Draws a string starting at the given x, y location, rotated to an angle
pub fn draw_text_angled(angle: i32, txt: &str, x: i32, y: i32) {
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_draw2(angle, txt.as_ptr(), x, y) }
}

/// Draws a UTF-8 string right to left starting at the given x, y location
pub fn rtl_draw(txt: &str, x: i32, y: i32) {
    let n = txt.len() as i32;
    let txt = CString::new(txt).unwrap();
    unsafe { cfl_rtl_draw(txt.as_ptr(), n, x, y) }
}

/// Draws a frame with text
pub fn draw_frame(s: &str, x: i32, y: i32, w: i32, h: i32) {
    let s = CString::new(s).unwrap();
    unsafe { cfl_frame(s.as_ptr(), x, y, w, h) }
}

/// Draws a frame with text.
/// Differs from frame() by the order of the line segments
pub fn draw_frame2(s: &str, x: i32, y: i32, w: i32, h: i32) {
    let s = CString::new(s).unwrap();
    unsafe { cfl_frame2(s.as_ptr(), x, y, w, h) }
}

/// Draws a box given the box type, size, position and color
pub fn draw_box(box_type: FrameType, x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { cfl_draw_box(box_type as i32, x, y, w, h, color as u32) }
}

/// Checks whether platform supports true alpha blending for RGBA images
pub fn can_do_alpha_blending() -> bool {
    unsafe {
        match cfl_can_do_alpha_blending() {
            0 => false,
            _ => true,
        }
    }
}

/// Get a human-readable string from a shortcut value
pub fn shortcut_label(shortcut: Shortcut) -> String {
    unsafe {
        let x = cfl_shortcut_label(shortcut as u32);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a selection rectangle, erasing a previous one by XOR'ing it first.
pub fn overlay_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_overlay_rect(x, y, w, h) }
}

/// Erase a selection rectangle without drawing a new one
pub fn overlay_clear() {
    unsafe { cfl_overlay_clear() }
}

/// Sets the cursor style
pub fn set_cursor(cursor: Cursor) {
    unsafe { cfl_set_cursor(cursor as i32) }
}

/// Sets the cursor style
pub fn set_cursor_with_color(cursor: Cursor, fg: Color, bg: Color) {
    unsafe { cfl_set_cursor2(cursor as i32, fg as i32, bg as i32) }
}

/// Sets the status
pub fn set_status(x: i32, y: i32, w: i32, h: i32) {
    unsafe { cfl_set_status(x, y, w, h) }
}

/// Sets spot within the window
pub fn set_spot<Win: WindowExt>(font: Font, size: u32, x: i32, y: i32, w: i32, h: i32, win: &Win) {
    unsafe {
        assert!(!win.was_deleted());
        cfl_set_spot(
            font as i32,
            size as i32,
            x,
            y,
            w,
            h,
            win.as_widget_ptr() as *mut raw::c_void,
        )
    }
}

/// Resets the spot within the window
pub fn reset_spot() {
    unsafe { cfl_reset_spot() }
}

/// Captures part of the window and returns raw data
pub fn capture_window<Win: WindowExt>(win: &mut Win) -> Result<RgbImage, FltkError> {
    assert!(!win.was_deleted());
    let cp = win.width() as u32 * win.height() as u32 * 3;
    win.show();
    unsafe {
        let x = cfl_read_image(std::ptr::null_mut(), 0, 0, win.width(), win.height(), 0);
        if x.is_null() {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize).to_vec();
            Ok(RgbImage::new(&x, win.width() as u32, win.height() as u32, 3)?)
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
//     assert!(!widget.was_deleted());
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

/// Transforms raw data to png file
pub fn write_to_png_file<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(), "SVG images are not supported!");
    // assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage images are not supported!");
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_png(*image.to_raw_data() as *mut u8, path.as_ptr(), image.data_w() as i32, image.data_h() as i32) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to jpg file
pub fn write_to_jpg_file<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(), "SVG images are not supported!");
    // assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage images are not supported!");
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_jpg(*image.to_raw_data() as *mut u8, path.as_ptr(), image.data_w() as i32, image.data_h() as i32) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to bmp file
pub fn write_to_bmp_file<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(), "SVG images are not supported!");
    // assert!(std::any::type_name::<I>() != std::any::type_name::<crate::image::SharedImage>(), "SharedImage images are not supported!");
    let path = path.to_str().ok_or(FltkError::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not convert path to string!",
    )))?;
    let path = std::ffi::CString::new(path)?;
    unsafe {
        match cfl_raw_image_to_bmp(*image.to_raw_data() as *mut u8, path.as_ptr(), image.data_w() as i32, image.data_h() as i32) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}
