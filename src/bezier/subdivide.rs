use num_traits::Float;

use crate::bezier::Cubic;
use crate::Subdivide;

impl<T> Subdivide<T> for Cubic<T>
where
    T: Float + Default,
{
    fn subdivide(&self, t: T) -> (Self, Self) {
        debug_assert!(T::zero() <= t && t <= T::one());
        let u = T::one() - t;
        let mut beta = self.0;

        let mut head: [T; 4] = Default::default();
        let mut tail: [T; 4] = Default::default();

        head[0] = beta[0];

        beta[0] = beta[0] * u + beta[1] * t;
        head[1] = beta[0];

        beta[1] = beta[1] * u + beta[2] * t;

        beta[2] = beta[2] * u + beta[3] * t;
        tail[2] = beta[2];

        beta[0] = beta[0] * u + beta[1] * t;
        head[2] = beta[0];

        beta[1] = beta[1] * u + beta[2] * t;
        tail[1] = beta[1];

        beta[0] = beta[0] * u + beta[1] * t;
        head[3] = beta[0];
        tail[0] = beta[0];

        tail[3] = beta[3];

        (Self(head), Self(tail))
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
        assert_eq!(
            x.subdivide(0.5),
            (
                Cubic::new(2.0, 3.0, 4.0, 5.0),
                Cubic::new(5.0, 6.0, 7.0, 8.0),
            ),
        );
        assert_eq!(
            y.subdivide(0.5),
            (
                Cubic::new(1.0, 1.5, 2.0, 2.5),
                Cubic::new(2.5, 3.0, 3.5, 4.0),
            ),
        );
    }
}
