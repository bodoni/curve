/// A goodness of fit.
pub trait Goodness<T> {
    /// Check if the candidate is admissible.
    fn admit(&mut self, candidate: &T, original: &T) -> Option<bool>;
}
