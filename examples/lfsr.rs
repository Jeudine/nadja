// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::{concat, Logic, VLogic};
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, Signal, Simulator, Wire};

#[macro_use] extern crate derive_new;
#[macro_use] extern crate nadja_derive;

#[derive(new)]
struct LFSRComb<'a> {
    state_o: &'a dyn Channel<VLogic<20>>,
}

impl<'a> Channel<VLogic<20>> for LFSRComb<'a> {
    fn read(&self) -> VLogic<20> {
        let state_o = self.state_o.read();
        concat(
            VLogic::new([state_o[19] ^ state_o[16]]),
            state_o.sub::<0, 19>(),
        )
    }
}

#[channel]
fn test(logic1_i: Logic, logic2_i: Logic) {
    logic1_i & logic2_i
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
