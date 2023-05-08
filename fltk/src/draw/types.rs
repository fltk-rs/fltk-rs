/// Defines a pair of `x, y` coordinates
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinates<T: Copy> {
    /// Horizontal X coordinate
    pub x: T,
    /// Vertical Y coordinate
    pub y: T,
}

/// `i32` Coordinates
#[derive(Debug, Copy, Clone)]
pub struct Coord<T: Copy>(pub T, pub T);
// pub type Coord = Coordinates<i32>; // TODO for 2.0

/// `f64` Coordinates
pub type Coordf = Coordinates<f64>;

impl<T: Copy> Coordinates<T> {
    /// Returns a new pair of `x, y` coordinates
    pub fn new(x: T, y: T) -> Self {
        Coordinates { x, y }
    }

    /// Returns a tuple of the values
    pub fn tup(&self) -> (T, T) {
        (self.x, self.y)
    }
}

// Conversions From/Into array and tuple

impl<T: Copy> From<[T; 2]> for Coordinates<T> {
    fn from(array: [T; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

impl<T: Copy> From<Coordinates<T>> for [T; 2] {
    fn from(c: Coordinates<T>) -> Self {
        [c.x, c.y]
    }
}

impl<T: Copy> From<(T, T)> for Coordinates<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T: Copy> From<Coordinates<T>> for (T, T) {
    fn from(c: Coordinates<T>) -> Self {
        (c.x, c.y)
    }
}

/// Defines a pair of `w, h` (width, height) values representing size
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Size {
    /// Width
    pub w: i32,
    /// Height
    pub h: i32,
}

impl Size {
    /// Returns a new pair of `w, h` (width, height) values
    pub fn new(w: i32, h: i32) -> Self {
        Size { w, h }
    }

    /// Returns a tuple of the values
    pub fn tup(&self) -> (i32, i32) {
        (self.w, self.h)
    }
}

// Conversions From/Into array and tuple

impl From<[i32; 2]> for Size {
    fn from(array: [i32; 2]) -> Self {
        Self {
            w: array[0],
            h: array[1],
        }
    }
}

impl From<Size> for [i32; 2] {
    fn from(c: Size) -> Self {
        [c.w, c.h]
    }
}

impl From<(i32, i32)> for Size {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            w: tuple.0,
            h: tuple.1,
        }
    }
}
impl From<Size> for (i32, i32) {
    fn from(c: Size) -> Self {
        (c.w, c.h)
    }
}

/// Defines a pair of `r, c` (row, column) representing a Cell
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cell {
    /// Horizontal X coordinate
    pub r: i32,
    /// Vertical Y coordinate
    pub c: i32,
}

impl Cell {
    /// Returns a new pair of `r, c` (row, column) cell
    pub fn new(r: i32, c: i32) -> Self {
        Cell { r, c }
    }

    /// Returns a tuple of the values
    pub fn tup(&self) -> (i32, i32) {
        (self.r, self.c)
    }
}

// Conversions From/Into array and tuple

impl From<[i32; 2]> for Cell {
    fn from(array: [i32; 2]) -> Self {
        Cell {
            r: array[0],
            c: array[1],
        }
    }
}

impl From<Cell> for [i32; 2] {
    fn from(c: Cell) -> Self {
        [c.r, c.c]
    }
}

impl From<(i32, i32)> for Cell {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            r: tuple.0,
            c: tuple.1,
        }
    }
}
impl From<Cell> for (i32, i32) {
    fn from(c: Cell) -> Self {
        (c.r, c.c)
    }
}

use std::ops::{Add, Sub};

/// Defines a rectangular bounding box
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

/// `i32` Coordinates
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

    /// Sets the Rectangle to be padded
    pub fn with_padding(self, pad: T) -> Self {
        Self {
            x: self.x + pad,
            y: self.y + pad,
            w: self.w - (pad + pad),
            h: self.h - (pad + pad),
        }
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

    /// Returns a tuple of the values
    pub fn tup(&self) -> (T, T, T, T) {
        (self.x, self.y, self.w, self.h)
    }
}

// Conversions From/Into array and tuple

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
