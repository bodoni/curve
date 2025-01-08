//! Comparison of curves.

use num_traits::Float;

use crate::bezier::Cubic;
use crate::compare::Compare;

/// A comparison based on the coordinate-wise absolute distance between control points.
pub struct IndependentCrudeAbsolute<T: Float> {
    tolerance: T,
}

impl<T: Float> IndependentCrudeAbsolute<T> {
    /// Create an instance.
    #[inline]
    pub fn new(tolerance: T) -> Self {
        Self { tolerance }
    }
}

impl<T: Float> Compare<Cubic<T>> for IndependentCrudeAbsolute<T> {
    fn compare(&self, one: &Cubic<T>, other: &Cubic<T>) -> bool {
        absolute(one.0, other.0, self.tolerance)
    }
}

impl<T: Float> Compare<(Cubic<T>, Cubic<T>)> for IndependentCrudeAbsolute<T> {
    fn compare(&self, one: &(Cubic<T>, Cubic<T>), other: &(Cubic<T>, Cubic<T>)) -> bool {
        absolute(one.0 .0, other.0 .0, self.tolerance)
            && absolute(one.1 .0, other.1 .0, self.tolerance)
    }
}

#[rustfmt::skip]
fn absolute<T: Float>(one: [T; 4], other: [T; 4], tolerance: T) -> bool {
    (one[0] - other[0]).abs() < tolerance &&
    (one[1] - other[1]).abs() < tolerance &&
    (one[2] - other[2]).abs() < tolerance &&
    (one[3] - other[3]).abs() < tolerance
}
