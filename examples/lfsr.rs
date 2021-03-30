// An implementation of a 20-bit Fibonacci LFSR with the following taps [20, 17]

//TODO simplify the use
use nadja::simulator::Simulator;
use nadja::logic::VLogic;
use nadja::logic::Logic;
use nadja::process::RegRst;
use nadja::process::Process;
use nadja::module::Module;
use nadja::gate::Xor;
use nadja::interface::Channel;

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

impl Channel<VLogic<20>> for LFSR
{
    fn read(&self) -> VLogic<20> {
        let res = VLogic::new([self.state_o[19] ^ self.state_o[16];20]);
        for i in 1..20 {
            res[i] = self.state_o[i-1];
        }
        res
    }

}

fn main() {
    let i_lfsr: LFSR = Default::default();
    let comb: Xor<Logic>;
    let reg = RegRst::new(i_lfsr.);
}
