use num_traits::Float;

use crate::bezier::Cubic;
use crate::Subdivide;

impl<T> Subdivide<T> for Cubic<T>
where
    T: Float,
{
    fn subdivide(&self, _: usize) -> impl Iterator<Item = Self> {
        vec![].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::bezier::Cubic;
    use crate::Subdivide;

    #[test]
    fn subdivide() {
        let x = Cubic::new(2.0, 4.0, 6.0, 8.0);
        let y = Cubic::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(x.subdivide(2).collect::<Vec<_>>(), vec![]);
        assert_eq!(y.subdivide(2).collect::<Vec<_>>(), vec![]);
    }
}
