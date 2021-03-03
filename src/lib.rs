pub mod process;
pub mod signal;
pub mod interface;
pub mod simulator;
pub mod trace;
pub mod gate;

pub use crate::signal::Signal;

#[cfg(test)]
mod tests {
    use crate::process::clock::Clk;
    use crate::process::register::Reg;
    use crate::signal::Signal;
    use crate::simulable::Simulable;
    use crate::simulator::Simulator;
    use crate::wire::Wire;

    #[test]
    fn it_works() {
        let mut sim: Simulator = Default::default();

        let s1: Signal<bool> = Default::default();
        let s2: Signal<bool> = Default::default();
        let s3: Signal<bool> = Default::default();

        let r1 = Reg::new(&s1, &s2);
        let r2 = Reg::new(&s3, &s2);

        let clk = Wire::new(true, &[&r1, &r2]);
        let clk_p = Clk::new(&clk, 1);
        sim.start(8, &[&clk_p]);
    }
}
