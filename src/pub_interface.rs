use crate::interface::TChannel;
use crate::process::Process;

///Each node of a circuit can be represented by a variable implementing the [`Channel<T>`] trait (with `T`, the actual type of the circuit node).
///
///This trait is hidden by the library and you usually don't have to think about it.
pub trait Channel<T: TChannel> {
    /// Return the value of the circuit node
    fn read(&self) -> T;
}

///This trait is hidden by the library and you usually don't have to think about it.
pub trait HaveProc<'a> {
    fn get_procs(&'a self) -> Vec<&'a dyn Process<'a>>;
}
/// Input port of a module
pub type In<T> = dyn Channel<T>;

/// Output port of a module
pub type Out<T> = dyn Channel<T>;

/// Inout port of a module
pub type InOut<T> = dyn Channel<T>;

/// Parameter of a module
pub type Param<T> = T;
