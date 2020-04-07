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

/// Draws a rectangle with border color
pub fn draw_rect_with_color(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe {
        cfl_rect_with_color(x, y, w, h, color as u32)
    }
}

/// Draws a loop
pub fn draw_loop(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe {
        cfl_loop(x1, y1, x2, y2, x3, y3);
    }
}

/// Draws a filled rectangle
pub fn draw_rect_fill(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe {
        cfl_rectf_with_color(x, y, w, h, color as u32)
    }
}

/// Draws a focus rectangle
pub fn draw_focus_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        cfl_focus_rect(x, y, w, h)
    }
}

/// Sets the drawing color
pub fn set_draw_color(color: Color) {
    unsafe {
        cfl_set_color_int(color.to_u32())
    }
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

// /// Reads the drawn image from a region
// pub fn read_image(x: i32, y: i32, w: i32, h: i32) -> Vec<u8> {
//     let cp = w as usize * h as usize * 3;
//     unsafe {
//         let x = cfl_read_image(std::ptr::null_mut() as *mut std::os::raw::c_uchar, x, y, w, h, 0);
//         assert!(!x.is_null(), "Failed to read image from region!");
//         Vec::from_raw_parts(x, cp, cp)
//     }
// }

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