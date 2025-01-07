use num_traits::Float;

/// A curve that can be reduced into a lower-level one.
pub trait Reduce<T: Float> {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn reduce(&self) -> Self::Target;
}
