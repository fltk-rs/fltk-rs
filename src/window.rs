use crate::app::*;
use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::window::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct Window {
    _inner: *mut Fl_Window,
}

/// Defines the window type, can be set dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum WindowType {
    NormalWindow = 240,
    DoubleWindow = 241,
}

/// Creates a double window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
}

/// Creates a Menu window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct MenuWindow {
    _inner: *mut Fl_Menu_Window,
}

/// A wrapper around a raw OpenGL context
#[derive(Debug)]
pub struct GlContext {
    _inner: *mut raw::c_void,
}

impl GlContext {
    /// Create a GlContext from an opaque gl context pointer
    pub unsafe fn from_raw(ptr: *mut raw::c_void) -> GlContext {
        GlContext { _inner: ptr, }
    }

    /// Returns the underlying pointer
    pub unsafe fn into_raw(self) -> *mut raw::c_void {
        self._inner
    }
}

/// Creates a OpenGL window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct GlWindow {
    _inner: *mut Fl_Gl_Window,
}

impl GlWindow {
    /// Flush window content
    pub fn flush(&mut self) {
        unsafe { Fl_Gl_Window_flush(self._inner) }
    }

    /// Returns whether the OpeGL context is still valid
    pub fn valid(&self) -> bool {
        unsafe {
            match Fl_Gl_Window_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the OpeGL context as still valid
    pub fn set_valid(&mut self, v: bool) {
        unsafe { Fl_Gl_Window_set_valid(self._inner, v as i8) }
    }

    /// Returns whether the context is valid upon creation
    pub fn context_valid(&self) -> bool {
        unsafe {
            match Fl_Gl_Window_context_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the context as valid upon creation
    pub fn set_context_valid(&mut self, v: bool) {
        unsafe { Fl_Gl_Window_set_context_valid(self._inner, v as i8) }
    }

    /// Returns the GlContext
    pub fn context(&self) -> Option<GlContext> {
        unsafe {
            let x = Fl_Gl_Window_context(self._inner);
            if x.is_null() {
                None
            } else {
                Some(GlContext { _inner: x })
            }
        }
    }

    /// Sets the GlContext
    pub fn set_context(&mut self, ctx: GlContext, destroy_flag: bool) {
        unsafe { Fl_Gl_Window_set_context(self._inner, ctx._inner, destroy_flag as i32) }
    }

    /// Swaps the back and front buffers
    pub fn swap_buffers(&mut self) {
        unsafe { Fl_Gl_Window_swap_buffers(self._inner) }
    }

    /// Sets the projection so 0,0 is in the lower left of the window
    /// and each pixel is 1 unit wide/tall.
    pub fn ortho(&mut self) {
        unsafe { Fl_Gl_Window_ortho(self._inner) }
    }

    /// Returns whether the GlWindow can do overlay
    pub fn can_do_overlay(&mut self) -> bool {
        unsafe {
            match Fl_Gl_Window_can_do_overlay(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Redraws the overlay
    pub fn redraw_overlay(&mut self) {
        unsafe { Fl_Gl_Window_redraw_overlay(self._inner) }
    }

    /// Hides the overlay
    pub fn hide_overlay(&mut self) {
        unsafe { Fl_Gl_Window_hide_overlay(self._inner) }
    }

    /// Makes the overlay current
    pub fn make_overlay_current(&mut self) {
        unsafe { Fl_Gl_Window_make_overlay_current(self._inner) }
    }

    /// Returns the pixels per unit
    pub fn pixels_per_unit(&mut self) -> f32 {
        unsafe { Fl_Gl_Window_pixels_per_unit(self._inner) }
    }

    /// Gets the window's width in pixels
    pub fn pixel_w(&mut self) -> i32 {
        unsafe { Fl_Gl_Window_pixel_w(self._inner) }
    }

    /// Gets the window's height in pixels
    pub fn pixel_h(&mut self) -> i32 {
        unsafe { Fl_Gl_Window_pixel_h(self._inner) }
    }
    
    /// Get the Mode of the GlWindow
    pub fn mode(&self) -> Mode {
        unsafe {
            mem::transmute(Fl_Gl_Window_mode(self._inner))
        }
    }

    /// Set the Mode of the GlWindow
    pub fn set_mode(&mut self, mode: Mode) {
        unsafe {
            Fl_Gl_Window_set_mode(self._inner, mode as i32);
        }
    }
}
