use num_traits::Float;

/// A curve that can be subdivided into two at a point in `(0, 1)`.
pub trait Subdivide<T: Float>: Sized {
    /// Perform the calculation.
    fn subdivide(&self, t: T) -> (Self, Self);
}
