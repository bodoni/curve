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
        (self.0.subdivide(t), self.1.subdivide(t))
    }
}
