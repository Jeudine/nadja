use crate::interface::Channel;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr};

struct And<'a, T: Copy + PartialEq + Display + BitAnd + BitAnd<Output = T>> {
    in1: &'a dyn Channel<'a, T>,
    in2: &'a dyn Channel<'a, T>,
}

struct Or<'a, T: Copy + PartialEq + Display + BitOr + BitOr<Output = T>> {
    in1: &'a dyn Channel<'a, T>,
    in2: &'a dyn Channel<'a, T>,
}

impl<'a, T> Channel<'a, T> for And<'a, T>
where
    T: Copy + PartialEq + Display + BitAnd + BitAnd<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() & self.in2.read()
    }
}

impl<'a, T> Channel<'a, T> for Or<'a, T>
where
    T: Copy + PartialEq + Display + BitOr + BitOr<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() | self.in2.read()
    }
}
