pub mod clock;
pub use clock::Clk;
pub mod register;
pub use register::{Reg, RegRst, FF};
pub mod reset;
pub use reset::Rst;

/*
mod interface {
    use crate::simulator::Simulator;

    /// Executes the process until the end or a break.
    /// In case the execution stops on a break, returns the duration of the break, otherwise return
    /// None.
    pub trait Process<'a> {
        fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize>;
    }
}
*/
