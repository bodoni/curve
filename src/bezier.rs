//! Bézier curves.

use {Curve, Trace};

/// A cubic Bézier curve.
#[derive(Clone, Copy, Debug)]
pub struct Cubic {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
}

impl Cubic {
    /// Create a curve.
    #[inline]
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Cubic {
        Cubic { x0: x0, y0: y0, x1: x1, y1: y1, x2: x2, y2: y2, x3: x3, y3: y3 }
    }

    /// Start tracing the curve.
    #[inline]
    pub fn trace<'l>(&'l self, steps: usize) -> Trace<'l, Self> {
        Trace { curve: self, steps: steps, step: 0 }
    }
}

impl Curve for Cubic {
    fn evaluate(&self, t: f64) -> (f64, f64) {
        debug_assert!(0.0 <= t && t <= 1.0);
        let t2 = t * t;
        let t3 = t2 * t;
        let ct = 1.0 - t;
        let ct2 = ct * ct;
        let ct3 = ct2 * ct;
        let x = ct3 * self.x0 + 3.0 * ct2 * t * self.x1 + 3.0 * ct * t2 * self.x2 + t3 * self.x3;
        let y = ct3 * self.y0 + 3.0 * ct2 * t * self.y1 + 3.0 * ct * t2 * self.y2 + t3 * self.y3;
        (x, y)
    }
}
