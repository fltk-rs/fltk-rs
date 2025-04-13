use crate::enums::{Align, Color, ColorDepth, Cursor, Font, FrameType, Shortcut};
use crate::image::RgbImage;
use crate::prelude::*;
use crate::surface::ImageSurface;
use crate::utils::FlString;
use fltk_sys::draw::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

bitflags::bitflags! {
    /// Defines the line styles supported by fltk
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LineStyle: i32 {
        /// Solid line
        const Solid = 0;
        /// Dash
        const Dash = 1;
        /// Dot
        const Dot = 2;
        /// Dash dot
        const DashDot = 3;
        /// Dash dot dot
        const DashDotDot = 4;
        /// Cap flat
        const CapFlat = 0x100;
        /// Cap round
        const CapRound = 0x200;
        /// Cap square
        const CapSquare = 0x300;
        /// Join miter
        const JoinMiter = 0x1000;
        /// Join round
        const JoinRound = 0x2000;
        /// Join bevel
        const JoinBevel = 0x3000;
    }
}

/// Opaque type around `Fl_Region`
pub struct Region(pub(crate) *mut raw::c_void);

/// Shows a color map
pub fn show_colormap(old_color: Color) -> Color {
    unsafe { mem::transmute(Fl_show_colormap(old_color.bits())) }
}

/// Sets the color using rgb values
pub fn set_color_rgb(r: u8, g: u8, b: u8) {
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Gets the last used color
pub fn get_color() -> Color {
    unsafe { mem::transmute(Fl_get_color()) }
}

/// Draws a line
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        Fl_line(x1, y1, x2, y2);
    }
}

/// Draws a line from (x,y) to (x1,y1) and another from (x1,y1) to (x2,y2)
pub fn draw_polyline(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe { Fl_polyline(x1, y1, x2, y2, x3, y3) }
}

/// Draws a point
pub fn draw_point(x: i32, y: i32) {
    unsafe { Fl_point(x, y) }
}

/// Draws a rectangle
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_rect(x, y, w, h) }
}

/// Draws a rectangle with border color
pub fn draw_rect_with_color(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_rect_with_color(x, y, w, h, color.bits()) }
}

/// Draws a non-filled 3-sided polygon
pub fn draw_loop(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe {
        Fl_loop(x1, y1, x2, y2, x3, y3);
    }
}

/// Draws a non-filled 4-sided polygon
pub fn draw_loop_4sided(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) {
    unsafe {
        Fl_loop_4sided(
            x1, y1, x2, y2, x3, y3, x4, y4,
        );
    }
}

/// Draws a filled rectangle
pub fn draw_rect_fill(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_rectf_with_color(x, y, w, h, color.bits()) }
}

/// Draws a focus rectangle
pub fn draw_focus_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_focus_rect(x, y, w, h) }
}

/// Sets the drawing color
pub fn set_draw_hex_color(color: u32) {
    unsafe {
        crate::app::open_display();
    }
    let (r, g, b) = crate::utils::hex2rgb(color);
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Sets the drawing color
pub fn set_draw_rgb_color(r: u8, g: u8, b: u8) {
    unsafe {
        crate::app::open_display();
    }
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Sets the drawing color
pub fn set_draw_color(color: Color) {
    unsafe {
        crate::app::open_display();
    }
    unsafe { Fl_set_color_int(color.bits()) }
}

/// Draws a circle
pub fn draw_circle(x: f64, y: f64, r: f64) {
    unsafe {
        Fl_circle(x, y, r);
    }
}

/// Draws an arc
pub fn draw_arc(x: i32, y: i32, width: i32, height: i32, a: f64, b: f64) {
    unsafe {
        Fl_arc(x, y, width, height, a, b);
    }
}

/// Draws an arc
pub fn draw_arc_with_radius(x: f64, y: f64, r: f64, start: f64, end: f64) {
    unsafe { Fl_arc_with_radius(x, y, r, start, end) }
}

/// Draws a filled pie
pub fn draw_pie(x: i32, y: i32, width: i32, height: i32, a: f64, b: f64) {
    unsafe {
        Fl_pie(x, y, width, height, a, b);
    }
}

/// Sets the line style
///
/// # Warning
/// You are required to change this back to
/// [`set_line_style(LineStyle::Solid, 0)`](crate::draw::set_line_style)
/// after finishing
pub fn set_line_style(style: LineStyle, width: i32) {
    unsafe {
        crate::app::open_display();
        Fl_line_style(
            style.bits(),
            width,
            std::ptr::null_mut() as *mut std::os::raw::c_char,
        );
    }
}

/// Limits drawing to a region
pub fn push_clip(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        Fl_push_clip(x, y, w, h);
    }
}

/// Puts the drawing back
pub fn pop_clip() {
    unsafe {
        Fl_pop_clip();
    }
}

/// Sets the clip region
pub fn set_clip_region(r: &Region) {
    assert!(!r.0.is_null());
    unsafe { Fl_set_clip_region(r.0) }
}

/// Gets the clip region
pub fn clip_region() -> Region {
    unsafe {
        let ptr = Fl_clip_region();
        assert!(!ptr.is_null());
        Region(ptr)
    }
}

/// Pushes an empty clip region onto the stack so nothing will be clipped
pub fn push_no_clip() {
    unsafe { Fl_push_no_clip() }
}

/// Returns whether the rectangle intersect with the current clip region
pub fn not_clipped(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe { Fl_not_clipped(x, y, w, h) != 0 }
}

/// Restores the clip region
pub fn restore_clip() {
    unsafe { Fl_restore_clip() }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_x(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_x(x, y) }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_y(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_y(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dx(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_dx(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dy(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_dy(x, y) }
}

/// Adds coordinate pair to the vertex list without further transformations
pub fn transformed_vertex(xf: f64, yf: f64) {
    unsafe { Fl_transformed_vertex(xf, yf) }
}

/// Draws a filled rectangle
pub fn draw_rectf(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_rectf(x, y, w, h) }
}

/// Draws a filled rectangle with specified RGB color
pub fn draw_rectf_with_rgb(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color_r: u8,
    color_g: u8,
    color_b: u8,
) {
    unsafe { Fl_rectf_with_rgb(x, y, width, height, color_r, color_g, color_b) }
}

/// Fills a 3-sided polygon. The polygon must be convex
pub fn draw_polygon(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { Fl_polygon(x, y, x1, y1, x2, y2) }
}

/// Fills a 4-sided polygon. The polygon must be convex
pub fn draw_polygon_4sided(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) {
    unsafe {
        Fl_polygon_4sided(
            x1, y1, x2, y2, x3, y3, x4, y4,
        );
    }
}

/// Adds a series of points on a Bezier curve to the path
pub fn draw_curve(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) {
    unsafe {
        Fl_curve(
            x1, y1, x2, y2, x3, y3, x4, y4,
        );
    }
}

/// Draws a horizontal line from (x,y) to (x1,y)
pub fn draw_hline(x: i32, y: i32, x1: i32) {
    unsafe { Fl_xyline(x, y, x1) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then vertical from (x1,y) to (x1,y2)
pub fn draw_hvline(x: i32, y: i32, x1: i32, y2: i32) {
    unsafe { Fl_xyline2(x, y, x1, y2) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then a vertical from (x1,y) to (x1,y2)
/// and then another horizontal from (x1,y2) to (x3,y2)
pub fn draw_hvhline(x: i32, y: i32, x1: i32, y2: i32, x3: i32) {
    unsafe { Fl_xyline3(x, y, x1, y2, x3) }
}

/// Draws a vertical line from (x,y) to (x,y1)
pub fn draw_vline(x: i32, y: i32, y1: i32) {
    unsafe { Fl_yxline(x, y, y1) }
}

/// Draws a vertical line from (x,y) to (x,y1), then a horizontal from (x,y1) to (x2,y1)
pub fn draw_vhline(x: i32, y: i32, y1: i32, x2: i32) {
    unsafe { Fl_yxline2(x, y, y1, x2) }
}

/// Draws a vertical line from (x,y) to (x,y1) then a horizontal from (x,y1)
/// to (x2,y1), then another vertical from (x2,y1) to (x2,y3)
pub fn draw_vhvline(x: i32, y: i32, y1: i32, x2: i32, y3: i32) {
    unsafe { Fl_yxline3(x, y, y1, x2, y3) }
}

/// Saves the current transformation matrix on the stack
pub fn push_matrix() {
    unsafe { Fl_push_matrix() }
}

/// Pops the current transformation matrix from the stack
pub fn pop_matrix() {
    unsafe { Fl_pop_matrix() }
}

/// Concatenates scaling transformation onto the current one
pub fn scale_xy(x: f64, y: f64) {
    unsafe { Fl_scale_xy(x, y) }
}

/// Concatenates scaling transformation onto the current one for both x & y
pub fn scale(val: f64) {
    unsafe { Fl_scale(val) }
}

/// Concatenates translation transformation onto the current one
pub fn translate(x: f64, y: f64) {
    unsafe { Fl_translate(x, y) }
}

/// Concatenates rotation transformation onto the current one
pub fn rotate(d: f64) {
    unsafe { Fl_rotate(d) }
}

/// Concatenates another transformation onto the current one
pub fn mult_matrix(val_a: f64, val_b: f64, val_c: f64, val_d: f64, x: f64, y: f64) {
    unsafe { Fl_mult_matrix(val_a, val_b, val_c, val_d, x, y) }
}

/// Starts drawing a list of points. Points are added to the list with `fl_vertex()`
pub fn begin_points() {
    unsafe { Fl_begin_points() }
}

/// Starts drawing a list of lines
pub fn begin_line() {
    unsafe { Fl_begin_line() }
}

/// Starts drawing a closed sequence of lines
pub fn begin_loop() {
    unsafe { Fl_begin_loop() }
}

/// Starts drawing a convex filled polygon
pub fn begin_polygon() {
    unsafe { Fl_begin_polygon() }
}

/// Adds a single vertex to the current path
pub fn vertex(x: f64, y: f64) {
    unsafe { Fl_vertex(x, y) }
}

/// Ends list of points, and draws
pub fn end_points() {
    unsafe { Fl_end_points() }
}

/// Ends list of lines, and draws
pub fn end_line() {
    unsafe { Fl_end_line() }
}

/// Ends closed sequence of lines, and draws
pub fn end_loop() {
    unsafe { Fl_end_loop() }
}

/// Ends closed sequence of lines, and draws
pub fn end_polygon() {
    unsafe { Fl_end_polygon() }
}

/// Starts drawing a complex filled polygon
pub fn begin_complex_polygon() {
    unsafe { Fl_begin_complex_polygon() }
}

/// Call `gap()` to separate loops of the path
pub fn gap() {
    unsafe { Fl_gap() }
}

/// Ends complex filled polygon, and draws
pub fn end_complex_polygon() {
    unsafe { Fl_end_complex_polygon() }
}

/// Sets the current font, which is then used in various drawing routines
pub fn set_font(face: Font, fsize: i32) {
    unsafe { Fl_set_draw_font(face.bits(), fsize) }
}

/// Gets the current font, which is used in various drawing routines
pub fn font() -> Font {
    unsafe { mem::transmute(Fl_font()) }
}

/// Gets the current font size, which is used in various drawing routines
pub fn size() -> i32 {
    unsafe { Fl_size() }
}

/// Returns the recommended minimum line spacing for the current font
pub fn height() -> i32 {
    unsafe { Fl_height() }
}

/// Sets the line spacing for the current font
pub fn set_height(font: Font, size: i32) {
    unsafe {
        Fl_set_height(font.bits(), size);
    }
}

/// Returns the recommended distance above the bottom of a `height()` tall box to
/// draw the text at so it looks centered vertically in that box
pub fn descent() -> i32 {
    unsafe { Fl_descent() }
}

/// Returns the typographical width of a string
pub fn width(txt: &str) -> f64 {
    let len = txt.len();
    let txt = CString::safe_new(txt);
    unsafe { Fl_width(txt.as_ptr(), len as _) }
}

/// Measure the width and height of a text
pub fn measure(txt: &str, draw_symbols: bool) -> (i32, i32) {
    let txt = CString::safe_new(txt);
    let (mut x, mut y) = (0, 0);
    unsafe {
        Fl_measure(txt.as_ptr(), &mut x, &mut y, i32::from(draw_symbols));
    }
    (x, y)
}

/// Measure the width and height of a text
///
/// If `width` is non-zero, it will wrap to that width
pub fn wrap_measure(txt: &str, width: i32, draw_symbols: bool) -> (i32, i32) {
    let txt = CString::safe_new(txt);
    let (mut x, mut y) = (width, 0);
    unsafe {
        Fl_measure(txt.as_ptr(), &mut x, &mut y, i32::from(draw_symbols));
    }
    (x, y)
}

/// Measure the coordinates and size of the text where a bounding box using the
/// returned data would fit the text
pub fn text_extents(txt: &str) -> (i32, i32, i32, i32) {
    let txt = CString::safe_new(txt);
    let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
    unsafe {
        Fl_text_extents(txt.as_ptr(), &mut x, &mut y, &mut w, &mut h);
    }
    (x, y, w, h)
}

/// Returns the typographical width of a single character
pub fn char_width(c: char) -> f64 {
    unsafe { Fl_char_width(c as u32) }
}

/// Converts text from Windows/X11 latin1 character set to local encoding
pub fn latin1_to_local(txt: &str, n: i32) -> String {
    let txt = CString::safe_new(txt);
    unsafe {
        let x = Fl_latin1_to_local(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Converts text from local encoding to Windowx/X11 latin1 character set
pub fn local_to_latin1(txt: &str, n: i32) -> String {
    let txt = CString::safe_new(txt);
    unsafe {
        let x = Fl_local_to_latin1(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a string starting at the given x, y location
pub fn draw_text(txt: &str, x: i32, y: i32) {
    if size() == -1 && txt.len() == 1 {
        return;
    }
    let txt = CString::safe_new(txt);
    unsafe { Fl_draw(txt.as_ptr(), x, y) }
}

/// Draws a string starting at the given x, y location with width and height and alignment
pub fn draw_text_boxed(string: &str, x: i32, y: i32, width: i32, height: i32, align: Align) {
    if size() == -1 && string.len() == 1 {
        return;
    }
    let s = CString::safe_new(string);
    unsafe { Fl_draw_text_boxed(s.as_ptr(), x, y, width, height, align.bits()) }
}

/// Draws a string starting at the given x, y location, rotated to an angle
pub fn draw_text_angled(angle: i32, txt: &str, x: i32, y: i32) {
    if size() == -1 && txt.len() == 1 {
        return;
    }
    let txt = CString::safe_new(txt);
    unsafe { Fl_draw_text_angled(angle, txt.as_ptr(), x, y) }
}

/// Draws a UTF-8 string right to left starting at the given x, y location
pub fn rtl_draw(txt: &str, x: i32, y: i32) {
    if size() == -1 && txt.len() == 1 {
        return;
    }
    let len = txt.len() as i32;
    let txt = CString::safe_new(txt);
    unsafe { Fl_rtl_draw(txt.as_ptr(), len, x, y) }
}

/// Draws a series of line segments around the given box.
///
/// The string must contain groups of 4 letters which specify one of 24 standard
/// grayscale values, where 'A' is black and 'X' is white.
/// The order of each set of 4 characters is: top, left, bottom, right.
pub fn draw_frame(string: &str, x: i32, y: i32, width: i32, height: i32) {
    assert!(string.len() % 4 == 0);
    let s = CString::safe_new(string);
    unsafe { Fl_frame(s.as_ptr(), x, y, width, height) }
}

/// Draws a series of line segments around the given box
///
/// Differs from `frame()` by the order of the line segments which is bottom, right, top, left.
pub fn draw_frame2(string: &str, x: i32, y: i32, width: i32, height: i32) {
    assert!(string.len() % 4 == 0);
    let s = CString::safe_new(string);
    unsafe { Fl_frame2(s.as_ptr(), x, y, width, height) }
}

/// Draws a box given the box type, size, position and color
pub fn draw_box(box_type: FrameType, x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_draw_box(box_type.as_i32(), x, y, w, h, color.bits()) }
}

/// Checks whether platform supports true alpha blending for RGBA images
pub fn can_do_alpha_blending() -> bool {
    unsafe { Fl_can_do_alpha_blending() != 0 }
}

/// Get a human-readable string from a shortcut value
pub fn shortcut_label(shortcut: Shortcut) -> String {
    unsafe {
        let x = Fl_shortcut_label(shortcut.bits() as u32);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a selection rectangle, erasing a previous one by XOR'ing it first.
pub fn overlay_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_overlay_rect(x, y, w, h) }
}

/// Erase a selection rectangle without drawing a new one
pub fn overlay_clear() {
    unsafe { Fl_overlay_clear() }
}

/// Sets the cursor style
pub fn set_cursor(cursor: Cursor) {
    unsafe { Fl_set_cursor(cursor as i32) }
}

/// Sets the cursor style
pub fn set_cursor_with_color(cursor: Cursor, fg: Color, bg: Color) {
    unsafe { Fl_set_cursor_with_color(cursor as i32, fg.bits() as i32, bg.bits() as i32) }
}

/// Sets the status
pub fn set_status(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_set_status(x, y, w, h) }
}

/// Sets spot within the window
pub fn set_spot<Win: WindowExt>(font: Font, size: i32, x: i32, y: i32, w: i32, h: i32, win: &Win) {
    unsafe {
        Fl_set_spot(
            font.bits(),
            size,
            x,
            y,
            w,
            h,
            win.as_widget_ptr() as *mut raw::c_void,
        );
    }
}

/// Resets the spot within the window
pub fn reset_spot() {
    unsafe { Fl_reset_spot() }
}

/**
    Captures the window and returns raw data.
    Example usage:
    ```rust,no_run
    use fltk::{prelude::*, *};
    let mut win = window::Window::default();
    let image = draw::capture_window(&mut win).unwrap();
    ```
    # Errors
    The api can fail to capture the window as an image
*/
pub fn capture_window<Win: WindowExt>(win: &mut Win) -> Result<RgbImage, FltkError> {
    let cp = win.w() * win.h() * 3;
    win.show();
    unsafe {
        let x = Fl_read_image(std::ptr::null_mut(), 0, 0, win.w(), win.h(), 0);
        if x.is_null() {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize);
            Ok(RgbImage::new(x, win.w(), win.h(), ColorDepth::Rgb8)?)
        }
    }
}

/**
    Captures part of the window and returns raw data.
    Example usage:
    ```rust,no_run
    use fltk::{prelude::*, *};
    let mut win = window::Window::default();
    let image = draw::capture_window(&mut win).unwrap();
    ```
    # Errors
    The api can fail to capture the window as an image
*/
pub fn capture_window_part<Win: WindowExt>(
    win: &mut Win,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> Result<RgbImage, FltkError> {
    let cp = win.w() * win.h() * 3;
    win.show();
    unsafe {
        let x = Fl_capture_window_part(win.as_widget_ptr() as _, x, y, w, h);
        if x.is_null() {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize);
            Ok(RgbImage::new(x, w, h, ColorDepth::Rgb8)?)
        }
    }
}

/**
    Captures the image surface object and returns raw data.
    Example usage:
    ```rust,no_run
    use fltk::{prelude::*, *};
    let mut surface = surface::ImageSurface::new(100, 100, false);
    let image = draw::capture_surface(&mut surface, 100, 100).unwrap();
    ```
    # Errors
    The api can fail to capture the image surface as an image
*/
pub fn capture_surface(surface: &ImageSurface, w: i32, h: i32) -> Result<RgbImage, FltkError> {
    let cp = w * h * 3;
    unsafe {
        ImageSurface::push_current(surface);
        let x = Fl_read_image(std::ptr::null_mut(), 0, 0, w, h, 0);
        ImageSurface::pop_current();
        if x.is_null() {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize);
            Ok(RgbImage::new(x, w, h, ColorDepth::Rgb8)?)
        }
    }
}

/// Draw an image into a widget.
/// Requires a call to [`app::set_visual(Mode::Rgb8).unwrap()`](`crate::app::set_visual`).
/// Doesn't support transparency, for that you would have to use `RgbImage`.
/// # Errors
/// Errors on invalid or unsupported image formats
pub fn draw_image(
    data: &[u8],
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: ColorDepth,
) -> Result<(), FltkError> {
    let sz = (w * h * depth as i32) as usize;
    if sz > data.len() {
        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
    }
    unsafe {
        Fl_draw_image(data.as_ptr(), x, y, w, h, depth as i32, 0);
    }
    Ok(())
}

/// Draw a check mark
pub fn draw_check(x: i32, y: i32, w: i32, h: i32, col: Color) {
    unsafe {
        Fl_draw_check(x, y, w, h, col.bits());
    }
}

/// Draw an image into a widget.
/// Requires a call to [`app::set_visual(Mode::Rgb8).unwrap()`](`crate::app::set_visual`).
/// A negative depth flips the image horizontally,
/// while a negative line data flips it vertically.
/// Allows passing a line-data parameter
/// # Errors
/// Errors on invalid or unsupported image formats
/// # Safety
/// Passing wrong line data can read to over or underflow
pub unsafe fn draw_image_ext(data: &[u8], x: i32, y: i32, w: i32, h: i32, depth: i32, line_data: i32) {
    unsafe {
        Fl_draw_image(data.as_ptr(), x, y, w, h, depth, line_data);
    }
}

/// Draws a rounded box
pub fn draw_rbox(x: i32, y: i32, w: i32, h: i32, max_radius: i32, fill: bool, col: Color) {
    let max_radius = if max_radius < 0 { 0 } else { max_radius };
    let offset: [f64; 5] = [0.0, 0.07612, 0.29289, 0.61732, 1.0];
    let mut rs = w * 2 / 5;
    let rsy = h * 2 / 5;
    if rs > rsy {
        rs = rsy;
    }
    if rs > max_radius {
        rs = max_radius;
    }
    if rs == 5 {
        rs = 4;
    }
    if rs == 7 {
        rs = 8;
    }

    let rs = f64::from(rs);
    let x = f64::from(x);
    let y = f64::from(y);
    let w = f64::from(w);
    let h = f64::from(h);
    let old_col = get_color();
    let len = offset.len();

    set_draw_color(col);
    if fill {
        begin_polygon();
    } else {
        begin_loop();
    }
    unsafe {
        for i in 0..len {
            vertex(
                0.5 + x + offset.get_unchecked(len - i - 1) * rs,
                0.5 + y + offset.get_unchecked(i) * rs,
            );
        }
        for i in 0..len {
            vertex(
                0.5 + x + offset.get_unchecked(i) * rs,
                0.5 + y + h - 1.0 - offset.get_unchecked(len - i - 1) * rs,
            );
        }
        for i in 0..len {
            vertex(
                0.5 + x + w - 1.0 - offset.get_unchecked(len - i - 1) * rs,
                0.5 + y + h - 1.0 - offset.get_unchecked(i) * rs,
            );
        }
        for i in 0..len {
            vertex(
                0.5 + x + w - 1.0 - offset.get_unchecked(i) * rs,
                0.5 + y + offset.get_unchecked(len - i - 1) * rs,
            );
        }
    }
    if fill {
        end_polygon();
    } else {
        end_loop();
    }
    set_draw_color(old_col);
}

#[cfg(feature = "enable-glwindow")]
/// Start drawing using OpenGL functions inside a widget's draw routine
/// # Safety
/// Requires OpenGL support, Only works with SingleWindow
pub unsafe fn gl_start() {
    unsafe {
        fltk_sys::window::Fl_gl_start();
    }
}

#[cfg(feature = "enable-glwindow")]
/// Finish drawing using OpenGL functions inside a widget's draw routine
/// # Safety
/// Requires OpenGL support, Only works with SingleWindow
pub unsafe fn gl_finish() {
    unsafe {
        fltk_sys::window::Fl_gl_finish();
    }
}

/// Draws a rounded rectangle
pub fn draw_rounded_rect(x: i32, y: i32, w: i32, h: i32, r: i32) {
    unsafe { Fl_rounded_rect(x, y, w, h, r) }
}

/// Draws a filled rounded rectangle
pub fn draw_rounded_rectf(x: i32, y: i32, w: i32, h: i32, r: i32) {
    unsafe { Fl_rounded_rectf(x, y, w, h, r) }
}

/// Draws a filled circle
pub fn draw_circle_fill(x: i32, y: i32, d: i32, c: Color) {
    unsafe {
        Fl_draw_circle(x, y, d, c.bits());
    }
}

/// Like `draw_text`, however uses FLTK's `fl_draw` which takes the length of the string, so it doesn't need to allocate
pub fn draw_text_n(string: &str, x: i32, y: i32) {
    let len = string.len();
    if size() == -1 && len == 1 {
        return;
    }
    unsafe { Fl_draw_text_n(string.as_ptr() as _, len as _, x, y) }
}

/// Override the drawing scale
pub fn override_scale() -> f32 {
    unsafe { Fl_override_scale() }
}

/// Restore the drawing scale
pub fn restore_scale(s: f32) {
    unsafe { Fl_restore_scale(s) }
}
