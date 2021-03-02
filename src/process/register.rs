use crate::process::Process;
use crate::signal::Signal;
use crate::simulable::Channel;
use crate::simulator::Simulator;
use crate::trace::Trace;
use std::fmt::Display;

pub struct Reg<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a dyn Channel<'a, T>,
    q: &'a Signal<'a, T>,
    //TODO: do a register enable: RegEn
}
impl<'a, T> Reg<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(d: &'a Signal<'a, T>, q: &'a Signal<'a, T>) -> Self {
        Self { d: d, q: q }
    }
}

impl<'a, T> Process<'a> for Reg<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        self.q.write(self.d.read(), simulator);
        None
    }
}
