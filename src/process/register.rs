use crate::interface::Channel;
use crate::interface::Simulable;
use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::Simulator;
use crate::trace::Trace;
use std::fmt::Display;

pub struct Reg<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
}

impl<'a, T> Reg<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(d: &'a dyn Channel<T>, q: &'a Signal<'a, T>) -> Self {
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

pub struct RegRst<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
    rst: &'a dyn Channel<bool>,
}

impl<'a, T> RegRst<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(d: &'a dyn Channel<T>, q: &'a Signal<'a, T>, rst: &'a dyn Channel<bool>) -> Self {
        Self {
            d: d,
            q: q,
            rst: rst,
        }
    }
}

impl<'a, T> Process<'a> for RegRst<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        if self.rst.read() {
            self.q.write(self.d.read(), simulator);
        }
        None
    }
}
