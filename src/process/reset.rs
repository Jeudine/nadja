use super::{Process, ProcessRes};
use crate::interface::Event;
use crate::logic::Logic;
use crate::{Simulator, Wire};
use std::cell::Cell;

pub struct RstBool<'a> {
    rst: &'a Wire<bool>,
    sensitivity: Vec<&'a dyn Process<'a>>,
    duration: Cell<Option<usize>>,
    date: Cell<Option<usize>>,
}

impl<'a> RstBool<'a> {
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

impl<'a> Process<'a> for RstBool<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> ProcessRes {
        let date = self.date.get();
        match date {
            Some(x) => {
                self.date.set(None);
                ProcessRes::Break(x)
            }
            None => {
                self.rst.update(&|x| !x);
                let duration = self.duration.get();
                match duration {
                    Some(x) => {
                        simulator.push(self);
                        self.duration.set(None);
                        ProcessRes::Break(x)
                    }
                    None => ProcessRes::End,
                }
            }
        }
    }
}

impl<'a> Event<'a> for RstBool<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}

pub struct RstLogic<'a> {
    rst: &'a Wire<Logic>,
    sensitivity: Vec<&'a dyn Process<'a>>,
    duration: Cell<Option<usize>>,
    date: Cell<Option<usize>>,
}

impl<'a> RstLogic<'a> {
    pub fn new(
        wire: &'a Wire<Logic>,
        active_edge: bool,
        date: usize,
        duration: usize,
        sensitivity: &[&'a dyn Process<'a>],
    ) -> Self {
        wire.write(Logic::from(!active_edge));
        Self {
            rst: wire,
            sensitivity: sensitivity.to_vec(),
            duration: Cell::new(Some(duration)),
            date: Cell::new(Some(date)),
        }
    }
}

impl<'a> Process<'a> for RstLogic<'a> {
    fn execute(&'a self, simulator: &mut Simulator<'a>) -> ProcessRes {
        let date = self.date.get();
        match date {
            Some(x) => {
                self.date.set(None);
                ProcessRes::Break(x)
            }
            None => {
                self.rst.update(&|x| !x);
                let duration = self.duration.get();
                match duration {
                    Some(x) => {
                        simulator.push(self);
                        self.duration.set(None);
                        ProcessRes::Break(x)
                    }
                    None => ProcessRes::End,
                }
            }
        }
    }
}

impl<'a> Event<'a> for RstLogic<'a> {
    fn trigger(&self) -> &[&dyn Process<'a>] {
        &self.sensitivity[..]
    }
}
