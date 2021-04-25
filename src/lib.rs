//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.

#[macro_use]
extern crate async_trait;

mod scheduler;

use config::{Config, ConfigError, File};
use dstore::Local;
pub use raft::RaftNode;
pub use scheduler::Scheduler;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
pub struct RaExConfig {
    pub global_addr: String,
    pub local_addr: String,
    pub nodes: Vec<String>,
}

impl RaExConfig {
    pub fn new(file_name: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name(file_name))?;

        config.try_into()
    }
}

pub struct RaEx<T> {
    store: Arc<Mutex<Local>>,
    config: Arc<RaExConfig>,
    scheduler: Box<dyn Scheduler<T>>,
    running: Option<T>,
}

impl<T: 'static> RaEx<T> {
    pub async fn start(config: Arc<RaExConfig>, scheduler: Box<dyn Scheduler<T>>) -> Self {
        let global_addr = config.global_addr.clone();
        let local_addr = config.local_addr.clone();
        Self {
            config,
            store: Local::new(global_addr, local_addr).await.unwrap(),
            scheduler,
            running: None,
        }
    }

    pub async fn run(&mut self) {
        loop {
            if let None = self.running {
                self.scheduler.next().await;
                self.scheduler.execute().await;

                self.running = Some(self.scheduler.current().await);
            }
        }
    }
}
