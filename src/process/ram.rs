use super::{Process, ProcessRes};
use crate::logic::{Logic, VLogic};
use crate::util::log2;
use crate::{Channel, Simulator};

pub struct RamSp<'a, const WIDTH: usize, const DEPTH: usize>
where
    VLogic<{ log2(DEPTH) }>: Sized,
{
    mem: [VLogic<WIDTH>; DEPTH],
    wr_en: &'a dyn Channel<Logic>,
    addr: &'a dyn Channel<VLogic<{ log2(DEPTH) }>>,
    data_i: &'a dyn Channel<VLogic<WIDTH>>,
    data_o: &'a dyn Channel<VLogic<WIDTH>>,
}

impl<'a, const WIDTH: usize, const DEPTH: usize> Process<'a> for RamSp<'a, WIDTH, DEPTH>
where
    VLogic<{ log2(DEPTH) }>: Sized,
{
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        if self.wr_en.read() == Logic::Logic1 {
            self.mem[addr
        }
        self.data_o.write(
        ProcessRes::End
    }
}
