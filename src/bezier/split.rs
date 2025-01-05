// Reference:
// https://github.com/fonttools/fonttools/blob/main/Lib/fontTools/cu2qu/cu2qu.py

use crate::bezier::Cubic;
use crate::Split;

macro_rules! implement {
    ($($float:ty),*) => ($(
        impl Split<$float> for Cubic<$float> {
            fn split(&self, n: usize) -> impl Iterator<Item = Self> {
                let dt = 1.0 / n as $float;
                let dt2 = dt * dt;
                let dt3 = dt * dt2;
                (0..n)
                    .map(move |i| {
                        let t = i as $float * dt;
                        let t2 = t * t;
                        Cubic {
                            a: self.a * dt3,
                            b: (3.0 * self.a * t + self.b) * dt2,
                            c: (2.0 * self.b * t + self.c + 3.0 * self.a * t2) * dt,
                            d: self.a * t * t2 + self.b * t2 + self.c * t + self.d,
                        }
                    })
            }
        }
    )*);
}

implement!(f32, f64);
