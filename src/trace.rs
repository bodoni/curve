use std::marker::PhantomData;

use num_traits::Float;

use crate::Evaluate;

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<'l, T, U> {
    curve: &'l U,
    point: usize,
    points: usize,
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
            point: 0,
            points,
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
        if self.point >= self.points {
            return None;
        }
        let time = T::from(self.point).unwrap() / T::from(self.points - 1).unwrap();
        self.point += 1;
        Some(self.curve.evaluate(time))
    }
}
