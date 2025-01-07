use num_traits::Float;

use crate::compare::Compare;
use crate::expand::Expand;
use crate::reduce::Reduce;
use crate::subdivide::Subdivide;

/// An approximation of a curve.
pub struct Approximation<T, U, V> {
    curves: Vec<U>,
    comparision: V,
    time: T,
}

impl<T, U, V> Approximation<T, U, V>
where
    T: Float,
{
    /// Create an instance.
    pub fn new(curve: U, comparision: V) -> Self {
        let one = T::one();
        Self {
            curves: vec![curve],
            comparision,
            time: one / (one + one),
        }
    }
}

impl<T, U, V> Iterator for Approximation<T, U, V>
where
    T: Float,
    U: Reduce<T> + Subdivide<T>,
    <U as Reduce<T>>::Target: Expand<T, Target = U>,
    V: Compare<U>,
{
    type Item = <U as Reduce<T>>::Target;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(original) = self.curves.pop() {
            let reduced = original.reduce();
            let expanded = reduced.expand();
            if self.comparision.compare(&original, &expanded) {
                return Some(reduced);
            }
            let (head, tail) = original.subdivide(self.time);
            self.curves.push(tail);
            self.curves.push(head);
        }
        None
    }
}
