//! Curves.

/// A type identifying itself as a curve.
pub trait Curve {
    /// Evalute the curve at a point in `[0, 1]`.
    fn evaluate(&self, f64) -> (f64, f64);
}

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<'l, T: 'l + Curve> {
    curve: &'l T,
    steps: usize,
    step: usize,
}

impl<'l, T: Curve> Iterator for Trace<'l, T> {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<(f64, f64)> {
        let step = self.step;
        if step < self.steps {
            self.step += 1;
            Some(self.curve.evaluate(step as f64 / ((self.steps - 1) as f64)))
        } else {
            None
        }
    }
}

pub mod bezier;
