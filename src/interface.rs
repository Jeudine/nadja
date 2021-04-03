use super::simulator::Simulator;
use super::trace::Trace;
use crate::Channel;
use std::fmt::{Debug, Formatter, Result};

pub trait TChannel: Copy + PartialEq + Debug {}

impl<T: TChannel> Debug for dyn Channel<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.read().fmt(f)
    }
}

pub trait Event<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>];
}

pub trait TValue: TChannel + Default + Trace {}

pub trait Simulable<'a, T: TValue>: Default + Channel<T> + Event<'a> {
    fn new(val: T) -> Self;
    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T;
    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T;
}

pub trait SimulableTrig<'a, T: TValue>: Default + Channel<T> + Event<'a> {
    fn new(val: T, sensitivity: &[&'a dyn Process<'a>]) -> Self;
    fn write(&'a self, val: T, simulator: &mut Simulator<'a>) -> T;
    fn update(&'a self, f: &dyn Fn(T) -> T, simulator: &mut Simulator<'a>) -> T;
}

impl TChannel for bool {}
impl TValue for bool {}

/// Executes the process until the end or a break.
/// In case the execution stops on a break, returns the duration of the break, otherwise return
/// None.
pub trait Process<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize>;
}
