use fltk_sys::fl;

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe { ((fl::Fl_screen_w() as f64), (fl::Fl_screen_h() as f64)) }
}

/// Returns a pair of the x & y coords of the screen
pub fn screen_coords() -> (i32, i32) {
    unsafe { (fl::Fl_screen_x(), fl::Fl_screen_y()) }
}

/// Set the screen scale
pub fn set_screen_scale(n: i32, factor: f32) {
    unsafe { fl::Fl_set_screen_scale(n as i32, factor) }
}

/// Get the screen scale
pub fn screen_scale(n: i32) -> f32 {
    unsafe { fl::Fl_screen_scale(n as i32) }
}

/// Return whether scaling the screen is supported
pub fn screen_scaling_supported() -> bool {
    unsafe { fl::Fl_screen_scaling_supported() != 0 }
}

/// Get the screen count
pub fn screen_count() -> i32 {
    unsafe { fl::Fl_screen_count() as i32 }
}

/// Get the screen number based on its coordinates
pub fn screen_num(x: i32, y: i32) -> i32 {
    unsafe { fl::Fl_screen_num(x, y) }
}

/// Get a screen's dpi resolution
/// # Returns
/// (vertical, horizontal) resolutions
pub fn screen_dpi(screen_num: i32) -> (f32, f32) {
    let mut h: f32 = 0.;
    let mut v: f32 = 0.;
    unsafe {
        fl::Fl_screen_dpi(&mut h, &mut v, screen_num);
    }
    (h, v)
}

/// Get a screen's xywh
pub fn screen_xywh(screen_num: i32) -> (i32, i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    let mut h = 0;
    unsafe {
        fl::Fl_screen_xywh(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
}

/// Get a screen's working area
pub fn screen_work_area(screen_num: i32) -> (i32, i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    let mut h = 0;
    unsafe {
        fl::Fl_screen_work_area(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
}
