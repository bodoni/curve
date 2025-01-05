//! Bézier curves.

use num_traits::Float;

use crate::{Curve, Trace};

/// A linear Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Linear<T: Float> {
    a: T,
    b: T,
}

/// A quadratic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Quadratic<T: Float> {
    a: T,
    b: T,
    c: T,
}

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T: Float> {
    a: T,
    b: T,
    c: T,
    d: T,
}

impl<T: Float> Linear<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Float> Quadratic<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T: Float> Cubic<T> {
    /// Create a curve.
    #[inline]
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self { a, b, c, d }
    }
}

macro_rules! implement {
    ($($curve:ident),*) => ($(
        impl<T: Float> $curve<T> where $curve<T>: Curve<T> {
            /// Start tracing the curve.
            #[inline]
            pub fn trace(&self, steps: usize) -> Trace<'_, T, Self> {
                Trace::new(self, steps)
            }
        }
    )*);
}

implement!(Linear, Quadratic, Cubic);

macro_rules! implement {
    ($($float:ty),*) => ($(
        impl Curve<$float> for Linear<$float> {
            #[inline]
            fn evaluate(&self, t: $float) -> $float {
                debug_assert!((0.0..=1.0).contains(&t));
                (1.0 - t) * self.a + t * self.b
            }
        }

        impl Curve<$float> for Quadratic<$float> {
            #[inline]
            fn evaluate(&self, t: $float) -> $float {
                debug_assert!((0.0..=1.0).contains(&t));
                let c = 1.0 - t;
                c * c * self.a + 2.0 * c * t * self.b + t * t * self.c
            }
        }

        impl Curve<$float> for Cubic<$float> {
            #[inline]
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

#[cfg(test)]
mod tests {
    use super::{Cubic, Linear, Quadratic};
    use assert;

    #[test]
    fn linear() {
        let curve1 = Linear::new(1.0, 5.0);
        let curve2 = Linear::new(2.0, 3.0);
        let trace = vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)];
        assert_eq!(
            trace,
            curve1.trace(3).zip(curve2.trace(3)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn quadratic() {
        let curve1 = Quadratic::new(1.0, 3.0, 5.0);
        let curve2 = Quadratic::new(2.0, 1.0, 3.0);
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
        for (i, (x, y)) in curve1.trace(10).zip(curve2.trace(10)).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }

    #[test]
    fn cubic() {
        let curve1 = Cubic::new(1.0, 3.0, 5.0, 6.0);
        let curve2 = Cubic::new(2.0, 1.0, 3.0, 2.0);
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
        for (i, (x, y)) in curve1.trace(10).zip(curve2.trace(10)).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }
}
