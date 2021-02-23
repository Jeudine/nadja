use crate::signal;
use crate::process::always;
use crate::process;
use crate::simulator;

pub struct Clock {
    clock_signal: signal::Signal<bool>,
    clock_process: always::Always,
    posedge_sensitivity: Vec<&'static dyn process::Process>,
    negedge_sensitivity: Vec<&'static dyn process::Process>,
    function: fn(&mut simulator::Simulator),
}

impl Clock {
}
impl Default for Clock {
    fn default() -> Self {
        clock_process: self.function
    }
}
