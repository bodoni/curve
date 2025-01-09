//! Goodness of fit.

use num_traits::Float;

use crate::bezier::Cubic;
use crate::goodness::Goodness;

/// A goodness of fit based on the coordinate-wise absolute distance between control points.
pub struct CrudeIndependentAbsolute<T: Float> {
    absolute_distance: T,
    relative_distance: T,
    subdivision: usize,
    index: usize,
}

impl<T: Float> CrudeIndependentAbsolute<T> {
    /// Create an instance.
    ///
    /// * `absolute_distance` dictates the maximum coordinate-wise distance between the control
    ///   points of a candidate curve and an original one.
    ///
    /// * `relative_distance` dictates the maximum coordinate-wise distance between the control
    ///   points of a candidate curve and an original one relative to the largest coordinate-wise
    ///   distance between the endpoints.
    ///
    /// * `subdivision` dictates the maximum number of subdivisions that can be made during an
    ///   approximation process.
    #[inline]
    pub fn new(absolute_distance: T, relative_distance: T, subdivision: usize) -> Self {
        Self {
            absolute_distance,
            relative_distance,
            subdivision,
            index: 0,
        }
    }
}

impl<T: Float> Goodness<Cubic<T>> for CrudeIndependentAbsolute<T> {
    fn admit(&mut self, candidate: &Cubic<T>, original: &Cubic<T>) -> Option<bool> {
        let size = (original.0[0] - original.0[3]).abs();
        let distance = self.absolute_distance.min(self.relative_distance * size);
        if admit(candidate.0, original.0, distance) {
            return Some(true);
        }
        if self.index < self.subdivision {
            self.index += 1;
            return Some(false);
        }
        None
    }
}

impl<T: Float> Goodness<(Cubic<T>, Cubic<T>)> for CrudeIndependentAbsolute<T> {
    fn admit(
        &mut self,
        candidate: &(Cubic<T>, Cubic<T>),
        original: &(Cubic<T>, Cubic<T>),
    ) -> Option<bool> {
        let size = {
            let width = (original.0[0] - original.0[3]).abs();
            let height = (original.1[0] - original.1[3]).abs();
            width.max(height)
        };
        let distance = self.absolute_distance.min(self.relative_distance * size);
        if admit(candidate.0 .0, original.0 .0, distance)
            && admit(candidate.1 .0, original.1 .0, distance)
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
fn admit<T: Float>(one: [T; 4], other: [T; 4], distance: T) -> bool {
    (one[0] - other[0]).abs() <= distance &&
    (one[1] - other[1]).abs() <= distance &&
    (one[2] - other[2]).abs() <= distance &&
    (one[3] - other[3]).abs() <= distance
}
