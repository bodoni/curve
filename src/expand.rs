use num_traits::Float;

/// A curve that can be expanded into a higher-order one.
pub trait Expand<T: Float> {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn expand(&self) -> Self::Target;
}

impl<T, U> Expand<T> for (U, U)
where
    T: Float,
    U: Expand<T>,
{
    type Target = (<U as Expand<T>>::Target, <U as Expand<T>>::Target);

    #[inline]
    fn expand(&self) -> Self::Target {
        (self.0.expand(), self.1.expand())
    }
}
