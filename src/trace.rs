use std::marker::PhantomData;

use num_traits::Float;

use crate::Evaluate;

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<'l, T, U> {
    curve: &'l U,
    points: usize,
    index: usize,
    phantom: PhantomData<T>,
}

impl<'l, T, U> Trace<'l, T, U>
where
    T: Float,
    U: Evaluate<T>,
{
    /// Create an instance.
    #[inline]
    pub fn new(curve: &'l U, points: usize) -> Self {
        Self {
            curve,
            points,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<T, U> Iterator for Trace<'_, T, U>
where
    T: Float,
    U: Evaluate<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points {
            return None;
        }
        let time = T::from(self.index).unwrap() / T::from(self.points - 1).unwrap();
        self.index += 1;
        Some(self.curve.evaluate(time))
    }
}
