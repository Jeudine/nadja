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

/*
struct LFSR {
    INIT_STATE: Param<VLogic<20>>,
    state_o: Output<VLogic<20>>,
    state_d: CFunc,
    reg: RegRst<VLogic<20>>,
    state_q: Signal<VLogic<20>>,
}

{
    state_d: new(state_q),
    reg: new(state_d, state_q, rst_ni, INIT_STATE),
}
*/
#[derive(Default)]
struct LFSRSig {
    pub state_q: Signal<VLogic<20>>,
}

struct LFSRComb<'a> {
    //input
    rst_ni: &'a dyn Channel<bool>,

    //output
    pub state_o: &'a dyn Channel<VLogic<20>>,

    //channel function
    state_d: CFunc<'a>,
}

impl<'a> LFSRComb<'a> {
    pub fn new(sig: &'a LFSRSig, rst_ni: &'a dyn Channel<bool>) -> Self {
        Self {
            rst_ni: rst_ni,
            state_o: &sig.state_q,
            state_d: CFunc::new(&sig.state_q),
        }
    }
}

struct LFSRProc<'a> {
    reg: RegRst<'a, VLogic<20>>,
}

impl<'a> LFSRProc<'a> {
    pub fn new(sig: &'a LFSRSig, comb: &'a LFSRComb, INIT_STATE: VLogic<20>) -> Self {
        Self {
            reg: RegRst::new(&comb.state_d, &sig.state_q, comb.rst_ni, INIT_STATE),
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

    //module
    let LFSR_i_sig = LFSRSig::default();
    let LFSR_i_comb = LFSRComb::new(&LFSR_i_sig, &rst_ni);
    let LFSR_i_proc = LFSRProc::new(&LFSR_i_sig, &LFSR_i_comb, INIT_STATE);

    //output
    let state_o = LFSR_i_comb.state_o;

    // clk & rst
    let clk = Clk::new(1, &[&LFSR_i_proc.reg], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&LFSR_i_proc.reg]);

    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();
    println!("{:?}", state_o.read());
    //println!("{:?}", state_o[19]);
}
