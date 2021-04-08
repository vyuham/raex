//! RaEx is a tool to help you build high performance compute clusters, with which you can run
//! computational tasks that would otherwise be incredibly inefficient on a single system.

mod scheduler;

use config::{Config, ConfigError, File};
use dstore::Local;
pub use scheduler::Scheduler;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
pub struct RaExConfig {
    pub global_addr: String,
    pub local_addr: String,
}

impl RaExConfig {
    pub fn new(file_name: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name(file_name))?;

        config.try_into()
    }
}

pub struct RaEx {
    store: Arc<Mutex<Local>>,
    config: RaExConfig,
    scheduler: Box<dyn Scheduler>,
}

impl RaEx {
    pub async fn start(config: &str, scheduler: Box<dyn Scheduler>) -> Self {
        let config = RaExConfig::new(config).unwrap();
        Self {
            config: config.clone(),
            store: Local::new(config.global_addr, config.local_addr)
                .await
                .unwrap(),
            scheduler,
        }
    }
}
