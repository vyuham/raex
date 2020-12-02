use config::{Config, File};
use serde::{Serialize, Deserialize};

/// A configuration module to handle details of pre-configured cluster
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
        if let Err(e) = settings.merge(File::with_name("raex")) {
            eprintln!("Error: {}", e);
            panic!("Can't configure node.")
        } else {
            let nodes: Vec<String> = settings.get::<Vec<String>>("nodes").unwrap();
            let global: String = settings.get::<String>("global").unwrap();
            Self { nodes, global }
        }
    }

    pub fn nodes(self) -> Vec<String> { self.nodes }
    pub fn global(self) -> String { self.global }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_config_test() {
        let loaded_config = RaexConfig::load();
        let expected = RaexConfig {
            nodes: vec!["0.0.0.0".to_string()], 
            global: "0.0.0.0".to_string()
        };
        assert_eq!(loaded_config, expected);
    }
}