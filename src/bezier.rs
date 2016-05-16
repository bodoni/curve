//! Bézier curves.

use num::Float;

use {Curve, Point, Trace};

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T: Float> {
    a: Point<T>,
    b: Point<T>,
    c: Point<T>,
    d: Point<T>,
}

impl<T: Float> Cubic<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>, d: Point<T>) -> Cubic<T> {
        Cubic { a: a, b: b, c: c, d: d }
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
            fn evaluate(&self, t: $float) -> Point<$float> {
                debug_assert!(0.0 <= t && t <= 1.0);

                let t2 = t * t;
                let t3 = t2 * t;
                let ct = 1.0 - t;
                let ct2 = ct * ct;
                let ct3 = ct2 * ct;

                let x =       ct3 *      self.a.0 +
                        3.0 * ct2 * t  * self.b.0 +
                        3.0 * ct  * t2 * self.c.0 +
                                    t3 * self.d.0;

                let y =       ct3 *      self.a.1 +
                        3.0 * ct2 * t  * self.b.1 +
                        3.0 * ct  * t2 * self.c.1 +
                                    t3 * self.d.1;

                (x, y)
            }
        }
    );
}

implement!(f32);
implement!(f64);
