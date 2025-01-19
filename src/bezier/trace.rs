use num_traits::Float;

use crate::bezier::{Cubic, Linear, Quadratic};
use crate::evaluate::Evaluate;
use crate::trace::Trace;

macro_rules! implement {
    ($($type:ident),*) => ($(
        impl<T: Float> $type<T> where $type<T>: Evaluate<T> {
            /// Start tracing the curve.
            #[inline]
            pub fn trace(&self, steps: usize) -> Trace<'_, T, Self> {
                Trace::new(self, steps)
            }
        }
    )*);
}

implement!(Linear, Quadratic, Cubic);

