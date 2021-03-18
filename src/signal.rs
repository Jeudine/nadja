use super::process::Process;
use crate::interface::{Channel, Event, Simulable};
use crate::simulator::Simulator;
use crate::trace::Trace;
use std::cell::Cell;
use std::fmt::{Display, Formatter, Result};

pub struct Signal<'a, T: Copy + PartialEq + Default + Display + Trace> {
    cur_val: Cell<T>,
    new_val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T> Display for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.cur_val.get().fmt(f)
    }
}

impl<'a, T> Default for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn default() -> Self {
        Self {
            cur_val: Default::default(),
            new_val: Default::default(),
            sensitivity: Default::default(),
        }
    }
}

impl<'a, T> Event<'a> for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn trigger(&self) -> &[&dyn Process<'a>] {
        if self.cur_val.get() != self.new_val.get() {
            self.cur_val.set(self.new_val.get());
            &self.sensitivity[..]
        } else {
            &[]
        }
    }
}

impl<'a, T> Channel<T> for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn read(&self) -> T {
        self.cur_val.get()
    }
}

impl<'a, T> Simulable<'a, T> for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self {
        Self {
            cur_val: Cell::new(val),
            new_val: Cell::new(val),
            sensitivity: sensitivity.to_vec(),
        }
    }

    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T {
        // can be optimized
        // TODO: if write is call two times
        self.new_val.set(val);
        simulator.push(self);
        val
    }

    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T {
        // can be optimized
        // TODO: if write is call two times
        let val = f(self.cur_val.get());
        self.new_val.set(val);
        simulator.push(self);
        val
    }
}
