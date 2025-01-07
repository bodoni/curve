use num_traits::Float;

use crate::bezier::Cubic;
use crate::compare::Compare;

/// The default goodness of fit.
pub struct Comparison<T: Float> {
    tolerance: T,
}

impl<T: Float> Comparison<T> {
    /// Create an instance.
    #[inline]
    pub fn new(tolerance: T) -> Self {
        Self { tolerance }
    }
}

impl<T: Float> Compare<Cubic<T>> for Comparison<T> {
    fn compare(&self, one: &Cubic<T>, other: &Cubic<T>) -> bool {
        let beta = [
            one.0[0] - other.0[0],
            one.0[1] - other.0[1],
            one.0[2] - other.0[2],
            one.0[3] - other.0[3],
        ];
        let mut sum = T::zero();
        sum = sum + beta[0] * beta[0];
        sum = sum + beta[1] * beta[1];
        sum = sum + beta[2] * beta[2];
        sum = sum + beta[3] * beta[3];
        sum.sqrt() < self.tolerance
    }
}
