use super::process;

pub struct Simulator<'a> {
    modified: Vec<&'a mut dyn Update>,
    queue_schedule: Vec<Vec<&'static dyn process::Process>>,
    process_queue: Vec<&'static dyn process::Process>,
    duration: usize,
}

impl<'a> Simulator<'a> {
    fn update(&mut self) {
        let processes = &mut self.process_queue;
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

    pub fn schedule_process(&mut self, p: &'static dyn process::Process, t_r: usize) {
        self.queue_schedule[t_r].push(p);
    }

    /// Executes the processes in the process queue and empties it.
    /// Returns false if the process queue is initially empty, true otherwise.
    fn execute(&mut self) -> bool {
        if self.process_queue.is_empty() {
            false
        } else {
            self.process_queue
                .clone()
                .iter()
                .map(|x| (*x, x.execute(self)))
                .collect::<Vec<_>>()
                .iter()
                .for_each(|x| match x.1 {
                    Some(duration) => self.queue_schedule[duration].push(x.0),
                    None => (),
                });
            self.process_queue = Vec::new();
            true
        }
    }

    pub fn start(&mut self, duration: usize) {
        self.process_queue = Vec::with_capacity(duration);
        self.duration = duration;
        //add the initial processes
        for time in 0..duration {
            self.process_queue = match self.queue_schedule.pop() {
                Some(queue) => queue,
                None => panic!("Unexpected error!"),
            };
            while self.execute() {
                self.update();
            }
        }
    }
}

pub trait Update {
    fn update(&mut self) -> Option<&[&'static dyn process::Process]>;
}
