use super::nj_trace;
use std::fmt;

pub struct Signal<T: Copy + PartialEq + Default + fmt::Display + nj_trace::Trace> {
    cur_val:T,
    new_val:T
}

impl<T: Copy + PartialEq + Default + fmt::Display + nj_trace::Trace> Signal<T> {
    pub fn read(&self) -> &T {
        &self.cur_val
    }

    pub fn write(&mut self, v:&T) {
        self.new_val = *v;
    }

    fn update(&mut self) {
        self.cur_val = self.new_val;
    }
}

impl <T: Copy + PartialEq + Default + fmt::Display + nj_trace::Trace> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cur_val == other.cur_val
    }
}

impl <T: Copy + PartialEq + Default + fmt::Display + nj_trace::Trace> Default for Signal<T> {
    fn default() -> Self {
        Signal { cur_val: Default::default(), new_val: Default::default() }
    }
}

impl <T: Copy + PartialEq + Default + fmt::Display + nj_trace::Trace> fmt::Display for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cur_val.fmt(f)
    }
}
