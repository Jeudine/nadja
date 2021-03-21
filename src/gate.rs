use crate::interface::{TChannel ,Channel};
use std::fmt::Debug;
use std::ops::Not as BitNot;
use std::ops::{BitAnd, BitOr, BitXor};

struct And<'a, T: TChannel + BitAnd + BitAnd<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Or<'a, T: TChannel + BitOr + BitOr<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Xor<'a, T: TChannel + BitXor + BitXor<Output = T>> {
    in1: &'a dyn Channel<T>,
    in2: &'a dyn Channel<T>,
}

struct Not<'a, T: Copy + PartialEq + Debug + BitNot + BitNot<Output = T>> {
    in1: &'a dyn Channel<T>,
}

impl<'a, T> Channel<T> for And<'a, T>
where
    T: TChannel + BitAnd + BitAnd<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() & self.in2.read()
    }
}

impl<'a, T> Channel<T> for Or<'a, T>
where
    T: TChannel + BitOr + BitOr<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() | self.in2.read()
    }
}

impl<'a, T> Channel<T> for Xor<'a, T>
where
    T: TChannel + BitXor + BitXor<Output = T>,
{
    fn read(&self) -> T {
        self.in1.read() ^ self.in2.read()
    }
}

impl<'a, T> Channel<T> for Not<'a, T>
where
    T: TChannel + BitNot + BitNot<Output = T>,
{
    fn read(&self) -> T {
        !self.in1.read()
    }
}
