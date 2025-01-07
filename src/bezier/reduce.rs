use num_traits::Float;

use crate::bezier::{Cubic, Quadratic};
use crate::Reduce;

impl<T> Reduce<T> for Cubic<T>
where
    T: Float,
{
    type Target = Quadratic<T>;

    fn reduce(&self, _: T) -> Option<Self::Target> {
        None
    }
}
