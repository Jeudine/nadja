use super::{Process, ProcessRes};
use crate::interface::Simulable;
use crate::logic::{Logic, VLogic};
use crate::{Channel, Signal, Simulator};
use std::cell::Cell;

pub struct RamSp<'a, const DATA_WIDTH: usize, const ADDR_WIDTH: usize, const DEPTH: usize> {
    mem: [Cell<VLogic<DATA_WIDTH>>; DEPTH],
    addr: &'a dyn Channel<VLogic<ADDR_WIDTH>>,
    data: &'a dyn Channel<VLogic<DATA_WIDTH>>,
    we: &'a dyn Channel<Logic>,
    q: &'a Signal<VLogic<DATA_WIDTH>>,
}

impl<'a, const DATA_WIDTH: usize, const ADDR_WIDTH: usize, const DEPTH: usize> Process<'a>
    for RamSp<'a, DATA_WIDTH, ADDR_WIDTH, DEPTH>
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

impl<'a, const DATA_WIDTH: usize, const ADDR_WIDTH: usize, const DEPTH: usize>
    RamSp<'a, DATA_WIDTH, ADDR_WIDTH, DEPTH>
{
    pub fn new(
        addr: &'a dyn Channel<VLogic<ADDR_WIDTH>>,
        data: &'a dyn Channel<VLogic<DATA_WIDTH>>,
        we: &'a dyn Channel<Logic>,
        q: &'a Signal<VLogic<DATA_WIDTH>>,
    ) -> Self {
        let val = VLogic::new([Logic::Logicx; DATA_WIDTH]);
        Self {
            mem: unsafe {
                let mut mem: [Cell<VLogic<DATA_WIDTH>>; DEPTH] =
                    std::mem::MaybeUninit::uninit().assume_init();
                for i in &mut mem[..] {
                    std::ptr::write(i, Cell::new(val));
                }
                mem
            },
            addr: addr,
            data: data,
            we: we,
            q: q,
        }
    }

    pub fn init(&self, mem: &[usize; DEPTH]) {
        self.mem
            .iter()
            .zip(mem.iter())
            .for_each(|(x, y)| x.set((*y).into()));
    }
}
