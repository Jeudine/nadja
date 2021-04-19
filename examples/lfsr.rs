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
fn LFSRComb(state_i: VLogic<20>) -> VLogic<20> {
    concat(
        VLogic::new([state_i[19] ^ state_i[16]]),
        state_i.sub::<0, 19>(),
    )
}

fn main() {
    //parameter
    let init_state = concat(
        VLogic::new([Logic::Logic1; 1]),
        VLogic::new([Logic::Logic0; 19]),
    );

    //input
    let rst_ni: Wire<bool> = Default::default();

    //output
    let state_o: Signal<VLogic<20>> = Default::default();

    //comb
    let lfsr_comb = LFSRComb::new(&state_o);

    //process
    let reg = RegRst::new(&lfsr_comb, &state_o, &rst_ni, init_state);

    // clk & rst
    let clk = Clk::new(1, &[&reg], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&reg]);

    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();
    println!("{:?}", state_o);
    //println!("{:?}", state_o[19]);
}
