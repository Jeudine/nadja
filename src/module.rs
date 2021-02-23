use crate::process;

pub struct Module {
    processes: Vec<&'static dyn process::Process>,
}
