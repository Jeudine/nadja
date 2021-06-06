// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RstLogic};
use nadja::{Channel, HaveProc, Simulator, Wire};

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

#[seq]
mod lfsr {
    use super::{CFunc, WIDTH};

    struct io {
        rst_ni: In<Logic>,
        state_o: Out<VLogic<WIDTH>>,
    }

    fn core() {
        let state_q: WIDTH = FF(state_d, rst_ni, 1);
        let state_d = CFunc { state_i: state_q };
        Output { state_o: state_q };
    }
}

fn main() {
    //input
    let rst_ni: Wire<Logic> = Default::default();

    //instance
    lfsr!(i_lfsr { rst_ni: rst_ni });
    // clk & rst
    let clk = Clk::new(1, &i_lfsr.get_procs(), &[]);
    let rst_n_proc = RstLogic::new(&rst_ni, false, 2, 2, &i_lfsr.get_procs());

    //simulation
    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();

    //output
    println!("{:?}", i_lfsr.state_o);
}
