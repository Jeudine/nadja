use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use std::fmt::Display;

pub trait Notify<'a> {
    fn trigger(&self) -> Option<&[&dyn Process<'a>]>;
}

pub trait Channel<'a, T: Copy + PartialEq + Display>: Display + Notify<'a> {
    fn read(&self) -> T;
    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T;
    fn update(&'a self, f: & dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T;
}

pub trait Simulable<'a, T: Copy + PartialEq + Default + Display + Trace>:
    Default + Channel<'a, T>
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self;
}
