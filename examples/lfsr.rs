// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, Input, Output, Signal, Simulator, Wire};

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate nadja_derive;

const WIDTH: usize = 20;

#[channel]
fn CFunc(state_i: VLogic<WIDTH>) -> VLogic<WIDTH> {
    concat(
        VLogic::new([state_i[WIDTH - 1] ^ state_i[16]]),
        state_i.sub::<0, { WIDTH - 1 }>(),
    )
}

#[module]
struct LFSR {
    //Parameter
    INIT_STATE: Param<VLogic<WIDTH>>,
    //Input
    rst_ni: Input<bool>,
    //Output
    state_o: Output<VLogic<WIDTH>>,
    //Channel function
    state_d: CFunc,
    //Process
    state_q: RegRst<VLogic<WIDTH>>,
}

impl<'a> LFSRComb<'a> {
    pub fn new(sig: &'a LFSRSig, rst_ni: &'a Input<bool>) -> Self {
        Self {
            rst_ni: rst_ni,
            state_o: &sig.state_q,
            state_d: CFunc::new(&sig.state_q),
        }
    }
}

impl<'a> LFSRProc<'a> {
    pub fn new(sig: &'a LFSRSig, comb: &'a LFSRComb, INIT_STATE: VLogic<20>) -> Self {
        Self {
            state_q: RegRst::new(&comb.state_d, &sig.state_q, comb.rst_ni, INIT_STATE),
        }
    }
}

fn main() {
    //parameter
    let INIT_STATE = concat(
        VLogic::new([Logic::Logic1; 1]),
        VLogic::new([Logic::Logic0; WIDTH - 1]),
    );

    //input
    let rst_ni: Wire<bool> = Default::default();

    //module
    let LFSR_i_sig = LFSRSig::default();
    let LFSR_i_comb = LFSRComb::new(&LFSR_i_sig, &rst_ni);
    let LFSR_i_proc = LFSRProc::new(&LFSR_i_sig, &LFSR_i_comb, INIT_STATE);

    //output
    let state_o = LFSR_i_comb.state_o;

    // clk & rst
    let clk = Clk::new(1, &[&LFSR_i_proc.state_q], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&LFSR_i_proc.state_q]);

    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();
    println!("{:?}", state_o.read());
    //println!("{:?}", state_o[19]);
}
