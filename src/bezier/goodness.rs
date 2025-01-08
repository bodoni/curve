//! Goodness of fit.

use num_traits::Float;

use crate::bezier::Cubic;
use crate::goodness::Goodness;

/// A goodness of fit based on the coordinate-wise absolute distance between control points.
pub struct CrudeIndependentAbsolute<T: Float> {
    distance: T,
    subdivision: usize,
    index: usize,
}

impl<T: Float> CrudeIndependentAbsolute<T> {
    /// Create an instance.
    ///
    /// `distance` dictates the maximum absolute coordinate-wise distance between the control
    /// points of a candidate curve and an original one, and `subdivision` dictates the maximum
    /// number of subdivisions during an approximation process.
    #[inline]
    pub fn new(distance: T, subdivision: usize) -> Self {
        Self {
            distance,
            subdivision,
            index: 0,
        }
    }
}

impl<T: Float> Goodness<Cubic<T>> for CrudeIndependentAbsolute<T> {
    fn admit(&mut self, candidate: &Cubic<T>, original: &Cubic<T>) -> Option<bool> {
        if absolute(candidate.0, original.0, self.distance) {
            return Some(true);
        }
        if self.index < self.subdivision {
            self.index += 1;
            return Some(false);
        }
        None
    }
}

impl<T: Float + std::fmt::Debug> Goodness<(Cubic<T>, Cubic<T>)> for CrudeIndependentAbsolute<T> {
    fn admit(
        &mut self,
        candidate: &(Cubic<T>, Cubic<T>),
        original: &(Cubic<T>, Cubic<T>),
    ) -> Option<bool> {
        if absolute(candidate.0 .0, original.0 .0, self.distance)
            && absolute(candidate.1 .0, original.1 .0, self.distance)
        {
            return Some(true);
        }
        if self.index < self.subdivision {
            self.index += 1;
            return Some(false);
        }
        None
    }
}

#[rustfmt::skip]
fn absolute<T: Float>(one: [T; 4], other: [T; 4], distance: T) -> bool {
    (one[0] - other[0]).abs() <= distance &&
    (one[1] - other[1]).abs() <= distance &&
    (one[2] - other[2]).abs() <= distance &&
    (one[3] - other[3]).abs() <= distance
}
