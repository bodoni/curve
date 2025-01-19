//! Curves.
//!
//! # Examples
//!
//! Trace a Bézier curve:
//!
//! ```
//! use curve::bezier::Linear;
//! use curve::Trace;
//!
//! let curve = (Linear::new(1.0, 5.0), Linear::new(2.0, 3.0));
//! let points = Trace::new(curve, 3).collect::<Vec<_>>();
//! assert_eq!(points, vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)]);
//! ```
//!
//! Approximate a cubic Bézier curve with a sequence of quadratic:
//!
//! ```
//! use curve::bezier::goodness::CrudeIndependentAbsolute;
//! use curve::bezier::Cubic;
//! use curve::Approximation;
//!
//! let goodness = CrudeIndependentAbsolute::new(1.0, f64::MAX, usize::MAX);
//! let cubic = (Cubic::new(0.0, 0.0, 90.0, 100.0), Cubic::new(0.0, 50.0, 0.0, 0.0));
//! let quadratics = Approximation::new(cubic, goodness).collect::<Vec<_>>();
//! assert_eq!(quadratics.len(), 4);
//! ```

pub mod bezier;

mod align;
mod approximation;
mod evaluate;
mod expand;
mod goodness;
mod reduce;
mod subdivide;
mod trace;

pub use align::Align;
pub use approximation::Approximation;
pub use evaluate::Evaluate;
pub use expand::Expand;
pub use goodness::Goodness;
pub use reduce::Reduce;
pub use subdivide::Subdivide;
pub use trace::Trace;
