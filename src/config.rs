use crate::{
    errors::ConfigError,
    schema::{Devices, Map},
};
use serde_derive::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    devices: Devices,
    mappings: Mappings,
}

#[derive(Deserialize, Debug)]
struct Mappings {
    maps: Vec<Map>,
}

pub fn read_config_file(path: &Path) -> Result<Config, ConfigError> {
    let binding = match fs::read_to_string(path) {
        Ok(b) => Ok(b),
        Err(_) => Err(ConfigError::ReadError(format!(
            "Config file not found at {:?}",
            path.as_os_str()
        ))),
    }?;
    let content = binding.as_str();
    let config: Config = toml::from_str(content)?;
    Ok(config)
}
