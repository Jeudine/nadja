use crate::interface::Channel;
use crate::interface::{Simulable, TValue};
use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::Simulator;

#[derive(new)]
pub struct Reg<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
}

impl<'a, T: TValue> Process<'a> for Reg<'a, T>
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        self.q.write(self.d.read(), simulator);
        None
    }
}

#[derive(new)]
pub struct RegRst<'a, T: TValue> {
    d: &'a dyn Channel<T>,
    q: &'a Signal<'a, T>,
    nrst: &'a dyn Channel<bool>,
    init_state: &'a dyn Channel<T>,
}

impl<'a, T: TValue> Process<'a> for RegRst<'a, T>
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> Option<usize> {
        if self.nrst.read() {
            self.q.write(self.d.read(), simulator);
        } else {
            self.q.write(self.init_state.read(), simulator);
        }
        None
    }
}
