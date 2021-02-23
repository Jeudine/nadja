use crate::process;
use crate::simulator;

//TODO need to use thread in order to implement wait()
pub struct Initial {
    function: fn(&mut simulator::Simulator),
}

impl process::Process for Initial {
    fn execute(&self, simulator: &mut simulator::Simulator) -> Option<usize> {
        (self.function)(simulator);
        Option::None
    }
}
