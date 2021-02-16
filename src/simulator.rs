use super::process;

pub struct Simulator<'a> {
    modified: Vec<&'a mut dyn Update>,
    processes: Vec<&'static process::Process>,
}

impl<'a> Simulator<'a> {
    fn update(&mut self) {
        let processes = &mut self.processes;
        self.modified
            .iter_mut()
            .map(|x| x.update())
            .for_each(|x| match x {
                Some(p) => {
                    let p_f: Vec<_> = p
                        .iter()
                        .filter(|x| processes.iter().all(|y| !std::ptr::eq(*x, y)))
                        .collect();
                    processes.extend(p_f)
                }
                None => (),
            });
        self.modified = Vec::new();
    }

    pub fn push(&mut self, u: &'a mut dyn Update) {
        self.modified.push(u);
    }

    fn execute(&mut self) {
        let processes = self.processes.clone();
        processes.iter().for_each(|x| x.execute(self));
    }
}

pub trait Update {
    fn update(&mut self) -> Option<&[&'static process::Process]>;
}
