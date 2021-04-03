// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::logic::concat;
use nadja::logic::Logic;
use nadja::logic::VLogic;
use nadja::process::{Clk, RegRst, Rst};
use nadja::{Channel, Signal, Simulator, Wire};

#[macro_use]
extern crate derive_new;

#[derive(new)]
struct LFSRComb<'a> {
    state_o: &'a dyn Channel<VLogic<20>>,
}

impl<'a> Channel<VLogic<20>> for LFSRComb<'a> {
    fn read(&self) -> VLogic<20> {
        let state_o = self.state_o.read();
        //println!("output: {:?}", state_o);
        //TODO for concat do a macro no matter if it's a logic or a VLogic
        concat(
            VLogic::new([state_o[19] ^ state_o[16]]),
            state_o.sub::<0, 19>(),
        )
    }
}

fn main() {
    let init_state_i: Wire<VLogic<20>> = Wire::new(concat(
        VLogic::new([Logic::Logic1; 1]),
        VLogic::new([Logic::Logic0; 19]),
    ));
    let state_o: Signal<VLogic<20>> = Default::default();
    let rst_ni: Wire<bool> = Default::default();

    let lfsr_comb = LFSRComb::new(&state_o);
    let reg = RegRst::new(&lfsr_comb, &state_o, &rst_ni, &init_state_i);

    let clk = Clk::new(1, &[&reg], &[]);
    let rst_n_proc = Rst::new(&rst_ni, false, 2, 2, &[&reg]);

    let mut sim = Simulator::new(2097154, &[&clk, &rst_n_proc]);
    sim.run();
    println!("{:?}", state_o);
}
