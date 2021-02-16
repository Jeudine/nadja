use crate::process;
use crate::signal;

pub struct Module {
    processes: Vec<&'static process::Process>,
    //signals: Vec<&'static signal::Signal>,
}
