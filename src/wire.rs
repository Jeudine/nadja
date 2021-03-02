use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use crate::simulable::{Channel, Notify, Simulable};
use std::cell::Cell;
use std::fmt::{Display, Formatter, Result};

pub struct Wire<'a, T: Copy + PartialEq + Default + Display + Trace> {
    val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T> Display for Wire<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.val.get().fmt(f)
    }
}

impl<'a, T> Default for Wire<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn default() -> Self {
        Self {
            val: Default::default(),
            sensitivity: Default::default(),
        }
    }
}

impl<'a, T> Notify<'a> for Wire<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn trigger(&self) -> Option<&[&dyn Process<'a>]> {
        Option::Some(&self.sensitivity[..])
    }
}

impl<'a, T> Channel<'a, T> for Wire<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn read(&self) -> T {
        self.val.get()
    }

    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T {
        // can be optimized
        // TODO: if write is call two times
        self.val.set(val);
        simulator.push(self);
        val
    }

    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T {
        // can be optimized
        // TODO
        let val = f(self.val.get());
        self.val.set(val);
        simulator.push(self);
        val
    }
}

impl<'a, T> Simulable<'a, T> for Wire<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self {
        Self {
            val: Cell::new(val),
            sensitivity: sensitivity.to_vec(),
        }
    }
}
