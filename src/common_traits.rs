/// Both incrementable and decrementable.
pub trait Scrollable {
    /// Get the next value for the object.
    fn next(&self) -> Self;

    /// Get the previous value for the object.
    fn prev(&self) -> Self;

    /// Set the object's value to that of `next()`.
    fn incr(&mut self)
    where
        Self: Sized,
    {
        *self = self.next();
    }

    /// Set the object's value to that of `prev()`.
    fn decr(&mut self)
    where
        Self: Sized,
    {
        *self = self.prev();
    }
}
