use config::{Config, File};
use serde::{Deserialize, Serialize};

/// A configuration module to handle details of pre-configured cluster
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RaexConfig {
    /// URL of Local cache.
    local: String,
    /// URL of Global data-store.
    global: String,
}

impl RaexConfig {
    pub fn load(path: &str) -> Self {
        let mut settings = Config::default();
        // The target file to load configuration from is raex.toml
        if let Err(e) = settings.merge(File::with_name(path)) {
            eprintln!("Error: {}", e);
            panic!("Can't configure node.")
        } else {
            let local = settings.get::<String>("local").unwrap();
            let global = settings.get::<String>("global").unwrap();
            Self { local, global }
        }
    }

    pub fn local(&self) -> &String {
        &self.local
    }
    pub fn global(&self) -> &String {
        &self.global
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_config_test() {
        let loaded_config = RaexConfig::load("example/raex");
        let expected = RaexConfig {
            local: "0.0.0.0".to_string(),
            global: "0.0.0.0".to_string(),
        };
        assert_eq!(loaded_config, expected);
    }
}
