use num_traits::Float;

/// A curve that can be expanded into a higher-order one.
pub trait Expand<T: Float>: Sized {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn expand(&self) -> Self::Target;
}
