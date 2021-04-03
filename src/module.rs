use crate::interface::TValue;
use crate::signal::Signal;
use crate::simulator::Simulator;

pub trait Module {
    fn route() -> Self;
}
