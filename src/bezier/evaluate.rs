use num_traits::Float;

use crate::bezier::{Cubic, Linear, Quadratic};
use crate::Evaluate;

impl<T> Evaluate<T> for Linear<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        let (a, b) = (T::one() - t, t);
        self.a * a + self.b * b
    }
}

impl<T> Evaluate<T> for Quadratic<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        let (a, b, c) = {
            let _1 = T::one();
            let u = _1 - t;
            (u * u, (_1 + _1) * u * t, t * t)
        };
        self.a * a + self.b * b + self.c * c
    }
}

impl<T> Evaluate<T> for Cubic<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        let (a, b, c, d) = {
            let _1 = T::one();
            let u = _1 - t;
            let u2 = u * u;
            let t2 = t * t;
            let _3 = _1 + _1 + _1;
            (u2 * u, _3 * u2 * t, _3 * u * t2, t2 * t)
        };
        self.a * a + self.b * b + self.c * c + self.d * d
    }
}
