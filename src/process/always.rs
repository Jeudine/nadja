use crate::process;
use crate::simulator;

pub struct Always {
    function: fn(&mut simulator::Simulator),
}

impl process::Process for Always {
    fn execute(&self, simulator: &mut simulator::Simulator) -> Option<usize> {
        (self.function)(simulator);
        Option::None
    }
}
