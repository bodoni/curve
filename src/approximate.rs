use num_traits::Float;

/// A curve that can be approximated with a sequece of lower ones.
pub trait Approximate<T: Float>: Sized {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn approximate(&self, tolerance: T) -> impl Iterator<Item = Self::Target>;
}
