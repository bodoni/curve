// Reference:
// https://github.com/fonttools/fonttools/blob/main/Lib/fontTools/cu2qu/cu2qu.py

use num_traits::Float;

use crate::bezier::Cubic;
use crate::Subdivide;

impl<T> Subdivide<T> for Cubic<T>
where
    T: Float,
{
    fn subdivide(&self, n: usize) -> impl Iterator<Item = Self> {
        let dt = T::from(n).unwrap().recip();
        let dt2 = dt * dt;
        let dt3 = dt * dt2;
        let two = T::one() + T::one();
        let three = two + T::one();
        (0..n).map(move |i| {
            let t = T::from(i).unwrap() * dt;
            let t2 = t * t;
            Self {
                a: self.a * dt3,
                b: (three * self.a * t + self.b) * dt2,
                c: (two * self.b * t + self.c + three * self.a * t2) * dt,
                d: self.a * t * t2 + self.b * t2 + self.c * t + self.d,
            }
        })
    }
}
