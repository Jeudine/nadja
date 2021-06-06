use super::{Process, ProcessRes};
use crate::interface::{Simulable, TValue};
use crate::logic::{Logic, VLogic};
use crate::Channel;
use crate::Signal;
use crate::Simulator;

#[derive(new)]
pub struct Reg<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<T>,
}

impl<'a, T: TValue> Process<'a> for Reg<'a, T> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        self.q.write(self.d.read(), simulator);
        ProcessRes::End
    }
}

#[derive(new)]
pub struct RegRst<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    rst_n: &'a dyn Channel<bool>,
    init_state: &'a T,
    q: &'a Signal<T>,
}

impl<'a, T: TValue> Process<'a> for RegRst<'a, T> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        if self.rst_n.read() {
            self.q.write(self.d.read(), simulator);
        } else {
            self.q.write(*self.init_state, simulator);
        }
        ProcessRes::End
    }
}

pub struct FF<'a, const WIDTH: usize> {
    d: &'a dyn Channel<VLogic<WIDTH>>,
    rst_n: &'a dyn Channel<Logic>,
    init_state: VLogic<WIDTH>,
    q: &'a Signal<VLogic<WIDTH>>,
}

impl<'a, const WIDTH: usize> Process<'a> for FF<'a, WIDTH> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        if self.rst_n.read() == Logic::Logic1 {
            self.q.write(self.d.read(), simulator);
        } else {
            self.q.write(self.init_state, simulator);
        }
        ProcessRes::End
    }
}

impl<'a, const WIDTH: usize> FF<'a, WIDTH> {
    pub fn new(
        d: &'a dyn Channel<VLogic<WIDTH>>,
        rst_n: &'a dyn Channel<Logic>,
        init_state: usize,
        q: &'a Signal<VLogic<WIDTH>>,
    ) -> Self {
        Self {
            d: d,
            rst_n: rst_n,
            init_state: VLogic::from(init_state),
            q: q,
        }
    }
}
