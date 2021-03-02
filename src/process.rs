use super::simulator::Simulator;
pub mod register;
pub mod clock;

/// Executes the process until the end or a break.
/// In case the execution stops on a break, returns the duration of the break, otherwise return
/// None.
pub trait Process<'a> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize>;
}
