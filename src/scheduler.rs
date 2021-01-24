use crate::executor::Exec;
use std::collections::{HashMap, VecDeque};

/// Maintains schedule within raft consensus.
pub struct Sched {
    /// A queue of processes waiting to be executed.
    scheduled: VecDeque<Exec>,
    /// A list of processes being executed.
    executing: HashMap<u8, Option<Exec>>,
    /// A log of all processes that have finished executing.
    completed: Vec<Exec>,
}

impl Sched {
    pub fn default() -> Self {
        Self {
            scheduled: VecDeque::new(),
            executing: HashMap::new(),
            completed: vec![],
        }
    }

    pub fn schedule(&mut self, exec: Exec) {
        self.scheduled.push_back(exec);
    }

    pub fn add_executor_node(&mut self) -> u8 {
        let mut node_id: u8 = 0;
        for i in 0..u8::MAX {
            // executing.get() -> Option<Option<Exec>>, so Some(None) implies the node exists
            if self.executing.get(&i).is_none() {
                self.executing.insert(i, None);
                node_id = i;
                break;
            }
        }
        node_id
    }

    pub fn node_status(&self) -> String {
        let mut output = "".to_string();
        for (node, exec) in self.executing.clone() {
            output.push_str(&format!(
                "Node #{}: Running process {}",
                node,
                match exec {
                    Some(exec) => exec.in_words(),
                    None => "Nothing".to_string(),
                }
            ))
        }
        output
    }

    pub fn next(&mut self) {
        for (node, exec) in self.executing.clone() {
            match (exec, self.scheduled.pop_front()) {
                (None, Some(exec)) => {
                    self.executing.insert(node, Some(exec));
                }
                (None, _) => panic!("No process scheduled"),
                _ => panic!("Couldn't find a free node"),
            }
        }
    }
}
