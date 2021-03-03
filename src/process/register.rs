use crate::interface::Channel;
use crate::interface::Simulable;
use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::Simulator;
use crate::trace::Trace;
use std::fmt::Display;

pub struct Reg<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a dyn Channel<'a, T>,
    q: &'a Signal<'a, T>,
}

impl<'a, T> Reg<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(d: &'a dyn Channel<'a, T>, q: &'a Signal<'a, T>) -> Self {
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

pub struct RegEn<'a, T: Copy + PartialEq + Default + Display + Trace> {
    d: &'a dyn Channel<'a, T>,
    q: &'a Signal<'a, T>,
    en: &'a dyn Channel<'a, bool>,
}

impl<'a, T> RegEn<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn new(
        d: &'a dyn Channel<'a, T>,
        q: &'a Signal<'a, T>,
        en: &'a dyn Channel<'a, bool>,
    ) -> Self {
        Self { d: d, q: q, en: en }
    }
}

impl<'a, T> Process<'a> for RegEn<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        if self.en.read() {
            self.q.write(self.d.read(), simulator);
        }
        None
    }
}
