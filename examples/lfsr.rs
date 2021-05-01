// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, Input, Output, Signal, Simulator, Wire};
//TODO simplify macro
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate nadja_derive;
#[macro_use]
extern crate mashup;

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

#[out]
fn LFSR() {
    state_o = sig.state_q;
}

#[comb]
fn LFSR() {
    state_d = CFunc(&sig.state_q);
}

#[proc]
fn LFSR() {
    state_q = RegRst(&comb.state_d, &sig.state_q, input.rst_ni, &input.INIT_STATE);
}

fn main() {
    //parameter
    let INIT_STATE = concat(
        VLogic::new([Logic::Logic1; 1]),
        VLogic::new([Logic::Logic0; WIDTH - 1]),
    );

    //input
    let rst_ni: Wire<bool> = Default::default();

    //instance
    LFSR!(i_LFSR {
        INIT_STATE: INIT_STATE,
        rst_ni: &rst_ni,
    });

    // clk & rst
    let clk = Clk::new(1, &[&i_LFSR.p.state_q], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&i_LFSR.p.state_q]);

    //simulation
    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();

    //output
    println!("{:?}", i_LFSR.o.state_o);
}
