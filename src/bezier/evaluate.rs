use crate::bezier::{Cubic, Linear, Quadratic};
use crate::Evaluate;

macro_rules! implement {
    ($($float:ty),*) => ($(
        impl Evaluate<$float> for Linear<$float> {
            fn evaluate(&self, t: $float) -> $float {
                debug_assert!((0.0..=1.0).contains(&t));
                (1.0 - t) * self.a + t * self.b
            }
        }

        impl Evaluate<$float> for Quadratic<$float> {
            fn evaluate(&self, t: $float) -> $float {
                debug_assert!((0.0..=1.0).contains(&t));
                let c = 1.0 - t;
                c * c * self.a + 2.0 * c * t * self.b + t * t * self.c
            }
        }

        impl Evaluate<$float> for Cubic<$float> {
            fn evaluate(&self, t: $float) -> $float {
                debug_assert!((0.0..=1.0).contains(&t));
                let c = 1.0 - t;
                let c2 = c * c;
                let t2 = t * t;
                c2 * c * self.a + 3.0 * c2 * t * self.b + 3.0 * c * t2 * self.c + t2 * t * self.d
            }
        }
    )*);
}

implement!(f32, f64);
