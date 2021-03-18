use crate::interface::Channel;
use std::fmt::Display;
use std::ops::Not as BitNot;
use std::ops::{BitAnd, BitOr, BitXor};

struct And<'a, T: Copy + PartialEq + Display + BitAnd + BitAnd<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Or<'a, T: Copy + PartialEq + Display + BitOr + BitOr<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Xor<'a, T: Copy + PartialEq + Display + BitXor + BitXor<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Not<'a, T: Copy + PartialEq + Display + BitNot + BitNot<Output = T>> {
    in1: &'a dyn Channel<T>,
}

impl<'a, T> Channel<T> for And<'a, T>
where
    T: Copy + PartialEq + Display + BitAnd + BitAnd<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() & self.in2.read()
    }
}

impl<'a, T> Channel<T> for Or<'a, T>
where
    T: Copy + PartialEq + Display + BitOr + BitOr<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() | self.in2.read()
    }
}

impl<'a, T> Channel<T> for Xor<'a, T>
where
    T: Copy + PartialEq + Display + BitXor + BitXor<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() ^ self.in2.read()
    }
}

impl<'a, T> Channel<T> for Not<'a, T>
where
    T: Copy + PartialEq + Display + BitNot + BitNot<Output = T>,
{
    fn read(&self) -> T {
        !self.in1.read()
    }
}
