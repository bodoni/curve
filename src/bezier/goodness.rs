//! Goodness of fit.

use num_traits::Float;

use crate::bezier::Cubic;
use crate::goodness::Goodness;

/// A goodness of fit based on the coordinate-wise absolute distance between control points.
pub struct CrudeIndependentAbsolute<T: Float> {
    proximity: T,
    fragmentation: usize,
    subdivisions: usize,
}

impl<T: Float> CrudeIndependentAbsolute<T> {
    /// Create an instance.
    ///
    /// `proximity` dictates the maximum absolute coordinate-wise distance between the control
    /// points of a candidate curve and an original one, and `fragmentation` dictates the maximum
    /// number of admission checks, typically corresponding to the number of subdivisions during an
    /// approximation process.
    #[inline]
    pub fn new(proximity: T, fragmentation: usize) -> Self {
        Self {
            proximity,
            fragmentation,
            subdivisions: 0,
        }
    }
}

impl<T: Float> Goodness<Cubic<T>> for CrudeIndependentAbsolute<T> {
    fn admit(&mut self, candidate: &Cubic<T>, original: &Cubic<T>) -> Option<bool> {
        if absolute(candidate.0, original.0, self.proximity) {
            return Some(true);
        }
        if self.subdivisions < self.fragmentation {
            self.subdivisions += 1;
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
        if absolute(candidate.0 .0, original.0 .0, self.proximity)
            && absolute(candidate.1 .0, original.1 .0, self.proximity)
        {
            return Some(true);
        }
        if self.subdivisions < self.fragmentation {
            self.subdivisions += 1;
            return Some(false);
        }
        None
    }
}

#[rustfmt::skip]
fn absolute<T: Float>(one: [T; 4], other: [T; 4], proximity: T) -> bool {
    (one[0] - other[0]).abs() < proximity &&
    (one[1] - other[1]).abs() < proximity &&
    (one[2] - other[2]).abs() < proximity &&
    (one[3] - other[3]).abs() < proximity
}
