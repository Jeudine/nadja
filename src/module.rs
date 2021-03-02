use crate::process::Process;
use crate::signal::Signal;
use crate::simulator::{Simulator, Update};

#[derive(Default)]
pub struct Module {
    s1: Signal<bool>,
    s2: Signal<bool>,
    s3: Signal<bool>,
}

impl Module {

    /*
    pub fn new() -> Self {
        Self {

        }
    }
    */

    pub fn process<'a>(&'a self, sim: &mut Simulator<'a>) {
        self.s2.write(self.s1.read(), sim);
        self.s3.write(self.s2.read(), sim);
    }
}
