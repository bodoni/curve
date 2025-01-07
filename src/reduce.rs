use num_traits::Float;

/// A curve whose order can be lowered.
pub trait Reduce<T: Float>: Sized {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn reduce(&self) -> Self::Target;
}
