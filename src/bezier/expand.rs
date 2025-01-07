// Reference:
// https://pomax.github.io/bezierinfo/#reordering

use crate::bezier::{Cubic, Quadratic};
use crate::expand::Expand;

macro_rules! implement {
    ($($type:ty),*) => ($(
        impl Expand<$type> for Quadratic<$type> {
            type Target = Cubic<$type>;

            fn expand(&self) -> Self::Target {
                let beta = self.0;
                Self::Target::new(
                    beta[0],
                    (2.0 * beta[1] + beta[0]) / 3.0,
                    (beta[2] + 2.0 * beta[1]) / 3.0,
                    beta[2],
                )
            }
        }
    )*);
}

implement!(f32, f64);

#[cfg(test)]
mod tests {
    use crate::bezier::{Cubic, Quadratic};
    use crate::expand::Expand;
    use crate::reduce::Reduce;

    #[test]
    fn expand() {
        let x: Cubic<_> = Quadratic::new(-10.0, 50.0, 110.0).expand();
        let y: Cubic<_> = Quadratic::new(0.0, 150.0, 0.0).expand();
        assert_eq!(x, Cubic::new(-10.0, 30.0, 70.0, 110.0));
        assert_eq!(y, Cubic::new(0.0, 100.0, 100.0, 0.0));
        assert_eq!(x.reduce(), Quadratic::new(-10.0, 50.0, 110.0));
        assert_eq!(y.reduce(), Quadratic::new(0.0, 150.0, 0.0));
    }
}
