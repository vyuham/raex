use crate::executor::Exec;
use std::collections::VecDeque;

/// Maintains schedule within raft consensus.
pub struct Sched {
    /// A queue of processes waiting to be executed.
    scheduled: VecDeque<Exec>,
    /// A list of processes being executed.
    executing: Vec<Exec>,
    /// A log of all processes that have finished executing.
    completed: Vec<Exec>
}

impl Sched {
    pub fn default() -> Self {
        Self {
            scheduled: VecDeque::new(),
            executing: vec![],
            completed: vec![]
        }
    }

    pub fn schedule(&mut self, exec: Exec) {
        self.scheduled.push_back(exec);
    }

    pub fn execute(&mut self) {
        if let Some(exec) = self.scheduled.pop_front() {
            self.executing.push(exec);
        } else {
            panic!()
        }
    }
}