use num_traits::Float;

/// A curve that can be evaluated.
pub trait Evaluate<T: Float> {
    /// Perform the calculation.
    fn evaluate(&self, t: T) -> T;
}
