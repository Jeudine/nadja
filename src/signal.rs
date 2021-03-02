use super::process::Process;
use super::simulator;
use super::trace::Trace;
use std::cell::Cell;
use std::fmt::{Display, Result, Formatter};

//TODO Take a look at update method from cell
pub struct Signal<'a, T: Copy + PartialEq + Default + Display + Trace> {
    cur_val: Cell<T>,
    new_val: Cell<T>,
    sensitivity: Vec<&'a dyn Process<'a>>,
}

impl<'a, T> Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    pub fn read(&self) -> T {
        self.cur_val.get()
    }

    pub fn write(&'a self, v: T, simulator: &mut simulator::Simulator<'a>) {
        // can be optimized
        // TODO: if write is call two times
        self.new_val.set(v);
        simulator.push(self);
    }

    /*
    pub fn block_write<'a>(&'a mut self, v: T) {
        self.cur_val = v;
    }
    */

    /*
    pub fn add_process(&mut self, p: &'static dyn Process) {
        self.sensitivity.push(p);
    }
    */
}

impl<'a, T> PartialEq for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn eq(&self, other: &Self) -> bool {
        self.cur_val == other.cur_val
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

impl<'a, T> Display for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.cur_val.get().fmt(f)
    }
}

impl<'a, T> simulator::Update<'a> for Signal<'a, T>
where
    T: Copy + PartialEq + Default + Display + Trace,
{
    fn update(&self) -> Option<&[& dyn Process<'a>]> {
        if self.cur_val.get() != self.new_val.get() {
            self.cur_val.set(self.new_val.get());
            Option::Some(&self.sensitivity[..])
        } else {
            Option::None
        }
    }
}
