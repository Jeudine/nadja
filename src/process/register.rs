use crate::interface::Channel;
use crate::interface::{Simulable, TValue};
use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::Simulator;

pub struct Reg<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
}

impl<'a, T: TValue> Reg<'a, T>
{
    pub fn new(d: &'a dyn Channel<T>, q: &'a Signal<'a, T>) -> Self {
        Self { d: d, q: q }
    }
}

impl<'a, T: TValue> Process<'a> for Reg<'a, T>
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        self.q.write(self.d.read(), simulator);
        None
    }
}

pub struct RegRst<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
    rst: &'a dyn Channel<bool>,
}

impl<'a, T: TValue> RegRst<'a, T>
{
    pub fn new(d: &'a dyn Channel<T>, q: &'a Signal<'a, T>, rst: &'a dyn Channel<bool>) -> Self {
        Self {
            d: d,
            q: q,
            rst: rst,
        }
    }
}

impl<'a, T: TValue> Process<'a> for RegRst<'a, T>
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        if self.rst.read() {
            self.q.write(self.d.read(), simulator);
        }
        None
    }
}
