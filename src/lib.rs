pub mod gate;
pub mod interface;
pub mod logic;
pub mod process;
pub mod signal;
pub mod simulator;
pub mod trace;
pub mod wire;

pub use crate::signal::Signal;

#[cfg(test)]
mod tests {
    use crate::interface::Simulable;
    use crate::process::clock::Clk;
    use crate::process::register::{Reg, RegRst};
    use crate::process::reset::Rst;
    use crate::signal::Signal;
    use crate::simulator::Simulator;
    use crate::wire::Wire;

    #[test]
    fn it_works() {
        let rst_n: Wire<bool> = Default::default();
        let s1: Signal<bool> = Default::default();
        let s2: Signal<bool> = Default::default();
        let s3: Signal<bool> = Default::default();

        let r1 = RegRst::new(&s1, &s2, &rst_n);
        let r2 = RegRst::new(&s3, &s2, &rst_n);

        let clk = Clk::new(0, &[&r1, &r2], &[]);
        let rst_n_proc = Rst::new(&rst_n, false, 2, 2, &[&r1, &r2]);

        let mut sim = Simulator::new(16, &[&clk, &rst_n_proc]);
        sim.run();
    }
}
