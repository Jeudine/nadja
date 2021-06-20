use crate::logic::{Logic, VLogic};
use super::{Process, ProcessRes};
use crate::{Channel, Simulator};

pub struct RamSp<'a, const WIDTH: usize, const DEPTH: usize> {
    mem: [VLogic<WIDTH>; DEPTH],
    wr_en: &'a dyn Channel<Logic>,
    addr: &'a dyn Channel<VLogic<{usize::pow(2, 2)}>>, //TODO
    data_i: &'a dyn Channel<VLogic<WIDTH>>,
    data_o: &'a dyn Channel<VLogic<WIDTH>>,

}

impl<'a, const WIDTH: usize, const DEPTH: usize> Process<'a> for RamSp<'a, WIDTH, DEPTH> {
    fn execute(&self, simulator: &mut Simulator<'a>) -> ProcessRes {
        ProcessRes::End
    }
}
