use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use std::fmt::Display;

pub trait Notify<'a> {
    fn update(&self) -> Option<&[&dyn Process<'a>]>;
}

pub trait Channel<'a, T: Copy + PartialEq + Display>: Display + Notify<'a> {
    fn read(&self) -> T;
    fn write(&'a self, val: T, sim: &mut Simulator<'a>);
}

pub trait Simulable<'a, T: Copy + PartialEq + Default + Display + Trace>:
    Default + Channel<'a, T>
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self;
}
