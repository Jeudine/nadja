use super::process::Process;
use super::simulator::Simulator;
use super::trace::Trace;
use std::fmt::{Debug, Formatter, Result};

pub trait TChannel: Copy + PartialEq + Debug {}

pub trait Channel<T: TChannel > {
    fn read(&self) -> T;
}

impl<T: TChannel> Debug for dyn Channel<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.read().fmt(f)
    }
}

pub trait Event<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>];
}

pub trait TValue: TChannel + Default + Trace {}

pub trait Simulable<'a, T: TValue>:
    Default + Channel<T> + Event<'a>
{
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self;
    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T;
    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T;
}

impl TChannel for bool {}
impl TValue for bool {}
