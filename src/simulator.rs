use super::process;

struct Simulator<'a> {
    modified: Vec<&'a mut dyn Update>,
    processes: Vec<&'a dyn process::Process>,
}

impl<'a> Simulator<'a> {
    fn update(&mut self) {
        //TODO: See if sensivity list is triggered when updating (use the result)
        self.modified.iter_mut().map(|x| x.update()); //return a vec of bool
                                                      // also see filter and for_each
    }

    fn push(&mut self, u: &'a mut dyn Update) {
        self.modified.push(u);
    }
}

pub trait Update {
    fn update(&mut self) -> Option<&[&dyn process::Process]>;
}
