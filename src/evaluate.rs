use num_traits::Float;

/// A curve that can be evaluated at a point in `[0, 1]`.
pub trait Evaluate<T: Float> {
    /// Perform the calculation.
    fn evaluate(&self, t: T) -> T;
}
