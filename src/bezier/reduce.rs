// Reference:
// https://pomax.github.io/bezierinfo/#reordering

use crate::bezier::{Cubic, Quadratic};
use crate::Reduce;

macro_rules! implement {
    ($($type:ty),*) => ($(
        impl Reduce<$type> for Cubic<$type> {
            type Target = Quadratic<$type>;

            fn reduce(&self) -> Self::Target {
                let beta = self.0;
                Self::Target::new(
                    0.95 * beta[0] + 0.15 * beta[1] - 0.15 * beta[2] + 0.05 * beta[3],
                    -0.25 * beta[0] + 0.75 * beta[1] + 0.75 * beta[2] - 0.25 * beta[3],
                    0.05 * beta[0] - 0.15 * beta[1] + 0.15 * beta[2] + 0.95 * beta[3],
                )
            }
        }
    )*);
}

implement!(f32, f64);

#[cfg(test)]
mod tests {
    use crate::bezier::{Cubic, Quadratic};
    use crate::Reduce;

    #[test]
    fn reduce() {
        let x = Cubic::new(0.0, 0.0, 100.0, 100.0).reduce();
        let y = Cubic::new(0.0, 100.0, 100.0, 0.0).reduce();
        assert_eq!(x, Quadratic::new(-10.0, 50.0, 110.0));
        assert_eq!(y, Quadratic::new(0.0, 150.0, 0.0));
    }
}
