//! Goodness of fit.

use num_traits::Float;

use crate::bezier::Cubic;
use crate::goodness::Goodness;

/// A goodness of fit based on the coordinate-wise absolute distance between control points.
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

impl<T: Float> Goodness<Cubic<T>> for IndependentCrudeAbsolute<T> {
    fn admit(&self, candidate: &Cubic<T>, original: &Cubic<T>) -> bool {
        absolute(candidate.0, original.0, self.tolerance)
    }
}

impl<T: Float> Goodness<(Cubic<T>, Cubic<T>)> for IndependentCrudeAbsolute<T> {
    fn admit(&self, candidate: &(Cubic<T>, Cubic<T>), original: &(Cubic<T>, Cubic<T>)) -> bool {
        absolute(candidate.0 .0, original.0 .0, self.tolerance)
            && absolute(candidate.1 .0, original.1 .0, self.tolerance)
    }
}

#[rustfmt::skip]
fn absolute<T: Float>(one: [T; 4], other: [T; 4], tolerance: T) -> bool {
    (one[0] - other[0]).abs() < tolerance &&
    (one[1] - other[1]).abs() < tolerance &&
    (one[2] - other[2]).abs() < tolerance &&
    (one[3] - other[3]).abs() < tolerance
}
