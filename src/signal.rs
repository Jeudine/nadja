use super::process;
use super::simulator;
use super::trace;
use std::fmt;

pub struct Signal<T: Copy + PartialEq + Default + fmt::Display + trace::Trace> {
    cur_val: T,
    new_val: T,
    sensitivity: Vec<&'static dyn process::Process>,
}

impl<T> Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    pub fn read(&self) -> T {
        self.cur_val
    }

    pub fn write(&mut self, v: T) {
        self.new_val = v;
    }

    pub fn add_process(&mut self, p: &'static dyn process::Process) {
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
        Signal {
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
        self.cur_val.fmt(f)
    }
}

impl<T> simulator::Update for Signal<T>
where
    T: Copy + PartialEq + Default + fmt::Display + trace::Trace,
{
    fn update(&mut self) -> Option<&[&'static dyn process::Process]> {
        if self.cur_val != self.new_val {
            self.cur_val = self.new_val;
            Option::Some(&self.sensitivity[..])
        } else {
            Option::None
        }
    }
}
