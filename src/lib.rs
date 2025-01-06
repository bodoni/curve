//! Curves.
//!
//! # Example
//!
//! ```
//! let x = curve::bezier::Linear::new(1.0, 5.0);
//! let y = curve::bezier::Linear::new(2.0, 3.0);
//! let points = x.trace(3).zip(y.trace(3)).collect::<Vec<_>>();
//! assert_eq!(points, vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)]);
//! ```

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
