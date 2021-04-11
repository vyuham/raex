#[macro_use]
extern crate async_trait;

use raex::{Consensus, RaEx, RaExConfig, Scheduler};
use std::sync::Arc;

struct ExecUnit {}

struct RenderState {
    consensus: Consensus<ExecUnit>,
    config: Arc<RaExConfig>,
}

impl RenderState {
    async fn new(config: Arc<RaExConfig>) -> Self {
        Self {
            consensus: Consensus::start(config.clone()).await.unwrap(),
            config,
        }
    }
}

#[async_trait]
impl<T: Send + 'static> Scheduler<T> for RenderState {
    async fn add(&mut self, next: T) {
        unimplemented!();
    }

    async fn next(&mut self) {
        unimplemented!()
    }

    async fn execute(&mut self) {
        unimplemented!()
    }

    async fn current(&mut self) -> T {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() {
    let cfg = Arc::new(RaExConfig::new("examples/raex").unwrap());
    let  mut raex: RaEx<ExecUnit> =
        RaEx::start(cfg.clone(), Box::new(RenderState::new(cfg).await)).await;

    raex.run().await;
}
