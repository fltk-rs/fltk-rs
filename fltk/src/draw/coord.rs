
/// Defines a pair of `x, y` coordinates
#[derive(Copy, Clone, Debug)]
pub struct Coordinates<T: Copy> {
    /// Horizontal X coordinate
    pub x: T,
    /// Vertical Y coordinate
    pub y: T,
}

/// `i32` Coordinates
pub type Coord = Coordinates<i32>;

/// `f64` Coordinates
#[allow(non_camel_case_types)]
pub type Coord_f64 = Coordinates<f64>;

impl<T: Copy> Coordinates<T> {
    /// Returns a new pair of `x, y` coordinates
    pub fn new(x: T, y: T) -> Self {
        Coordinates { x, y }
    }

    /// Returns `x` interpreted as width.
    #[inline]
    pub fn w(&self) -> T {
        self.x
    }
    /// Returns `y` interpreted as height.
    #[inline]
    pub fn h(&self) -> T {
        self.y
    }
}

// Conversions From/Into array and tuple

impl<T: Copy> From<[T; 2]> for Coordinates<T> {
    fn from(array: [T; 2]) -> Self {
        Self { x: array[0], y: array[1] }
    }
}
impl<T: Copy> From<Coordinates<T>> for [T; 2] {
    fn from(c: Coordinates<T>) -> Self {
        [c.x, c.y]
    }
}

impl<T: Copy> From<(T, T)> for Coordinates<T> {
    fn from(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }
}
impl<T: Copy> From<Coordinates<T>> for (T, T) {
    fn from(c: Coordinates<T>) -> Self {
        (c.x, c.y)
    }
}
