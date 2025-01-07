/// A type that can compare curves.
pub trait Compare<T> {
    /// Perform the calculation.
    fn compare(&self, one: &T, other: &T) -> bool;
}
