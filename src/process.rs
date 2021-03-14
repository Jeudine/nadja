use super::simulator::Simulator;

pub mod clock;
pub mod register;
pub mod reset;

/// Executes the process until the end or a break.
/// In case the execution stops on a break, returns the duration of the break, otherwise return
/// None.
pub trait Process<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize>;
}
