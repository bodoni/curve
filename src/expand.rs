use num_traits::Float;

/// A curve whose order can be raised.
pub trait Expand<T: Float>: Sized {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn expand(&self) -> Self::Target;
}
