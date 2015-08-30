//! Curves.

extern crate num;

use num::Float;
use std::marker::PhantomData;

/// A type identifying itself as a curve.
pub trait Curve<T: Float> {
    /// Evalute the curve at a point in `[0, 1]`.
    fn evaluate(&self, T) -> (T, T);
}

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<'l, T: Float, C: 'l + Curve<T>> {
    curve: &'l C,
    steps: usize,
    position: usize,
    phantom: PhantomData<T>,
}

impl<'l, T: Float, C: Curve<T>> Trace<'l, T, C> {
    #[inline(always)]
    fn new(curve: &'l C, steps: usize) -> Self {
        Trace { curve: curve, steps: steps, position: 0, phantom: PhantomData }
    }
}

macro_rules! implement {
    ($float:ty) => (
        impl<'l, T: Curve<$float>> Iterator for Trace<'l, $float, T> {
            type Item = ($float, $float);

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
    );
}

implement!(f32);
implement!(f64);

pub mod bezier;
