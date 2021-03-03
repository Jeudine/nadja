use crate::process::Process;
use crate::simulator::Simulator;
use crate::signal::Signal;
use crate::interface::Simulable;

pub struct Clk<'a> {
    clk: &'a Signal<'a, bool>,
    half_period: usize,
}

impl<'a> Clk<'a> {
    pub fn new(clk: &'a Signal<'a, bool>, half_period: usize) -> Self {
        Self {
            clk: clk,
            half_period: half_period,
        }
    }
}

impl<'a> Process<'a> for Clk<'a> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        println!("clk: {}", self.clk);
        self.clk.update(&|x: bool| !x, simulator);
        Some(self.half_period)
    }
}
