/// A goodness of fit.
pub trait Goodness<T> {
    /// Check if the candidate is admissible.
    fn admit(&self, candidate: &T, original: &T) -> bool;
}
