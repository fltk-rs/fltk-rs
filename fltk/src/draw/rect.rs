use std::ops::{Add, Sub};

use super::{Coordinates};

/// Defines a rectangular bounding box
#[derive(Copy, Clone, Debug)]
pub struct Rectangle<T: Copy + Add<Output = T> + Sub<Output = T>> {
    /// Leftmost corner
    pub x: T,
    /// Topmost corner
    pub y: T,
    /// Width
    pub w: T,
    /// Height
    pub h: T,
}

/// `i32` Coordinates
pub type Rect = Rectangle<i32>;

/// `f64` Coordinates
#[allow(non_camel_case_types)]
pub type Rect_f64 = Rectangle<f64>;

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Rectangle<T> {
    /// Returns a new `Rectangle` from its top-left corner position,
    /// and the length of its sides.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, w: width, h: height, }
    }

    /// Returns a new `Rectangle` from the position of its `top_left`
    /// and `bottom_right` corners.
    pub fn from_coords(top_left: Coordinates<T>, bottom_right: Coordinates<T>) -> Self {
        Self {
            x: top_left.x,
            y: top_left.y,
            w: bottom_right.x - top_left.x,
            h: bottom_right.y - top_left.y,
        }
    }

    /// Returns the coordinates of the top-left corner
    pub fn top_left(&self) -> Coordinates<T> {
        Coordinates::new(self.x, self.y)
    }

    /// Returns the coordinates of the bottom-right corner
    pub fn bottom_right(&self) -> Coordinates<T> {
        Coordinates::new(self.x + self.w, self.y + self.h)
    }
}

// Conversions From/Into array and tuple

impl<T: Copy + Add<Output=T> + Sub<Output=T>> From<[T; 4]> for Rectangle<T> {
    fn from(array: [T; 4]) -> Self {
        Self { x: array[0], y: array[1], w: array[2], h: array[3] }
    }
}
impl<T: Copy + Add<Output=T> + Sub<Output=T>> From<Rectangle<T>> for [T; 4] {
    fn from(c: Rectangle<T>) -> Self {
        [c.x, c.y, c.w, c.h]
    }
}

impl<T: Copy + Add<Output=T> + Sub<Output=T>> From<(T, T, T, T)> for Rectangle<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1, w: tuple.2, h: tuple.3 }
    }
}
impl<T: Copy + Add<Output=T> + Sub<Output=T>> From<Rectangle<T>> for (T, T, T, T) {
    fn from(c: Rectangle<T>) -> Self {
        (c.x, c.y, c.w, c.h)
    }
}
