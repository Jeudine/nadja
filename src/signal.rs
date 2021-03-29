use super::process::Process;
use crate::interface::{Channel, Event, TValue, Simulable};
use crate::simulator::Simulator;
use std::cell::Cell;
use std::fmt::{Debug, Formatter, Result};

pub struct Signal<'a, T: TValue> {
    cur_val: Cell<T>,
    new_val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T: TValue> Debug for Signal<'a, T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.cur_val.get().fmt(f)
    }
}

impl<'a, T: TValue> Default for Signal<'a, T>
{
    fn default() -> Self {
        Self {
            cur_val: Default::default(),
            new_val: Default::default(),
            sensitivity: Default::default(),
        }
    }
}

impl<'a, T: TValue> Event<'a> for Signal<'a, T>
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

impl<'a, T: TValue> Channel<T> for Signal<'a, T>
{
    fn read(&self) -> T {
        self.cur_val.get()
    }
}

impl<'a, T: TValue> Simulable<'a, T> for Signal<'a, T>
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
