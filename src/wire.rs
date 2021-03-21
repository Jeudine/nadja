use super::process::Process;
use super::simulator::Simulator;
use crate::interface::{Channel, Event, Simulable, TValue};
use std::cell::Cell;
use std::fmt::{Debug, Formatter, Result};

pub struct Wire<T: TValue> {
    val: Cell<T>,
}

impl<T: TValue> Wire<T>
{
    pub fn new(val: T) -> Self {
        Self {
            val: Cell::new(val),
        }
    }

    pub fn write(&self, val: T) -> T {
        self.val.set(val);
        val
    }

    pub fn update(&self, f: &dyn Fn(T) -> T) -> T {
        self.val.update(f)
    }
}

impl<T: TValue> Debug for Wire<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.val.get().fmt(f)
    }
}

impl<T: TValue> Default for Wire<T>
{
    fn default() -> Self {
        Self {
            val: Default::default(),
        }
    }
}

impl<T: TValue> Channel<T> for Wire<T>
{
    fn read(&self) -> T {
        self.val.get()
    }
}

pub struct WireTrig<'a, T: TValue> {
    val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T: TValue> Debug for WireTrig<'a, T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.val.get().fmt(f)
    }
}

impl<'a, T: TValue> Default for WireTrig<'a, T>
{
    fn default() -> Self {
        Self {
            val: Default::default(),
            sensitivity: Default::default(),
        }
    }
}

impl<'a, T: TValue> Event<'a> for WireTrig<'a, T>
{
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}

impl<'a, T: TValue> Channel<T> for WireTrig<'a, T>
{
    fn read(&self) -> T {
        self.val.get()
    }
}

impl<'a, T: TValue> Simulable<'a, T> for WireTrig<'a, T>
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self {
        Self {
            val: Cell::new(val),
            sensitivity: sensitivity.to_vec(),
        }
    }

    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T {
        if val != self.val.get() {
            self.val.set(val);
            simulator.push(self);
        }
        val
    }

    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T {
        let cur_val = self.val.get();
        let new_val = f(cur_val);
        if cur_val != new_val {
            self.val.set(new_val);
            simulator.push(self);
        }
        new_val
    }
}
