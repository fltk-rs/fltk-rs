use crate::app::init::CURRENT_FRAME;
use crate::enums::{Color, FrameType, Mode};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::fl;
use std::{ffi::CString, mem, os::raw, sync::atomic::Ordering};

/// Set the app scheme
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Scheme {
    /// Base fltk scheming
    Base,
    /// inspired by the Aqua user interface on Mac OS X
    Plastic,
    /// inspired by the GTK+ theme
    Gtk,
    /// inspired by the Clearlooks Glossy scheme
    Gleam,
    /// Subset of Dmitrij K's oxy scheme
    Oxy,
}

/// sets the scheme of the application
pub fn set_scheme(scheme: Scheme) {
    let name_str = match scheme {
        Scheme::Base => "base",
        Scheme::Gtk => "gtk+",
        Scheme::Gleam => "gleam",
        Scheme::Plastic => "plastic",
        Scheme::Oxy => "oxy",
    };
    let name_str = CString::safe_new(name_str);
    unsafe {
        fl::Fl_set_scheme(name_str.as_ptr());
    }
}

/// Gets the scheme of the application
pub fn scheme() -> Scheme {
    unsafe {
        use Scheme::{Base, Gleam, Gtk, Oxy, Plastic};
        match fl::Fl_scheme() {
            0 => Base,
            1 => Gtk,
            2 => Gleam,
            3 => Plastic,
            4 => Oxy,
            _ => unreachable!(),
        }
    }
}

/// Alias Scheme to `AppScheme`
pub type AppScheme = Scheme;

/// Set the application's scrollbar size
pub fn set_scrollbar_size(sz: i32) {
    unsafe { fl::Fl_set_scrollbar_size(sz as i32) }
}

/// Get the app's scrollbar size
pub fn scrollbar_size() -> i32 {
    unsafe { fl::Fl_scrollbar_size() as i32 }
}

/// Return whether visible focus is shown
pub fn visible_focus() -> bool {
    unsafe { fl::Fl_visible_focus() != 0 }
}

/// Show focus around widgets
pub fn set_visible_focus(flag: bool) {
    unsafe { fl::Fl_set_visible_focus(flag as i32) }
}

/// Set the app's default frame type
pub fn set_frame_type(new_frame: FrameType) {
    unsafe {
        let new_frame = new_frame as i32;
        let curr = CURRENT_FRAME.load(Ordering::Relaxed);
        fl::Fl_set_box_type(curr, new_frame);
        CURRENT_FRAME.store(new_frame, Ordering::Relaxed);
    }
}

/// Set the app's default frame type without storing the old type
pub fn set_frame_type2(old_frame: FrameType, new_frame: FrameType) {
    unsafe {
        fl::Fl_set_box_type(old_frame as i32, new_frame as i32);
    }
}

/// Set the app's default frame type
pub fn set_frame_type_cb(
    old_frame: FrameType,
    cb: fn(x: i32, y: i32, w: i32, h: i32, c: Color),
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    unsafe {
        fl::Fl_set_box_type_cb(old_frame as i32, Some(mem::transmute(cb)), x, y, w, h);
    }
}

/// Get the app's frame type
pub fn frame_type() -> FrameType {
    let curr = CURRENT_FRAME.load(Ordering::Relaxed);
    FrameType::by_index(curr as _)
}

/// Swap the default frame type with a new frame type
pub fn swap_frame_type(new_frame: FrameType) {
    unsafe {
        let new_frame = new_frame as i32;
        let curr = CURRENT_FRAME.load(Ordering::Relaxed);
        fl::Fl_set_box_type(56, curr);
        fl::Fl_set_box_type(curr, new_frame);
        fl::Fl_set_box_type(new_frame, 56);
        CURRENT_FRAME.store(new_frame, Ordering::Relaxed);
    }
}

/// Get the shadow width for frames types with shadows
pub fn frame_shadow_width() -> i32 {
    unsafe { fl::Fl_box_shadow_width() }
}

/// Set the shadow width for frames types with shadows
pub fn set_frame_shadow_width(width: i32) {
    unsafe { fl::Fl_set_box_shadow_width(width) }
}

/// Get the max border radius for frame types
pub fn frame_border_radius_max() -> i32 {
    unsafe { fl::Fl_box_border_radius_max() }
}

/// Set the max border radius for frame types
pub fn set_frame_border_radius_max(radius: i32) {
    unsafe { fl::Fl_set_box_border_radius_max(radius) }
}

/// Makes FLTK use its own colormap. This may make FLTK display better
pub fn own_colormap() {
    unsafe { fl::Fl_own_colormap() }
}

/// Set the foreground color
pub fn foreground(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_foreground(r, g, b) }
}

/// Set the background color
pub fn background(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background(r, g, b) }
}

/// Set the background color for input and text widgets
pub fn background2(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background2(r, g, b) }
}

/// Set the foreground color
pub fn set_foreground_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_foreground(r, g, b) }
}

/// Set the background color
pub fn set_background_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background(r, g, b) }
}

/// Set the background color for input and text widgets
pub fn set_background2_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background2(r, g, b) }
}

/// Sets the app's default selection color
pub fn set_selection_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_selection_color(r, g, b) }
}

/// Sets the app's default selection color
pub fn set_inactive_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_inactive_color(r, g, b) }
}

/// Swap a color with a custom RGB value
pub fn set_color(old: Color, r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_set_color(old.bits() as u32, r, g, b) }
}

#[cfg(feature = "enable-glwindow")]
/// Swap a color with a custom RGBA value
pub fn set_color_with_alpha(old: Color, r: u8, g: u8, b: u8, a: u8) {
    unsafe { fl::Fl_set_color_with_alpha(old.bits() as u32, r, g, b, a) }
}

/// Gets the system colors
pub fn get_system_colors() {
    unsafe { fl::Fl_get_system_colors() }
}

/// Reload the app scheme
pub fn reload_scheme() -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_reload_scheme() {
            1 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
        }
    }
}

/// Get the default menu line-spacing
pub fn menu_linespacing() -> i32 {
    unsafe { fl::Fl_menu_linespacing() }
}

/// Set the menu line-spacing
pub fn set_menu_linespacing(val: i32) {
    unsafe { fl::Fl_set_menu_linespacing(val) }
}

/// Sets the visual mode of the application
/// # Errors
/// Returns Err(FailedOperation) if FLTK failed to set the visual mode
pub fn set_visual(mode: Mode) -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_visual(mode.bits() as i32) {
            0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            _ => Ok(()),
        }
    }
}

#[cfg(feature = "enable-glwindow")]
/// Sets the OpenGL visual mode of the application
/// # Errors
/// Returns Err(FailedOperation) if FLTK failed to set the visual mode
pub fn set_gl_visual(mode: Mode) -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_gl_visual(mode.bits() as i32) {
            0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            _ => Ok(()),
        }
    }
}

/// The current graphics context of the app, `fl_gc`.
/// `*mut c_void` to `HDC` on Windows, `CGContextRef` on macOS, `_XGC` on X11
pub type GraphicsContext = *mut raw::c_void;

/// Get the graphics context, `fl_gc`
pub fn graphics_context() -> GraphicsContext {
    unsafe {
        let ctx = fltk_sys::window::Fl_gc();
        assert!(!ctx.is_null());
        ctx
    }
}

/// The display global variable, `fl_display`.
/// `_XDisplay` on X11, `HINSTANCE` on Windows.
pub type Display = *mut raw::c_void;

/// Gets the display global variable, `fl_display`.
/// `_XDisplay` on X11, `HINSTANCE` on Windows.
pub fn display() -> Display {
    unsafe {
        let disp = fltk_sys::window::Fl_display();
        assert!(!disp.is_null());
        disp
    }
}

/// Causes all the windows that need it to be redrawn and graphics forced out through the pipes.
pub fn flush() {
    assert!(crate::app::is_ui_thread());
    unsafe { fl::Fl_flush() }
}

/// Redraws everything
pub fn redraw() {
    unsafe { fl::Fl_redraw() }
}

/// Open the current display
/// # Safety
/// A correct visual must be set prior to opening the display
pub unsafe fn open_display() {
    fl::Fl_open_display()
}

/// Close the current display
/// # Safety
/// The display shouldn't be closed while a window is shown
pub unsafe fn close_display() {
    fl::Fl_close_display()
}

/// Determines if the currently drawn box is active or inactive
pub fn draw_frame_active() -> bool {
    unsafe { fl::Fl_draw_box_active() != 0 }
}

/// Fl::box_color.
/// Gets the current frame color within box/frame drawing mode
pub fn frame_color(col: Color) -> Color {
    unsafe { mem::transmute(fl::Fl_box_color(col.bits())) }
}

/// Fl::set_box_color.
/// Sets the current frame color within box/frame drawing mode
pub fn set_frame_color(col: Color) {
    unsafe { fl::Fl_set_box_color(col.bits()) }
}

/// Add a new symbol, that can be accessed using the `@` prefix
pub fn add_symbol(label: &str, scalable: bool, draw_cb: fn(Color)) -> Result<(), FltkError> {
    unsafe {
        let label = CString::safe_new(label);
        let ret = fltk_sys::draw::Fl_add_symbol(
            label.into_raw() as _,
            mem::transmute(Some(draw_cb)),
            scalable as _,
        );
        if ret == 0 {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            Ok(())
        }
    }
}
