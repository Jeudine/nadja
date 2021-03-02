use crate::signal::Signal;
use std::fmt::Display;
use crate::trace::Trace;
use crate::process::Process;
use crate::simulator::Simulator;

struct Reg<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a Signal<'a, T>, //TODO: not necessarly a signal
    q: &'a Signal<'a, T>,

    //TODO: do a register enable: RegEn
}
impl<'a, T> Reg<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(d: &'a Signal<'a, T>, q: &'a Signal<'a, T>) -> Self {
        Self {
            d: d,
            q: q,
        }
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
