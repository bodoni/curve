//! Curves.

use num_traits::Float;

/// A curve that can be evaluated.
pub trait Evaluate<T: Float> {
    /// Evaluate the curve at a point in `[0, 1]`.
    fn evaluate(&self, t: T) -> T;
}

/// A curve that can be subdivided.
pub trait Subdivide<T: Float>: Sized {
    /// Subdivide the curve into several.
    fn subdivide(&self, n: usize) -> impl Iterator<Item = Self>;
}

pub mod bezier;

mod trace;

pub use trace::Trace;
