use std::ops::{Add, Sub};

macro_rules! vec2 {
    ($t: ident, $i1: ident, $i2: ident) => {
        #[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        /// A generic pair type
        pub struct $t<T: Copy> {
            /// First member
            pub $i1: T,
            /// Second member
            pub $i2: T,
        }

        impl<T: Copy> $t<T> {
            /// Constructor
            pub fn new($i1: T, $i2: T) -> Self {
                $t { $i1, $i2 }
            }

            /// returns a tuple representation
            pub fn tup(&self) -> (T, T) {
                (self.$i1, self.$i2)
            }
        }

        impl<T: Copy> From<[T; 2]> for $t<T> {
            fn from(array: [T; 2]) -> Self {
                Self {
                    $i1: array[0],
                    $i2: array[1],
                }
            }
        }

        impl<T: Copy> From<$t<T>> for [T; 2] {
            fn from(c: $t<T>) -> Self {
                [c.$i1, c.$i2]
            }
        }

        impl<T: Copy> From<(T, T)> for $t<T> {
            fn from(tuple: (T, T)) -> Self {
                Self {
                    $i1: tuple.0,
                    $i2: tuple.1,
                }
            }
        }

        impl<T: Copy> From<$t<T>> for (T, T) {
            fn from(c: $t<T>) -> Self {
                (c.$i1, c.$i2)
            }
        }
    };
}

vec2!(Coordinates, x, y);

vec2!(GSize, w, h);

vec2!(GCell, r, c);

/// `i32` Coordinates holding an `x, y` members
pub type Coord = Coordinates<i32>;

/// `f64` Coordinates holding an `x, y` members
pub type Coordf = Coordinates<f64>;

/// `i32` Cell holding a `r, c` (row and column)
pub type Cell = GCell<i32>;

// /// `i32` Size holding a `w, h` (width and height)
// pub type Size = GSize<i32>;

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

    /// Returns a new `Rectangle` from the position of its Coords and Size
    pub fn from_coords_size(pos: Coordinates<T>, size: GSize<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.w,
            h: size.h,
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
