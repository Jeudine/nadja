use crate::interface::Notify;
use crate::process::Process;
use crate::simulator::Simulator;
use crate::wire::Wire;

pub struct Rst<'a> {
    rst: &'a Wire<bool>, //use a wire without sensitivity
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a> Rst<'a> {
    pub fn new(
        wire: &'a Wire<bool>,
        active_edge: bool,
        sensitivity: &[&'a dyn Process<'a>],
    ) -> Self {
        wire.write(!active_edge);
        Self {
            rst: wire,
            sensitivity: sensitivity.to_vec(),
        }
    }
}

impl<'a> Process<'a> for Rst<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize> {
        self.rst.update(&|x| !x);
        simulator.push(self);
        None
    }
}

impl<'a> Notify<'a> for Rst<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}
