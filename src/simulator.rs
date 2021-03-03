use crate::interface::Notify;
use crate::process::Process;

//TODO liftetime either all static or all 'a
#[derive(Default)]
pub struct Simulator<'a> {
    modified: Vec<&'a dyn Notify<'a>>,
    queue_schedule: Vec<Vec<&'a dyn Process<'a>>>,
    process_queue: Vec<&'a dyn Process<'a>>,
    duration: usize,
}

impl<'a> Simulator<'a> {
    fn update(&mut self) {
        let processes = &mut self.process_queue;
        self.modified.iter().map(|x| x.trigger()).for_each(|x| {
            processes.extend(
                x.iter()
                    .filter(|x| processes.iter().all(|y| !std::ptr::eq(*x, y)))
                    .collect::<Vec<_>>(),
            )
        });
        self.modified = Vec::new();
    }

    pub fn push(&mut self, u: &'a dyn Notify<'a>) {
        self.modified.push(u);
    }

    pub fn schedule_process(&mut self, p: &'a dyn Process<'a>, duration: usize) {
        if self.duration >= duration {
            self.queue_schedule[self.duration - duration].push(p);
        }
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
                    Some(duration) => {
                        if self.duration >= duration {
                            self.queue_schedule[self.duration - duration].push(x.0)
                        }
                    }
                    None => (),
                });
            self.process_queue = Vec::new();
            true
        }
    }

    // TODO: Prevent user to start two times the simulation
    // TODO: write a new
    pub fn start(&mut self, duration: usize, init: &[&'a dyn Process<'a>]) {
        self.queue_schedule.resize_with(duration, Default::default);
        self.duration = duration;
        self.queue_schedule.push(init.to_vec());
        //add the initial processes
        for _ in 0..duration {
            self.process_queue = match self.queue_schedule.pop() {
                Some(queue) => queue,
                None => Default::default(),
            };
            while self.execute() {
                self.update();
            }
            self.duration = self.duration - 1;
        }
    }
}
