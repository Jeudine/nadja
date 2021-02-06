use crate::nj_signal;
use crate::nj_trace;

struct Simulator {
}

impl Simulator {
    fn update<T: Copy + PartialEq + Default + std::fmt::Display + nj_trace::Trace>(s:nj_signal::Signal<T>) {
    }
}
