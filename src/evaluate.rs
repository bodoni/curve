use num_traits::Float;

/// A curve that can be evaluated.
pub trait Evaluate<T: Float> {
    /// The resulting type.
    type Target;

    /// Perform the calculation.
    fn evaluate(&self, t: T) -> Self::Target;
}

impl<T, U> Evaluate<T> for (U, U)
where
    T: Float,
    U: Evaluate<T, Target = T>,
{
    type Target = (T, T);

    #[inline]
    fn evaluate(&self, t: T) -> Self::Target {
        (self.0.evaluate(t), self.1.evaluate(t))
    }
}
