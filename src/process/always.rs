use crate::process;
use crate::simulator;
use crate::module;

pub struct Always {
    function: fn(& module::Module, &mut simulator::Simulator),
}

impl process::Process for Always {
    fn execute(&self, module: & module::Module, simulator: &mut simulator::Simulator) -> Option<usize> {
        (self.function)(module, simulator);
        Option::None
    }
}

impl Always {
    pub fn new(function: fn(& module::Module, &mut simulator::Simulator)) -> Self {
        Self {
            function: function,
        }

    }
}
