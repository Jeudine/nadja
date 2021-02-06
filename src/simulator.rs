use crate::signal;
use crate::trace;

struct Simulator {
}

impl Simulator {
    fn update<T: Copy + PartialEq + Default + std::fmt::Display + trace::Trace>(s:signal::Signal<T>) {
    }
}
