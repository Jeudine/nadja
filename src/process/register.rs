use crate::interface::{Process, ProcessRes, Simulable, TValue};
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
    q: &'a Signal<T>,
    nrst: &'a dyn Channel<bool>,
    init_state: T,
}

impl<'a, T: TValue> Process<'a> for RegRst<'a, T> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        if self.nrst.read() {
            self.q.write(self.d.read(), simulator);
        } else {
            self.q.write(self.init_state, simulator);
        }
        ProcessRes::End
    }
}
