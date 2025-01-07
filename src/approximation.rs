use num_traits::Float;

use crate::compare::Compare;
use crate::expand::Expand;
use crate::reduce::Reduce;
use crate::subdivide::Subdivide;

/// An approximation of a curve.
pub struct Approximation<'l, T, U, V> {
    curve: &'l U,
    comparision: V,
    phantom: std::marker::PhantomData<T>,
}

impl<'l, T, U, V> Approximation<'l, T, U, V> {
    /// Create an instance.
    pub fn new(curve: &'l U, comparision: V) -> Self {
        Self {
            curve,
            comparision,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T, U, V> Iterator for Approximation<'_, T, U, V>
where
    T: Float,
    U: Reduce<T> + Subdivide<T>,
    <U as Reduce<T>>::Target: Expand<T, Target = U>,
    V: Compare<U>,
{
    type Item = Vec<<U as Reduce<T>>::Target>;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.curve.reduce();
        if self.comparision.compare(self.curve, &candidate.expand()) {
            Some(vec![candidate])
        } else {
            None
        }
    }
}
