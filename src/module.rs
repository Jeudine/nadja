use crate::interface::TValue;
use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::Simulator;

pub trait Module {
    fn route() -> Self;
}
