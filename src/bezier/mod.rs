//! Bézier curves.

mod evaluate;
mod subdivide;
mod trace;

use num_traits::Float;

/// A linear Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Linear<T: Float> {
    a: T,
    b: T,
}

/// A quadratic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Quadratic<T: Float> {
    a: T,
    b: T,
    c: T,
}

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T: Float> {
    a: T,
    b: T,
    c: T,
    d: T,
}

impl<T: Float> Linear<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Float> Quadratic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T: Float> Cubic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}
