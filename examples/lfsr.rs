// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::gate::Xor;
use nadja::interface::Channel;
use nadja::logic::Logic;
use nadja::logic::VLogic;
use nadja::logic::concat;
use nadja::module::Module;
use nadja::process::Process;
use nadja::process::RegRst;
use nadja::simulator::Simulator;

#[derive(Default)]
struct LFSR {
    //Input
    pub init_state_i: VLogic<20>,
    pub rst_ni: bool,

    //Output
    pub state_o: VLogic<20>,
    //Comb

    //Process
}

impl Channel<VLogic<20>> for LFSR {
    fn read(&self) -> VLogic<20> {
        concat(
    }
}

fn main() {
    let i_lfsr: LFSR = Default::default();
    let comb: Xor<Logic>;
    //let reg = RegRst::new(i_lfsr.);
}
