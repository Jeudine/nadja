pub mod clock;
pub use clock::Clk;
pub mod register;
pub use register::{Reg, RegRst, FF};
pub mod reset;
use crate::simulator::Simulator;
pub use reset::{RstBool, RstLogic};

/// Executes the process until the end, a break or a stop call.
/// In case the execution breaks, returns the duration of the break.
pub trait Process<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> ProcessRes;
}

pub enum ProcessRes {
    End,
    Break(usize),
    Stop,
}
