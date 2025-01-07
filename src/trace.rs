use std::marker::PhantomData;

use num_traits::Float;

use crate::Evaluate;

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<'l, T, U> {
    curve: &'l U,
    steps: usize,
    position: usize,
    phantom: PhantomData<T>,
}

impl<'l, T, U> Trace<'l, T, U>
where
    T: Float,
    U: Evaluate<T>,
{
    /// Create an instance.
    #[inline]
    pub fn new(curve: &'l U, steps: usize) -> Self {
        Self {
            curve,
            steps,
            position: 0,
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
        let position = self.position;
        if position >= self.steps {
            return None;
        }
        self.position += 1;
        let t = T::from(position).unwrap() / T::from(self.steps - 1).unwrap();
        Some(self.curve.evaluate(t))
    }
}
