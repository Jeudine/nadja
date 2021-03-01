use super::process::Process;
use super::simulator;
use super::trace;
use std::fmt;
use std::cell::Cell;

//TODO Take a look at update method from cell
pub struct Signal<T: Copy + PartialEq + Default + fmt::Display + trace::Trace> {
    cur_val: Cell<T>,
    new_val: Cell<T>,
    sensitivity: Vec<&'static dyn Process>,
}

impl<T> Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    pub fn read(&self) -> T {
        self.cur_val.get()
    }

    pub fn write<'a>(&'a self, v: T, simulator: &mut simulator::Simulator<'a>) {
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

    pub fn add_process(&mut self, p: &'static dyn Process) {
        self.sensitivity.push(p);
    }
}

impl<T> PartialEq for Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    fn eq(&self, other: &Self) -> bool {
        self.cur_val == other.cur_val
    }
}

impl<T> Default for Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    fn default() -> Self {
        Self {
            cur_val: Default::default(),
            new_val: Default::default(),
            sensitivity: Default::default(),
        }
    }
}

impl<T> fmt::Display for Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cur_val.get().fmt(f)
    }
}

impl<T> simulator::Update for Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    fn update(& self) -> Option<&[&'static dyn Process]> {
        if self.cur_val.get() != self.new_val.get() {
            self.cur_val.set(self.new_val.get());
            Option::Some(&self.sensitivity[..])
        } else {
            Option::None
        }
    }
}
