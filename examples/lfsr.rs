// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, In, Out, Param, Signal, Simulator, Wire};
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

#[seq]
mod lfsr {
    /*
    struct param {
        WIDTH_param: usize,
    }
    */

    const WIDTH: usize = 20;

    struct io {
        //Input
        rst_ni: In<bool>,
        //Output
        state_o: Out<VLogic<WIDTH>>,
    }

    fn core() {
        let state_q: RegRst<VLogic<WIDTH>> = RegRst(state_d, rst_ni, INIT_STATE);
        let state_d = CFunc(state_q);
        state_o(state_q);
    }
}

/*
#[module]
struct LFSR {
//Parameter
INIT_STATE: Param<VLogic<WIDTH>>,
//Input
rst_ni: Input<bool>,
//Output
state_o: Output<VLogic<WIDTH>>,
//Process
state_q: RegRst<VLogic<WIDTH>>,
}

#[comb]
fn LFSR() {
state_d = CFunc(i_sig.state_q);
}

#[proc]
fn LFSR() {
state_q = RegRst(&comb.state_d, &sig.state_q, input.rst_ni, &input.INIT_STATE);
}

#[out]
fn LFSR() {
state_o = sig.state_q;
}
*/
fn main() {
    /*
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
    */
}
