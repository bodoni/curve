use num_traits::Float;

use crate::align::Align;
use crate::expand::Expand;
use crate::goodness::Goodness;
use crate::reduce::Reduce;
use crate::subdivide::Subdivide;

/// An approximation of a curve.
pub struct Approximation<T, U, V> {
    curves: Vec<U>,
    goodness: V,
    time: T,
}

impl<T, U, V> Approximation<T, U, V>
where
    T: Float,
{
    /// Create an instance.
    pub fn new(curve: U, goodness: V) -> Self {
        let one = T::one();
        Self {
            curves: vec![curve],
            goodness,
            time: one / (one + one),
        }
    }
}

impl<T, U, V> Iterator for Approximation<T, U, V>
where
    T: Float,
    U: Reduce<T> + Subdivide<T>,
    <U as Reduce<T>>::Target: Align<T, U> + Expand<T, Target = U>,
    V: Goodness<U>,
{
    type Item = <U as Reduce<T>>::Target;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(original) = self.curves.pop() {
            let reduced = original.reduce().align(&original);
            let expanded = reduced.expand();
            if self.goodness.admit(&original, &expanded) {
                return Some(reduced);
            }
            let (head, tail) = original.subdivide(self.time);
            self.curves.push(tail);
            self.curves.push(head);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::approximation::Approximation;
    use crate::bezier::goodness::CrudeIndependentAbsolute;
    use crate::bezier::{Cubic, Quadratic};
    use crate::expand::Expand;

    #[test]
    fn approximate() {
        let goodness = CrudeIndependentAbsolute::new(1.0);
        let x = Cubic::new(0.0, 0.0, 90.0, 100.0);
        let y = Cubic::new(0.0, 50.0, 0.0, 0.0);
        assert_eq!(
            render(Approximation::new((x, y), goodness)),
            "M0,0 Q1,18,14,21 Q28,24,46,19 Q65,13,80,7 Q96,1,100,0",
        );
    }

    #[test]
    fn exact() {
        let goodness = CrudeIndependentAbsolute::new(1e-6);
        let x = Quadratic::new(0.0, 50.0, 100.0).expand();
        let y = Quadratic::new(0.0, 100.0, 0.0).expand();
        assert_eq!(
            render(Approximation::new((x, y), goodness)),
            "M0,0 Q50,100,100,0",
        );
    }

    fn render<T>(curves: T) -> String
    where
        T: Iterator<Item = (Quadratic<f64>, Quadratic<f64>)>,
    {
        let curves = curves
            .map(|(x, y)| format!("Q{:.0},{:.0},{:.0},{:.0}", x[1], y[1], x[2], y[2]))
            .collect::<Vec<_>>()
            .join(" ");
        format!(r"M0,0 {curves}")
    }
}
