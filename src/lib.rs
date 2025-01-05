//! Curves.

use std::marker::PhantomData;

use num_traits::Float;

/// A curve that can be evaluated.
pub trait Evaluate<T: Float> {
    /// Evaluate the curve at a point in `[0, 1]`.
    fn evaluate(&self, t: T) -> T;
}

/// A curve that can be subdivided.
pub trait Subdivide<T: Float>: Sized {
    /// Subdivide the curve into several.
    fn subdivide(&self, n: usize) -> impl Iterator<Item = Self>;
}

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
    #[inline]
    fn new(curve: &'l U, steps: usize) -> Self {
        Self {
            curve,
            steps,
            position: 0,
            phantom: PhantomData,
        }
    }
}

impl<'l, T, U> Iterator for Trace<'l, T, U>
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
        T::from(position)
            .zip(T::from(self.steps - 1))
            .map(|(numerator, denominator)| self.curve.evaluate(numerator / denominator))
    }
}

pub mod bezier;
