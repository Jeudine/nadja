use super::simulator;

pub struct Process {
    function: fn(&mut simulator::Simulator),
}

impl Process {
    pub fn push_signals() {
    }

    pub fn execute(& self, simulator: &mut simulator::Simulator) {
        (self.function)(simulator);
    }
}
