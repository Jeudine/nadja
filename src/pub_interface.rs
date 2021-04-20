use crate::interface::TChannel;

///Each node of a circuit can be represented by a variable implementing the [`Channel<T>`] trait (with `T`, the actual type of the circuit node).
///
///This trait is hidden by the library and you usually don't have to think about it.
pub trait Channel<T: TChannel> {
    /// Return the value of the circuit node
    fn read(&self) -> T;
}
