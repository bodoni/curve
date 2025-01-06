use num_traits::Float;

use crate::bezier::{Cubic, Linear, Quadratic};
use crate::Evaluate;

impl<T> Evaluate<T> for Linear<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        (T::one() - t) * self.a + t * self.b
    }
}

impl<T> Evaluate<T> for Quadratic<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let two = T::one() + T::one();
        u * u * self.a + two * u * t * self.b + t * t * self.c
    }
}

impl<T> Evaluate<T> for Cubic<T>
where
    T: Float,
{
    fn evaluate(&self, t: T) -> T {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let u2 = u * u;
        let t2 = t * t;
        let three = T::one() + T::one() + T::one();
        u2 * u * self.a + three * u2 * t * self.b + three * u * t2 * self.c + t2 * t * self.d
    }
}
