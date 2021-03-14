use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use crate::interface::{Channel, Notify, Simulable};
use std::cell::Cell;
use std::fmt::{Display, Formatter, Result};

pub struct Wire<T: Copy + PartialEq + Display> {
    val: Cell<T>,
}

impl<T> Wire<T>
where
    T: Copy + PartialEq + Default + Display + Trace,
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
        let val = f(self.val.get());
        self.val.set(val);
        val
    }
}

impl<T> Display for Wire<T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.val.get().fmt(f)
    }
}

impl<T> Default for Wire<T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn default() -> Self {
        Self {
            val: Default::default(),
        }
    }
}

impl<T> Channel<T> for Wire<T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn read(&self) -> T {
        self.val.get()
    }
}

pub struct WireTrig<'a, T: Copy + PartialEq + Display> {
    val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T> Display for WireTrig<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.val.get().fmt(f)
    }
}

impl<'a, T> Default for WireTrig<'a, T>
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

impl<'a, T> Notify<'a> for WireTrig<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}

impl<'a, T> Channel<T> for WireTrig<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn read(&self) -> T {
        self.val.get()
    }
}

impl<'a, T> Simulable<'a, T> for WireTrig<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
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
