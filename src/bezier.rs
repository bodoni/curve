//! Bézier curves.

use num::Float;

use {Curve, Point, Trace};

/// A linear Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Linear<T: Float> {
    a: Point<T>,
    b: Point<T>,
}

/// A quadratic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Quadratic<T: Float> {
    a: Point<T>,
    b: Point<T>,
    c: Point<T>,
}

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T: Float> {
    a: Point<T>,
    b: Point<T>,
    c: Point<T>,
    d: Point<T>,
}

impl<T: Float> Linear<T> where Linear<T>: Curve<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: Point<T>, b: Point<T>) -> Self {
        Linear { a: a, b: b }
    }

    /// Start tracing the curve.
    #[inline]
    pub fn trace<'l>(&'l self, steps: usize) -> Trace<'l, T, Self> {
        Trace::new(self, steps)
    }
}

impl<T: Float> Quadratic<T> where Quadratic<T>: Curve<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>) -> Self {
        Quadratic { a: a, b: b, c: c }
    }

    /// Start tracing the curve.
    #[inline]
    pub fn trace<'l>(&'l self, steps: usize) -> Trace<'l, T, Self> {
        Trace::new(self, steps)
    }
}

impl<T: Float> Cubic<T> where Cubic<T>: Curve<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: Point<T>, b: Point<T>, c: Point<T>, d: Point<T>) -> Self {
        Cubic { a: a, b: b, c: c, d: d }
    }

    /// Start tracing the curve.
    #[inline]
    pub fn trace<'l>(&'l self, steps: usize) -> Trace<'l, T, Self> {
        Trace::new(self, steps)
    }
}

macro_rules! implement {
    ($($float:ty),*) => ($(
        impl Curve<$float> for Linear<$float> {
            fn evaluate(&self, t1: $float) -> Point<$float> {
                debug_assert!(0.0 <= t1 && t1 <= 1.0);
                let &Linear { a, b } = self;
                let c1 = 1.0 - t1;
                let x = c1 * a.0 + t1 * b.0;
                let y = c1 * a.1 + t1 * b.1;
                (x, y)
            }
        }

        impl Curve<$float> for Quadratic<$float> {
            fn evaluate(&self, t1: $float) -> Point<$float> {
                debug_assert!(0.0 <= t1 && t1 <= 1.0);
                let &Quadratic { a, b, c } = self;
                let t2 = t1 * t1;
                let c1 = 1.0 - t1;
                let c2 = c1 * c1;
                let x = c2 * a.0 + 2.0 * c1 * t1 * b.0 + t2 * c.0;
                let y = c2 * a.1 + 2.0 * c1 * t1 * b.1 + t2 * c.1;
                (x, y)
            }
        }

        impl Curve<$float> for Cubic<$float> {
            fn evaluate(&self, t1: $float) -> Point<$float> {
                debug_assert!(0.0 <= t1 && t1 <= 1.0);
                let &Cubic { a, b, c, d } = self;
                let t2 = t1 * t1;
                let t3 = t2 * t1;
                let c1 = 1.0 - t1;
                let c2 = c1 * c1;
                let c3 = c2 * c1;
                let x = c3 * a.0 + 3.0 * c2 * t1 * b.0 + 3.0 * c1 * t2 * c.0 + t3 * d.0;
                let y = c3 * a.1 + 3.0 * c2 * t1 * b.1 + 3.0 * c1 * t2 * c.1 + t3 * d.1;
                (x, y)
            }
        }
    )*);
}

implement!(f32, f64);

#[cfg(test)]
mod tests {
    use assert;
    use super::{Cubic, Linear, Quadratic};

    #[test]
    fn linear() {
        let curve = Linear::new((1.0, 2.0), (5.0, 3.0));
        let trace = vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)];
        assert_eq!(trace, curve.trace(3).collect::<Vec<_>>());
    }

    #[test]
    fn quadratic() {
        let curve = Quadratic::new((1.0, 2.0), (3.0, 1.0), (5.0, 3.0));
        let trace = vec![
            (1.0000000000000000e+00, 2.0000000000000000e+00),
            (1.4444444444444444e+00, 1.8148148148148147e+00),
            (1.8888888888888888e+00, 1.7037037037037037e+00),
            (2.3333333333333339e+00, 1.6666666666666667e+00),
            (2.7777777777777777e+00, 1.7037037037037037e+00),
            (3.2222222222222223e+00, 1.8148148148148149e+00),
            (3.6666666666666670e+00, 2.0000000000000000e+00),
            (4.1111111111111107e+00, 2.2592592592592595e+00),
            (4.5555555555555554e+00, 2.5925925925925926e+00),
            (5.0000000000000000e+00, 3.0000000000000000e+00),
        ];
        for (i, (x, y)) in curve.trace(10).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }

    #[test]
    fn cubic() {
        let curve = Cubic::new((1.0, 2.0), (3.0, 1.0), (5.0, 3.0), (6.0, 2.0));
        let trace = vec![
            (1.0000000000000000e+00, 2.0000000000000000e+00),
            (1.6652949245541835e+00, 1.7695473251028802e+00),
            (2.3223593964334710e+00, 1.7119341563786008e+00),
            (2.9629629629629632e+00, 1.7777777777777777e+00),
            (3.5788751714677640e+00, 1.9176954732510290e+00),
            (4.1618655692729769e+00, 2.0823045267489713e+00),
            (4.7037037037037033e+00, 2.2222222222222219e+00),
            (5.1961591220850476e+00, 2.2880658436213990e+00),
            (5.6310013717421121e+00, 2.2304526748971192e+00),
            (6.0000000000000000e+00, 2.0000000000000000e+00),
        ];
        for (i, (x, y)) in curve.trace(10).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }
}
