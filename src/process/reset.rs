use crate::interface::{Event, Process};
use crate::{Simulator, Wire};
use std::cell::Cell;

pub struct Rst<'a> {
    rst: &'a Wire<bool>,
    sensitivity: Vec<&'a dyn Process<'a>>,
    duration: Cell<Option<usize>>,
    date: Cell<Option<usize>>,
}

impl<'a> Rst<'a> {
    pub fn new(
        wire: &'a Wire<bool>,
        active_edge: bool,
        date: usize,
        duration: usize,
        sensitivity: &[&'a dyn Process<'a>],
    ) -> Self {
        wire.write(!active_edge);
        Self {
            rst: wire,
            sensitivity: sensitivity.to_vec(),
            duration: Cell::new(Some(duration)),
            date: Cell::new(Some(date)),
        }
    }
}

impl<'a> Process<'a> for Rst<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> Option<usize> {
        let date = self.date.get();
        match date {
            Some(_) => {
                self.date.set(None);
                date
            }
            None => {
                self.rst.update(&|x| !x);
                let duration = self.duration.get();
                match duration {
                    Some(_) => {
                        simulator.push(self);
                        self.duration.set(None);
                    }
                    None => (),
                }
                duration
            }
        }
    }
}

impl<'a> Event<'a> for Rst<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}
