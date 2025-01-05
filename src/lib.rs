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
pub struct Trace<'l, T: Float, C: 'l + Evaluate<T>> {
    curve: &'l C,
    steps: usize,
    position: usize,
    phantom: PhantomData<T>,
}

impl<'l, T: Float, C: Evaluate<T>> Trace<'l, T, C> {
    #[inline]
    fn new(curve: &'l C, steps: usize) -> Self {
        Self {
            curve,
            steps,
            position: 0,
            phantom: PhantomData,
        }
    }
}

macro_rules! implement {
    ($($float:ty),*) => ($(
        impl<'l, T: Evaluate<$float>> Iterator for Trace<'l, $float, T> {
            type Item = $float;

            fn next(&mut self) -> Option<Self::Item> {
                let position = self.position;
                if position < self.steps {
                    self.position += 1;
                    Some(self.curve.evaluate(position as $float / (self.steps - 1) as $float))
                } else {
                    None
                }
            }
        }
    )*);
}

implement!(f32, f64);

pub mod bezier;
