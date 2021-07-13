// A test of Single-Port ram

use nadja::{Simulator, Signal, Wire, logic::{VLogic, Logic}, process::{Process, ProcessRes, RamSp}};
use std::cell::Cell;

const W: usize = 32;
const D: usize = 256;
const P: usize = 2;

fn main() {
    let ramTester = RamTester::default();
    let ram: RamSp<W, W, D> = RamSp::new(&ramTester.addr, &ramTester.data, &ramTester.we, &ramTester.q);
}

#[derive(Default)]
struct RamTester {
    addr: Wire<VLogic<W>>,
    data: Wire<VLogic<W>>,
    we: Wire<Logic>,
    q: Signal<VLogic<W>>,

    counter: Cell<u32>,
}

impl<'a> Process<'a> for RamTester {
    fn execute(&'a self, _: &mut Simulator<'a>) -> ProcessRes {
        let counter = self.counter.get();
        match counter {
            0 => {
                self.addr.write(VLogic::from(0));
                self.data.write(VLogic::from(0));
                self.we.write(Logic::from(1));
                self. counter.set(1);
                ProcessRes::Break(P)
            }
            1 => {
                self.addr.write(VLogic::from(0));
                self.data.write(VLogic::from(0));
                self.we.write(Logic::from(0));
                self. counter.set(2);
                ProcessRes::Break(P)
            }
            _ => ProcessRes::Stop
        }
    }
}
