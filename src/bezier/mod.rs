//! Bézier curves.

mod evaluate;
mod subdivide;
mod trace;

use num_traits::Float;

/// A linear Bézier curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Linear<T: Float>([T; 2]);

/// A quadratic Bézier curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quadratic<T: Float>([T; 3]);

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cubic<T: Float>([T; 4]);

impl<T: Float> Linear<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T) -> Self {
        Self([a, b])
    }
}

impl<T: Float> Quadratic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T) -> Self {
        Self([a, b, c])
    }
}

impl<T: Float> Cubic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self([a, b, c, d])
    }
}
