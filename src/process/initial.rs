use crate::process;
use crate::simulator;
use crate::module;

//TODO need to use thread in order to implement wait()
pub struct Initial {
    function: fn(&mut simulator::Simulator),
}

impl process::Process for Initial {
    fn execute(&self, module: &mut module::Module, simulator: &mut simulator::Simulator) -> Option<usize> {
        (self.function)(simulator);
        Option::None
    }
}
