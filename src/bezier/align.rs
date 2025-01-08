use num_traits::Float;

use crate::align::Align;
use crate::bezier::{Cubic, Quadratic};

impl<T: Float> Align<T, Cubic<T>> for Quadratic<T> {
    fn align(mut self, other: &Cubic<T>) -> Self {
        self[0] = other[0];
        self[2] = other[3];
        self
    }
}
