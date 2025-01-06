use num_traits::Float;

use crate::bezier::Cubic;
use crate::Subdivide;

impl<T> Subdivide<T> for Cubic<T>
where
    T: Float + Default,
{
    fn subdivide(&self, _: T) -> (Self, Self) {
        Default::default()
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
        assert_eq!(x.subdivide(0.5), Default::default());
        assert_eq!(y.subdivide(0.5), Default::default());
    }
}
