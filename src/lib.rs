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

/// A curve that can be evaluated at a point in `[0, 1]`.
pub trait Evaluate<T: Float> {
    /// Perform the calculation.
    fn evaluate(&self, t: T) -> T;
}

/// A curve that can be reduced to a lower-order one.
pub trait Reduce<T: Float>: Sized {
    /// The reduced curve.
    type Target;

    /// Perform the calculation.
    fn reduce(&self, tolerance: T) -> Option<Self::Target>;
}

/// A curve that can be subdivided into two at a point in `(0, 1)`.
pub trait Subdivide<T: Float>: Sized {
    /// Perform the calculation.
    fn subdivide(&self, t: T) -> (Self, Self);
}

pub mod bezier;

mod trace;

pub use trace::Trace;
