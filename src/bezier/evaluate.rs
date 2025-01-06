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
            let one = T::one();
            let u = one - t;
            (u * u, (one + one) * u * t, t * t)
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
            let one = T::one();
            let u = one - t;
            let u2 = u * u;
            let t2 = t * t;
            let three = one + one + one;
            (u2 * u, three * u2 * t, three * u * t2, t2 * t)
        };
        self.a * a + self.b * b + self.c * c + self.d * d
    }
}
