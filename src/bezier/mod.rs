//! Bézier curves.

pub mod goodness;

mod align;
mod evaluate;
mod expand;
mod reduce;
mod subdivide;
mod trace;

/// A linear curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Linear<T>([T; 2]);

/// A quadratic curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quadratic<T>([T; 3]);

/// A cubic curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cubic<T>([T; 4]);

impl<T> Linear<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T) -> Self {
        Self([a, b])
    }
}

impl<T> Quadratic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T) -> Self {
        Self([a, b, c])
    }
}

impl<T> Cubic<T> {
    /// Create an instance.
    #[inline]
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self([a, b, c, d])
    }
}

impl<T> std::ops::Deref for Linear<T> {
    type Target = [T; 2];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Linear<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> std::ops::Deref for Quadratic<T> {
    type Target = [T; 3];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Quadratic<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> std::ops::Deref for Cubic<T> {
    type Target = [T; 4];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Cubic<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
