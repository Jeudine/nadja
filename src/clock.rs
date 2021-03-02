use crate::process;
use crate::signal;
use crate::simulator;
use std::cell;

/*
pub struct Clock {
    //sensitivity: Vec<&'static dyn process::Process>,
    //posedge_sensitivity: Vec<&'static dyn process::Process>,
    //negedge_sensitivity: Vec<&'static dyn process::Process>,
    clock_signal: cell::RefCell<signal::Signal<bool>>,
    half_period: usize,
}

impl Clock {
    fn new(half_period: usize, offset: usize) -> Self {
        Self {
            //sensitivity: Vec::new(),
            //posedge_sensitivity: Vec::new(),
            //negedge_sensitivity: Vec::new(),
            half_period: half_period,
            clock_signal: cell::RefCell::new(Default::default()),
        }
    }
}

impl process::Process for Clock {
    fn execute(&self, _: &mut simulator::Simulator) -> Option<usize> {
        self.clock_signal.replace(Default::default());
        Option::Some(self.half_period)
    }
}

/*
struct ClockProcess {
    function: fn(&mut simulator::Simulator),
    half_period: usize,
}

impl process::Process for ClockProcess {
    fn execute(&self, simulator: &mut simulator::Simulator) -> Option<usize> {
        (self.function)(simulator);
        Option::Some(self.half_period)
    }
}
*/

// Dans le constructor donner une reference du signal
*/
