// https://www.fltk.org/doc-1.4/group__fl__screen.html

use fltk_sys::fl;

use crate::{
    draw::{Coordinates, Rect},
    prelude::{FltkError, FltkErrorKind},
};
type Coord = Coordinates<i32>; // TEMP

/// An available screen
///
/// Unlike the standalone functions, it automatically checks the provided
/// coordinates and screen numbers are currently valid and inside boundaries.
#[derive(Debug, Copy, Clone)]
pub struct Screen {
    /// The screen number
    pub n: i32,
}

impl Screen {
    // constructors

    /// Returns a vector containing all the `Screen`s, ordered by screen number
    pub fn all_screens() -> Vec<Screen> {
        let mut screens: Vec<Self> = vec![];
        for n in 0..Self::count() {
            screens.push(Self::new(n).unwrap());
        }
        screens
    }

    /// Returns the `Screen` associated with the given screen number
    ///
    /// Returns an error if the provided `number` is not a valid screen number.
    pub fn new(number: i32) -> Result<Screen, FltkError> {
        let s = Screen { n: number };
        if s.is_valid() {
            Ok(s)
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the `Screen` that contains the specified screen position
    ///
    /// Returns an error if the provided coordinates are out of bounds.
    pub fn new_at<C: Into<Coord> + Copy>(pos: C) -> Result<Screen, FltkError> {
        let pos: Coord = pos.into();

        let s = Screen {
            n: unsafe { fl::Fl_screen_num(pos.x, pos.y) },
        };

        if Self::is_coord_inside_any_work_area(pos) {
            Ok(s)
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the `Screen` which intersects the most with the provided rectangle
    ///
    /// Returns an error if any coordinates on the provided retangle are out of bounds.
    pub fn new_inside<R: Into<Rect> + Copy>(rect: R) -> Result<Screen, FltkError> {
        let r: Rect = rect.into();
        let s = Screen {
            n: unsafe { fl::Fl_screen_num_inside(r.x, r.y, r.w, r.h) },
        };

        if Self::is_rect_inside_any_work_area(r) {
            Ok(s)
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    // associated functions

    /// Returns true if scaling factors are supported by this platform,
    /// wether shared by all screens, or each screen having its own value
    pub fn scaling_supported() -> bool {
        unsafe { fl::Fl_screen_scaling_supported() != 0 }
    }

    /// Returns true if each screen can have its own scaling factor value
    pub fn scaling_supported_separately() -> bool {
        unsafe { fl::Fl_screen_scaling_supported() == 2 }
    }

    /// Returns the number of available screens
    pub fn count() -> i32 {
        unsafe { fl::Fl_screen_count() }
    }

    /// Returns the screen number of the screen that contains
    /// the specified screen position coordinates
    ///
    /// Returns an error if the provided coordinates are out of bounds.
    pub fn num_at<C: Into<Coord> + Copy>(pos: C) -> Result<i32, FltkError> {
        let pos: Coord = pos.into();
        if Self::is_coord_inside_any_work_area(pos) {
            Ok(unsafe { fl::Fl_screen_num(pos.x, pos.y) })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the bounding rectangle of the work area of the screen that
    /// contains the specified screen position coordinates
    ///
    /// Returns an error if the provided coordinates are out of bounds.
    pub fn work_area_at<C: Into<Coord> + Copy>(pos: C) -> Result<Rect, FltkError> {
        let pos: Coord = pos.into();
        if Self::is_coord_inside_any_work_area(pos) {
            let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
            unsafe { fl::Fl_screen_work_area_at(&mut x, &mut y, &mut w, &mut h, pos.x, pos.y) }
            Ok(Rect { x, y, w, h })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the bounding rectangle of the work area of the screen currently under
    /// the mouse pointer coordinates
    pub fn work_area_mouse() -> Rect {
        let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
        unsafe { fl::Fl_screen_work_area_mouse(&mut x, &mut y, &mut w, &mut h) }
        Rect { x, y, w, h }
    }

    /// Returns the bounding rectangle of the work area
    /// with the provided screen `number`
    ///
    /// Returns an error if the provided `number` is not a valid screen number.
    pub fn work_area_num(number: i32) -> Result<Rect, FltkError> {
        let s = Screen { n: number };
        if s.is_valid() {
            let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
            unsafe { fl::Fl_screen_work_area(&mut x, &mut y, &mut w, &mut h, number) }
            Ok(Rect { x, y, w, h })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the bounding rectangle of the screen that
    /// contains the specified screen position coordinates
    ///
    /// Returns an error if the provided coordinates are out of bounds.
    pub fn xywh_at<C: Into<Coord> + Copy>(pos: C) -> Result<Rect, FltkError> {
        let pos: Coord = pos.into();
        if Self::is_coord_inside_any_xywh(pos) {
            let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
            unsafe { fl::Fl_screen_xywh_at(&mut x, &mut y, &mut w, &mut h, pos.x, pos.y) }
            Ok(Rect { x, y, w, h })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Returns the bounding rectangle of the screen that
    /// contains the specified screen position coordinates
    ///
    /// Returns an error if the provided coordinates are out of bounds.
    pub fn xywh_inside<R: Into<Rect> + Copy>(rect: R) -> Result<Rect, FltkError> {
        let r: Rect = rect.into();
        if Self::is_rect_inside_any_xywh(r) {
            let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
            unsafe { fl::Fl_screen_xywh_inside(&mut x, &mut y, &mut w, &mut h, r.x, r.y, r.w, r.h) }
            Ok(Rect { x, y, w, h })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Gets the bounding rectangle of the screen currently under
    /// the mouse pointer coordinates
    pub fn xywh_mouse() -> Rect {
        let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
        unsafe { fl::Fl_screen_xywh_mouse(&mut x, &mut y, &mut w, &mut h) }
        Rect { x, y, w, h }
    }

    /// Returns the bounding rectangle of the screen
    /// with the provided screen `number`
    ///
    /// Returns an error if the provided `number` is not a valid screen number.
    pub fn xywh_num(number: i32) -> Result<Rect, FltkError> {
        let s = Screen { n: number };
        if s.is_valid() {
            let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
            unsafe {
                fl::Fl_screen_xywh(&mut x, &mut y, &mut w, &mut h, number);
            }
            Ok(Rect { x, y, w, h })
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Controls the possibility to scale all windows by ctrl/+/-/0/ or cmd/+/-/0/
    ///
    /// This function should be called before `app::open_display` runs.
    /// If it is not called, the default is to handle these keys for window scaling.
    ///
    /// Pass a `value` of `false` to stop recognition of ctrl/+/-/0/
    /// (or cmd/+/-/0/ under macOS) keys as window scaling.
    pub fn keyboard_scaling(value: bool) {
        unsafe { fl::Fl_keyboard_screen_scaling(value as _) }
    }

    // methods

    /// Returns `true` if the current screen's number corresponds
    /// to a currently connected screen, or `false` otherwise.
    pub fn is_valid(&self) -> bool {
        self.n >= 0 && self.n < Self::count()
    }

    /// Returns the current screen (vertical, horizontal) resolution in dots-per-inch
    pub fn dpi(&self) -> (f32, f32) {
        let mut h: f32 = 0.;
        let mut v: f32 = 0.;
        unsafe {
            fl::Fl_screen_dpi(&mut h, &mut v, self.n);
        }
        (h, v)
    }

    /// Sets the value of the GUI scaling factor for the current screen
    pub fn set_scale(&self, factor: f32) {
        unsafe { fl::Fl_set_screen_scale(self.n, factor) }
    }

    /// Returns the value of the GUI scaling factor for the current screen
    pub fn scale(&self) -> f32 {
        unsafe { fl::Fl_screen_scale(self.n) }
    }

    /// Returns the the work area of the current screen
    pub fn work_area(&self) -> Rect {
        let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
        unsafe { fl::Fl_screen_work_area(&mut x, &mut y, &mut w, &mut h, self.n) }
        Rect { x, y, w, h }
    }

    /// Returns the topmost y coordinate of the current screen's work area
    pub fn y(&self) -> i32 {
        self.work_area().y
    }

    /// Returns the bottom-right x coordinate of the current screen's work area
    pub fn x(&self) -> i32 {
        self.work_area().x
    }

    /// Returns the width in pixels of the current screen's work area
    pub fn w(&self) -> i32 {
        self.work_area().w
    }

    /// Returns the height in pixels of the current screen's work area
    pub fn h(&self) -> i32 {
        self.work_area().h
    }

    /// Returns the top-left `x,y` coordinates of the current screen's work area
    pub fn top_left(&self) -> Coord {
        self.work_area().top_left()
    }

    /// Returns the bottom-right `x+w, y+h` coordinates of the current screen's work area
    pub fn bottom_right(&self) -> Coord {
        self.work_area().bottom_right()
    }

    // private methods

    // returns `true` if the provided position is inside the bounds
    // of any current work area boundaries, or `false` otherwise.
    fn is_coord_inside_any_work_area<C: Into<Coord> + Copy>(c: C) -> bool {
        let c: Coord = c.into();
        let main_wa: Rect = screen_work_area(0).into();
        // returns false if we get `0` but the coords are outside main screen's
        !(screen_num(c.x, c.y) == 0
            && (c.x < main_wa.x
                || c.y < main_wa.y
                || c.x >= main_wa.bottom_right().x
                || c.y >= main_wa.bottom_right().y))
    }
    // returns `true` if the provided rect is fully inside the bounds
    // of any current work area boundaries, or `false` otherwise.
    fn is_rect_inside_any_work_area<R: Into<Rect> + Copy>(r: R) -> bool {
        let r: Rect = r.into();
        Self::is_coord_inside_any_work_area(r.top_left())
            && Self::is_coord_inside_any_work_area(r.bottom_right())
    }

    // returns `true` if the provided position is inside the bounds
    // of any current screen xywh boundaries, or `false` otherwise.
    fn is_coord_inside_any_xywh<C: Into<Coord> + Copy>(c: C) -> bool {
        let c: Coord = c.into();
        let main_xywh: Rect = screen_xywh(0).into();
        // returns false if we get `0` but the coords are outside main screen's
        !(screen_num(c.x, c.y) == 0
            && (c.x < main_xywh.x
                || c.y < main_xywh.y
                || c.x >= main_xywh.bottom_right().x
                || c.y >= main_xywh.bottom_right().y))
    }
    // returns `true` if the provided rect is fully inside the bounds
    // of any current screeen xywh boundaries, or `false` otherwise.
    fn is_rect_inside_any_xywh<R: Into<Rect> + Copy>(r: R) -> bool {
        let r: Rect = r.into();
        Self::is_coord_inside_any_xywh(r.top_left())
            && Self::is_coord_inside_any_xywh(r.bottom_right())
    }
}

// standalone functions

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe { ((fl::Fl_screen_w() as f64), (fl::Fl_screen_h() as f64)) }
}

/// Returns a pair of the x & y coords of the screen
pub fn screen_coords() -> (i32, i32) {
    unsafe { (fl::Fl_screen_x(), fl::Fl_screen_y()) }
}

/// Sets the screen scale
pub fn set_screen_scale(n: i32, factor: f32) {
    unsafe { fl::Fl_set_screen_scale(n, factor) }
}

/// Returns the screen scale
///
/// If `screen_num` doesn't correspond to a valid screen number,
/// the main screen's number (`0`) will be used instead.
pub fn screen_scale(screen_num: i32) -> f32 {
    unsafe { fl::Fl_screen_scale(screen_num) }
}

/// Returns whether scaling the screen is supported
pub fn screen_scaling_supported() -> bool {
    unsafe { fl::Fl_screen_scaling_supported() != 0 }
}

/// Returns the screen count
pub fn screen_count() -> i32 {
    unsafe { fl::Fl_screen_count() }
}

/// Returns the screen number of a screen that contains the specified position
///
/// If the coordinates are out of bounds, the main screen's number (`0`)
/// will be returned instead.
pub fn screen_num(x: i32, y: i32) -> i32 {
    unsafe { fl::Fl_screen_num(x, y) }
}

/// Returns the screen number for the screen which intersects the most with
/// the provided rectangle
pub fn screen_num_inside<R: Into<Rect> + Copy>(rect: R) -> i32 {
    let r: Rect = rect.into();
    unsafe { fl::Fl_screen_num_inside(r.x, r.y, r.w, r.h) }
}

/// Returns a screen's (vertical, horizontal) dpi resolution
///
/// If `screen_num` doesn't correspond to a valid screen number,
/// the main screen's number (`0`) will be used instead.
pub fn screen_dpi(screen_num: i32) -> (f32, f32) {
    let (mut h, mut v) = (0_f32, 0_f32);
    unsafe {
        fl::Fl_screen_dpi(&mut h, &mut v, screen_num);
    }
    (h, v)
}

/// Get a screen's xywh
///
/// If `screen_num` doesn't correspond to a valid screen number,
/// the main screen's number (`0`) will be used instead.
pub fn screen_xywh(screen_num: i32) -> (i32, i32, i32, i32) {
    let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
    unsafe {
        fl::Fl_screen_xywh(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
}

/// Get a screen's working area
///
/// If `screen_num` doesn't correspond to a valid screen number,
/// the main screen's number (`0`) will be used instead.
pub fn screen_work_area(screen_num: i32) -> (i32, i32, i32, i32) {
    let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
    unsafe {
        fl::Fl_screen_work_area(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
}

/// Controls the possibility to scale all windows by ctrl/+/-/0/ or cmd/+/-/0/
///
/// This function should be called before `app::open_display` runs.
/// If it is not called, the default is to handle these keys for window scaling.
///
/// Pass a `value` of `false` to stop recognition of ctrl/+/-/0/
/// (or cmd/+/-/0/ under macOS) keys as window scaling.
pub fn keyboard_screen_scaling(value: bool) {
    unsafe { fl::Fl_keyboard_screen_scaling(value as _) }
}
