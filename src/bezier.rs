//! Bézier curves.

use num::Float;

use {Curve, Trace};

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T: Float> {
    x0: T,
    y0: T,
    x1: T,
    y1: T,
    x2: T,
    y2: T,
    x3: T,
    y3: T,
}

impl<T: Float> Cubic<T> {
    /// Create a curve.
    #[inline]
    pub fn new(x0: T, y0: T, x1: T, y1: T, x2: T, y2: T, x3: T, y3: T) -> Cubic<T> {
        Cubic { x0: x0, y0: y0, x1: x1, y1: y1, x2: x2, y2: y2, x3: x3, y3: y3 }
    }
}

macro_rules! implement {
    ($float:ty) => (
        impl Cubic<$float> {
            /// Start tracing the curve.
            #[inline(always)]
            pub fn trace<'l>(&'l self, steps: usize) -> Trace<'l, $float, Self> {
                Trace::new(self, steps)
            }
        }

        impl Curve<$float> for Cubic<$float> {
            fn evaluate(&self, t: $float) -> ($float, $float) {
                debug_assert!(0.0 <= t && t <= 1.0);

                let t2 = t * t;
                let t3 = t2 * t;
                let ct = 1.0 - t;
                let ct2 = ct * ct;
                let ct3 = ct2 * ct;

                let x = ct3 * self.x0 +
                        3.0 * ct2 * t * self.x1 +
                        3.0 * ct * t2 * self.x2 +
                        t3 * self.x3;

                let y = ct3 * self.y0 +
                        3.0 * ct2 * t * self.y1 +
                        3.0 * ct * t2 * self.y2 +
                        t3 * self.y3;

                (x, y)
            }
        }
    );
}

implement!(f32);
implement!(f64);
