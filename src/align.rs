use num_traits::Float;

/// A curve that can be aligned with another one.
pub trait Align<T: Float, U> {
    /// Perform the calculation.
    fn align(self, other: &U) -> Self;
}

impl<T, U, V> Align<T, (U, U)> for (V, V)
where
    T: Float,
    V: Align<T, U>,
{
    #[inline]
    fn align(self, other: &(U, U)) -> Self {
        (self.0.align(&other.0), self.1.align(&other.1))
    }
}
