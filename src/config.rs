use config::{Config, File};
use serde::{Serialize, Deserialize};

/// A configuration module to handle details of pre-configured cluster
#[derive(Debug, Serialize, Deserialize)]
pub struct RaexConfig {
    /// URL values used in locating nodes, used in GRPC comm.
    nodes: Vec<String>,
    /// URL of Global data-store.
    global: String
}

impl RaexConfig {
    pub fn load() -> Self {
        let mut settings = Config::default();
        // The target file to load configuration from is raex.toml
        settings.merge(File::with_name("raex"));
        Self {
            nodes: settings.get("nodes").unwrap(),
            global: settings.get("global").unwrap()
        }
    }

    pub fn nodes(self) -> Vec<String> { self.nodes }
    pub fn global(self) -> String { self.global }
}

