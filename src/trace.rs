use std::marker::PhantomData;

use num_traits::Float;

use crate::Evaluate;

/// A trace of a curve.
#[derive(Clone, Copy, Debug)]
pub struct Trace<T, U> {
    curve: U,
    points: usize,
    index: usize,
    phantom: PhantomData<T>,
}

impl<T, U> Trace<T, U>
where
    T: Float,
    U: Evaluate<T>,
{
    /// Create an instance.
    #[inline]
    pub fn new(curve: U, points: usize) -> Self {
        Self {
            curve,
            points,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<T, U> Iterator for Trace<T, U>
where
    T: Float,
    U: Evaluate<T>,
{
    type Item = <U as Evaluate<T>>::Target;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.points {
            return None;
        }
        let time = T::from(self.index).unwrap() / T::from(self.points - 1).unwrap();
        self.index += 1;
        Some(self.curve.evaluate(time))
    }
}

#[cfg(test)]
mod tests {
    use assert;

    use crate::bezier::{Cubic, Linear, Quadratic};
    use crate::trace::Trace;

    #[test]
    fn linear() {
        let x = Linear::new(1.0, 5.0);
        let y = Linear::new(2.0, 3.0);
        let trace = vec![(1.0, 2.0), (3.0, 2.5), (5.0, 3.0)];
        assert_eq!(trace, Trace::new((x, y), 3).collect::<Vec<_>>());
    }

    #[test]
    fn quadratic() {
        let x = Quadratic::new(1.0, 3.0, 5.0);
        let y = Quadratic::new(2.0, 1.0, 3.0);
        let trace = vec![
            (1.0000000000000000e+00, 2.0000000000000000e+00),
            (1.4444444444444444e+00, 1.8148148148148147e+00),
            (1.8888888888888888e+00, 1.7037037037037037e+00),
            (2.3333333333333339e+00, 1.6666666666666667e+00),
            (2.7777777777777777e+00, 1.7037037037037037e+00),
            (3.2222222222222223e+00, 1.8148148148148149e+00),
            (3.6666666666666670e+00, 2.0000000000000000e+00),
            (4.1111111111111107e+00, 2.2592592592592595e+00),
            (4.5555555555555554e+00, 2.5925925925925926e+00),
            (5.0000000000000000e+00, 3.0000000000000000e+00),
        ];
        for (i, (x, y)) in Trace::new((x, y), 10).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }

    #[test]
    fn cubic() {
        let x = Cubic::new(1.0, 3.0, 5.0, 6.0);
        let y = Cubic::new(2.0, 1.0, 3.0, 2.0);
        let trace = vec![
            (1.0000000000000000e+00, 2.0000000000000000e+00),
            (1.6652949245541835e+00, 1.7695473251028802e+00),
            (2.3223593964334710e+00, 1.7119341563786008e+00),
            (2.9629629629629632e+00, 1.7777777777777777e+00),
            (3.5788751714677640e+00, 1.9176954732510290e+00),
            (4.1618655692729769e+00, 2.0823045267489713e+00),
            (4.7037037037037033e+00, 2.2222222222222219e+00),
            (5.1961591220850476e+00, 2.2880658436213990e+00),
            (5.6310013717421121e+00, 2.2304526748971192e+00),
            (6.0000000000000000e+00, 2.0000000000000000e+00),
        ];
        for (i, (x, y)) in Trace::new((x, y), 10).enumerate() {
            assert::close(trace[i].0, x, 1e-15);
            assert::close(trace[i].1, y, 1e-15);
        }
    }
}
