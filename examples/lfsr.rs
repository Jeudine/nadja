// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, Signal, Simulator, Wire};

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate nadja_derive;

#[channel]
fn CFunc(state_i: VLogic<20>) -> VLogic<20> {
    concat(
        VLogic::new([state_i[19] ^ state_i[16]]),
        state_i.sub::<0, 19>(),
    )
}
struct LFSRIntern {}

struct LFSRComb<'a> {
    //input

    //output
    pub state_o: &'a dyn Channel<VLogic<20>>,

    //channel function
    cFunc: CFunc<'a>,

}

impl<'a> LFSRComb<'a> {
    pub fn new(intern: &'a LFSRIntern, state_o: &'a dyn Channel<VLogic<20>>) -> Self {
        Self {
            state_o: state_o,
            cFunc: CFunc::new(state_o),
        }
    }
}

struct LFSRProc<'a> {
    pub reg: RegRst<'a, VLogic<20>>,
}

impl<'a> LFSRProc<'a> {
    pub fn new(intern: &'a LFSRIntern, comb: &'a LFSRComb, INIT_STATE: VLogic<20>, rst_ni: &'a Wire<bool>, state_o: &'a Signal<VLogic<20>>) -> Self {
        Self {
            reg: RegRst::new(&comb.cFunc, state_o, rst_ni, INIT_STATE),
        }
    }
}


fn main() {
    //parameter
    let INIT_STATE = concat(
        VLogic::new([Logic::Logic1; 1]),
        VLogic::new([Logic::Logic0; 19]),
        );

    //input
    let rst_ni: Wire<bool> = Default::default();

    //output
    let state_o: Signal<VLogic<20>> = Default::default();

    //module
    let LFSR_iintern = LFSRIntern{};
    let LFSR_icomb = LFSRComb::new(&LFSR_iintern, &state_o);
    let LFSR_iproc = LFSRProc::new(&LFSR_iintern, &LFSR_icomb, INIT_STATE, &rst_ni, &state_o);

    // clk & rst
    let clk = Clk::new(1, &[&LFSR_iproc.reg], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&LFSR_iproc.reg]);

    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();
    println!("{:?}", state_o);
    //println!("{:?}", state_o[19]);
}
