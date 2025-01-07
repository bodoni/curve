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

pub mod bezier;

mod approximation;
mod compare;
mod evaluate;
mod expand;
mod reduce;
mod subdivide;
mod trace;

pub use approximation::Approximation;
pub use compare::Compare;
pub use evaluate::Evaluate;
pub use expand::Expand;
pub use reduce::Reduce;
pub use subdivide::Subdivide;
pub use trace::Trace;
