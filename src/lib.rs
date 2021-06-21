//! An event-driven simulator specifically designed for digital circuits.

//#![warn(missing_docs)]
#![feature(cell_update)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate nadja_derive;

/// Basic combinational logic gates
pub mod gate;
mod interface;
/// Public interface
pub mod pub_interface;
pub use crate::pub_interface::{Channel, HaveProc, In, InOut, Out, Param};
pub mod logic;
pub mod process;
pub mod signal;
pub use crate::signal::Signal;
pub mod simulator;
pub use crate::simulator::Simulator;
pub mod trace;
pub mod wire;
pub use crate::wire::{Wire, WireTrig};
pub mod util;

#[cfg(test)]
mod tests {
    use crate::logic::{Logic, VLogic};
    use crate::process::clock::Clk;
    use crate::process::register::{Reg, RegRst};
    use crate::process::reset::Rst;
    use crate::signal::Signal;
    use crate::simulator::Simulator;
    use crate::wire::Wire;

    #[test]
    fn it_works() {
        let rst_n: Wire<bool> = Default::default();
        let s1: Signal<Logic> = Default::default();
        let s2: Signal<Logic> = Default::default();
        let s3: Signal<Logic> = Default::default();

        let s4: Signal<VLogic<32>> = Default::default();
        let s5: Signal<VLogic<32>> = Default::default();
        println! {"{:?}", s4};
        let r1 = RegRst::new(&s1, &s2, &rst_n);
        let r2 = RegRst::new(&s3, &s2, &rst_n);

        let clk = Clk::new(1, &[&r1, &r2], &[]);
        let rst_n_proc = Rst::new(&rst_n, false, 2, 2, &[&r1, &r2]);

        let mut sim = Simulator::new(16, &[&clk, &rst_n_proc]);
        sim.run();
    }
}
