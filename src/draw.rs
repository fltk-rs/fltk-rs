pub use crate::prelude::*;
use fltk_sys::draw::*;
// use crate::image::Image;
// use std::{ffi::{CStr, CString}, mem, os::raw};

/// Draws a line
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        cfl_line(x1, y1, x2, y2);
    }
}

/// Draws a point
pub fn draw_point(x: i32, y: i32) {
    unsafe {
        cfl_point(x, y)
    }
}

/// Draws a rectangle
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        cfl_rect(x, y, w, h)
    }
}

/// Draws a filled rectangle
pub fn draw_rect_fill(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe {
        cfl_rectf_with_color(x, y, w, h, color as i32)
    }
}

/// Sets the drawing color
pub fn set_draw_color(color: Color) {
    unsafe {
        cfl_set_color_int(color.to_u32())
    }
}

/// Sets the line style
pub fn set_line_style(style: LineStyle, width: i32) {
    unsafe {
        cfl_line_style(style as i32, width, std::ptr::null_mut() as *mut std::os::raw::c_char);
    }
}

/// Not sure what this does!
pub fn push_clip(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        cfl_push_clip(x, y, w, h);
    }
}

/// Nor this!
pub fn pop_clip() {
    unsafe {
        cfl_pop_clip();
    }
}