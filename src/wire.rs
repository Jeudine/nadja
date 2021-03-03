use super::process::CombProcess;
use crate::simulable::Channel;
use std::fmt::{Display, Formatter, Result};

pub struct Wire<'a, T: Copy + PartialEq + Display> {
    comb_process: &'a dyn CombProcess<T>,
}

impl<'a, T> Display for Wire<'a, T>
where
    T: Copy + PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.comb_process.execute().fmt(f)
    }
}

impl<'a, T> Channel<'a, T> for Wire<'a, T>
where
    T: Copy + PartialEq + Display,
{
    fn read(&self) -> T {
        self.comb_process.execute()
    }
}

impl<'a, T> Wire<'a, T>
where
    T: Copy + PartialEq + Display,
{
    fn new(comb_process: &'a dyn CombProcess<T>) -> Self {
        Self {
            comb_process: comb_process,
        }
    }
}
