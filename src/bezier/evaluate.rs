// Reference:
// https://pomax.github.io/bezierinfo/#decasteljau

use num_traits::Float;

use crate::bezier::{Cubic, Linear, Quadratic};
use crate::evaluate::Evaluate;

impl<T> Evaluate<T> for Linear<T>
where
    T: Float,
{
    type Target = T;

    fn evaluate(&self, t: T) -> Self::Target {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let beta = self.0;
        beta[0] * u + beta[1] * t
    }
}

impl<T> Evaluate<T> for Quadratic<T>
where
    T: Float,
{
    type Target = T;

    fn evaluate(&self, t: T) -> Self::Target {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let mut beta = self.0;
        beta[0] = beta[0] * u + beta[1] * t;
        beta[1] = beta[1] * u + beta[2] * t;
        beta[0] * u + beta[1] * t
    }
}

impl<T> Evaluate<T> for Cubic<T>
where
    T: Float,
{
    type Target = T;

    fn evaluate(&self, t: T) -> Self::Target {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let mut beta = self.0;
        beta[0] = beta[0] * u + beta[1] * t;
        beta[1] = beta[1] * u + beta[2] * t;
        beta[2] = beta[2] * u + beta[3] * t;
        beta[0] = beta[0] * u + beta[1] * t;
        beta[1] = beta[1] * u + beta[2] * t;
        beta[0] * u + beta[1] * t
    }
}
