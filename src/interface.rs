use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use std::fmt::{Display, Formatter, Result};

pub trait Channel<'a, T: Copy + PartialEq + Display> {
    fn read(&self) -> T;
}

impl<'a, T> Display for dyn Channel<'a, T>
where
    T: Copy + PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.read().fmt(f)
    }
}

pub trait Notify<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>];
}

pub trait Simulable<'a, T: Copy + PartialEq + Display + Default + Trace>:
    Default + Channel<'a, T> + Notify<'a>
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self;
    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T;
    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T;
}
