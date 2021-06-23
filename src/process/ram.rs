use super::{Process, ProcessRes};
use crate::logic::{Logic, VLogic};
use crate::{Channel, Signal, Simulator};
use crate::interface::Simulable;
use std::cell::Cell;

pub struct RamSp<'a, const DATA_WIDTH: usize, const ADDR_WIDTH: usize, const DEPTH: usize>
{
    mem: [Cell<VLogic<DATA_WIDTH>>; DEPTH],
    we: &'a dyn Channel<Logic>,
    addr: &'a dyn Channel<VLogic<{ ADDR_WIDTH }>>,
    data: &'a dyn Channel<VLogic<DATA_WIDTH>>,
    q: &'a Signal<VLogic<DATA_WIDTH>>,
}

impl<'a, const DATA_WIDTH: usize, const ADDR_WIDTH: usize, const DEPTH: usize> Process<'a> for RamSp<'a, DATA_WIDTH, ADDR_WIDTH, DEPTH>
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        let addr: Option<usize> = self.addr.read().into();
        match addr {
            Some(x) => {
                self.q.write(self.mem[x].get(), simulator);
                if self.we.read() == Logic::Logic1 {
                    self.mem[x].set(self.data.read());
                }
            }
            None => (),
        }
        ProcessRes::End
    }
}
