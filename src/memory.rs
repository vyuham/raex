use dstore::local::Local;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::config::RaexConfig;

pub struct Memory {
    local: Arc<Mutex<Local>>,
}

impl Memory {
    async fn init(cfg: RaexConfig) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            local: Local::new(cfg.global().clone(), cfg.local().clone()).await?,
        })
    }
}
