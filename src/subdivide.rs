use num_traits::Float;

/// A curve that can be subdivided into two.
pub trait Subdivide<T: Float>: Sized {
    /// Perform the calculation.
    fn subdivide(&self, t: T) -> (Self, Self);
}

impl<T, U> Subdivide<T> for (U, U)
where
    T: Float,
    U: Subdivide<T>,
{
    #[inline]
    fn subdivide(&self, t: T) -> (Self, Self) {
        let one = self.0.subdivide(t);
        let other = self.1.subdivide(t);
        ((one.0, other.0), (one.1, other.1))
    }
}
