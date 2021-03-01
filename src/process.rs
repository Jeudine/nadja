use super::simulator;
use crate::module;
pub mod always;
pub mod initial;

/// Executes the process until the end or a break.
/// In case the execution stops on a break, returns the duration of the break, otherwise return
/// None.
pub trait Process {
    fn execute(&self, module: & module::Module, simulator: &mut simulator::Simulator) -> Option<usize>;
}
