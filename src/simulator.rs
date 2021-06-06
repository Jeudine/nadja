use crate::interface::Event;
use crate::process::{Process, ProcessRes};

pub struct Simulator<'a> {
    events: Vec<&'a dyn Event<'a>>,
    schedule: Vec<Vec<&'a dyn Process<'a>>>,
    cur_procs: Vec<&'a dyn Process<'a>>,
    next_delta_procs: Vec<&'a dyn Process<'a>>,
    duration: usize,
    date: usize,
}

/// Updates the variables in the events vector.
/// Adds the processes triggered by the updated values to the cur_procs vector.
impl<'a> Simulator<'a> {
    fn update(&mut self) {
        let processes = &mut self.cur_procs;
        self.events.iter().map(|x| x.trigger()).for_each(|x| {
            processes.extend(
                x.iter()
                    .filter(|x| processes.iter().all(|y| !std::ptr::eq(*x, y)))
                    .collect::<Vec<_>>(),
            )
        });
        self.events.clear();
    }

    pub fn push(&mut self, u: &'a dyn Event<'a>) {
        self.events.push(u);
    }

    #[inline]
    pub fn schedule_process(&mut self, p: &'a dyn Process<'a>, duration: usize) {
        if duration == 0 {
            self.next_delta_procs.push(p);
        } else if duration + self.date + 1 <= self.duration {
            self.schedule[self.duration - self.date - duration - 1].push(p);
        }
    }

    /// Appends the processes of the next delta into the cur_procs vector.
    /// Executes the processes in the cur_procs vector and empties it.
    /// Returns false if the process queue is initially empty, true otherwise.
    fn execute(&mut self) -> bool {
        self.cur_procs.append(&mut self.next_delta_procs);
        if self.cur_procs.is_empty() {
            false
        } else {
            self.cur_procs
                .clone()
                .iter()
                .map(|x| (*x, x.execute(self)))
                .collect::<Vec<_>>()
                .iter()
                .for_each(|x| match x.1 {
                    ProcessRes::End => (),
                    ProcessRes::Break(duration) => self.schedule_process(x.0, duration),
                    ProcessRes::Stop => (), //TODO
                });
            self.cur_procs.clear();
            true
        }
    }
    /// Simulator constructor
    pub fn new(duration: usize, init_processes: &[&'a dyn Process<'a>]) -> Self {
        let mut schedule = vec![Default::default(); duration];
        schedule[duration - 1] = init_processes.to_vec();
        Self {
            events: Vec::new(),
            schedule: schedule,
            cur_procs: Vec::new(),
            next_delta_procs: Vec::new(),
            duration: duration,
            date: 0,
        }
    }

    /// Runs the simuation until the end
    pub fn run(&mut self) {
        for date in 0..self.duration {
            self.date = date;
            match self.schedule.pop() {
                Some(procs) => {
                    self.cur_procs = procs;
                    while self.execute() {
                        self.update();
                    }
                }
                None => (),
            };
        }
    }

    pub fn get_date(&self) -> usize {
        self.date
    }
}
