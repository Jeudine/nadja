use crate::interface::Notify;
use crate::process::Process;
use crate::simulator::Simulator;
use std::cell::Cell;

pub struct Clk<'a> {
    clk: Cell<bool>,
    half_period: usize,
    posedge_sensitivity: Vec<&'a dyn Process<'a>>,
    negedge_sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a> Clk<'a> {
    pub fn new(
        half_period: usize,
        posedge_sensitivity: &[&'a dyn Process<'a>],
        negedge_sensitivity: &[&'a dyn Process<'a>],
    ) -> Self {
        Self {
            clk: Cell::new(true),
            half_period: half_period,
            posedge_sensitivity: posedge_sensitivity.to_vec(),
            negedge_sensitivity: negedge_sensitivity.to_vec(),
        }
    }
}

impl<'a> Process<'a> for Clk<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize> {
        println!("clk: {}", self.clk.get());
        self.clk.set(!self.clk.get());
        simulator.push(self);
        Some(self.half_period)
    }
}

impl<'a> Notify<'a> for Clk<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>] {
        if self.clk.get() {
            &self.posedge_sensitivity[..]
        } else {
            &self.negedge_sensitivity[..]
        }
    }
}
