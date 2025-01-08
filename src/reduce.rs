use num_traits::Float;

/// A curve that can be reduced into a lower-level one.
pub trait Reduce<T: Float> {
    /// The resulting curve.
    type Target;

    /// Perform the calculation.
    fn reduce(&self) -> Self::Target;
}

impl<T, U> Reduce<T> for (U, U)
where
    T: Float,
    U: Reduce<T>,
{
    type Target = (<U as Reduce<T>>::Target, <U as Reduce<T>>::Target);

    #[inline]
    fn reduce(&self) -> Self::Target {
        (self.0.reduce(), self.1.reduce())
    }
}
