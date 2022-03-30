use std::ops::{Add, Sub};

/// Defines a pair of two comparable values
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pair<T: Copy>(pub T, pub T);

/// Coordinates
#[derive(Debug, Copy, Clone)]
pub struct Coord<T: Copy>(pub T, pub T); // TODO deprecate for 2.0

/// A pair of `(row, col)` coordinates representing a cell
pub type Cell = Pair<i32>;

impl Cell {
    /// New pair of `(row, col)` coordinates
    pub fn new_cell(row: i32, col: i32) -> Self {
        Self(col, row)
    }
    /// The column
    pub fn col(&self) -> i32 {
        self.0
    }
    /// The row
    pub fn row(&self) -> i32 {
        self.1
    }
    /// Set the column
    pub fn set_col(&mut self, col: i32) {
        self.0 = col
    }
    /// Set the row
    pub fn set_row(&mut self, row: i32) {
        self.1 = row
    }
}

/// A pair of `(x, y)` coordinates representing a pixel
pub type Xy = Pair<i32>;

// TODO: rename to Coord for 2.0
impl Xy {
    /// New pair of `(x, y)` coordinates
    pub fn new_xy(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    /// The `x` coordinate
    pub fn x(&self) -> i32 {
        self.0
    }
    /// The `y` coordinate
    pub fn y(&self) -> i32 {
        self.1
    }
    /// Set `x`
    pub fn set_x(&mut self, x: i32) {
        self.0 = x
    }
    /// Set `y`
    pub fn set_y(&mut self, y: i32) {
        self.1 = y
    }
}

/// A pair of (width, heigth) sizes
pub type Size = Pair<i32>;

impl Size {
    /// New pair of `(width, height)` coordinates
    pub fn new_size(width: i32, height: i32) -> Self {
        Self(width, height)
    }
    /// The width
    pub const fn w(&self) -> i32 {
        self.0
    }
    /// The height
    pub const fn h(&self) -> i32 {
        self.1
    }
    /// The width
    pub const fn width(&self) -> i32 {
        self.0
    }
    /// The height
    pub const fn height(&self) -> i32 {
        self.1
    }
    /// Set the width
    pub fn set_w(&mut self, width: i32) {
        self.0 = width
    }
    /// Set the height
    pub fn set_h(&mut self, height: i32) {
        self.1 = height
    }
    /// Set the width
    pub fn set_width(&mut self, width: i32) {
        self.0 = width
    }
    /// Set the height
    pub fn set_height(&mut self, height: i32) {
        self.1 = height
    }
}

// /// Xy `f64` Coordinates
// pub type XyF64 = Pair<f64>;

impl<T: Copy> Pair<T> {
    /// New coordinate pair
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Returns a tuple of the values
    pub fn tup(&self) -> (T, T) {
        (self.0, self.1)
    }
}

// impl From/Into array and tuple for Pair

impl<T: Copy> From<[T; 2]> for Pair<T> {
    fn from(array: [T; 2]) -> Self {
        Self(array[0], array[1])
    }
}

impl<T: Copy> From<Pair<T>> for [T; 2] {
    fn from(c: Pair<T>) -> Self {
        [c.0, c.1]
    }
}

impl<T: Copy> From<(T, T)> for Pair<T> {
    fn from(tuple: (T, T)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl<T: Copy> From<Pair<T>> for (T, T) {
    fn from(c: Pair<T>) -> Self {
        (c.0, c.1)
    }
}

/// Defines a rectangular bounding box
//
// MAYBE generalize like Pair, rename to Quadruple tuple
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

/// `i32` Rectangle
pub type Rect = Rectangle<i32>;

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Rectangle<T> {
    /// Returns a new `Rectangle` from its top-left corner position,
    /// and the length of its sides.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            w: width,
            h: height,
        }
    }

    /// Returns a new `Rectangle` from the position of its `top_left`
    /// and `bottom_right` corners.
    pub fn from_pairs(top_left: Pair<T>, bottom_right: Pair<T>) -> Self {
        Self {
            x: top_left.0,
            y: top_left.1,
            w: bottom_right.0 - top_left.0,
            h: bottom_right.1 - top_left.1,
        }
    }

    /// Returns the coordinates of the top-left corner
    pub fn top_left(&self) -> Pair<T> {
        Pair::new(self.x, self.y)
    }

    /// Returns the coordinates of the bottom-right corner
    pub fn bottom_right(&self) -> Pair<T> {
        Pair::new(self.x + self.w, self.y + self.h)
    }

    /// Returns a tuple of the values
    pub fn tup(&self) -> (T, T, T, T) {
        (self.x, self.y, self.w, self.h)
    }
}

// impl From/Into array and tuple for Rectangle

impl<T: Copy + Add<Output = T> + Sub<Output = T>> From<[T; 4]> for Rectangle<T> {
    fn from(array: [T; 4]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            w: array[2],
            h: array[3],
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> From<Rectangle<T>> for [T; 4] {
    fn from(c: Rectangle<T>) -> Self {
        [c.x, c.y, c.w, c.h]
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> From<(T, T, T, T)> for Rectangle<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            w: tuple.2,
            h: tuple.3,
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> From<Rectangle<T>> for (T, T, T, T) {
    fn from(c: Rectangle<T>) -> Self {
        (c.x, c.y, c.w, c.h)
    }
}
