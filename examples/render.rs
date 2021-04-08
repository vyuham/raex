use raex::{RaEx, Scheduler};
use std::collections::{HashMap, VecDeque};

struct ExecUnit {}

struct RenderSched {
    schedule: VecDeque<ExecUnit>,
    executing: HashMap<i8, ExecUnit>,
}

impl RenderSched {
    fn new() -> Self {
        Self {
            schedule: VecDeque::new(),
            executing: HashMap::new(),
        }
    }
}

impl Scheduler for RenderSched {
    fn next(&mut self) {}

    fn execute(&mut self) {}
}

#[tokio::main]
async fn main() {
    RaEx::start("/tmp/raex", Box::new(RenderSched::new())).await;
}
