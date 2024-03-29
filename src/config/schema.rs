use crate::mapping::Map;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub devices: DevicesConfig,
    #[serde(default)]
    pub mappings: Option<MappingsConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DevicesConfig {
    #[serde(default = "empty")]
    pub include: Option<Vec<String>>,
    #[serde(default = "empty")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct MappingsConfig {
    pub maps: Option<Vec<Map>>,
}

impl Default for DevicesConfig {
    fn default() -> Self {
        Self {
            include: None,
            exclude: None,
        }
    }
}

impl Default for MappingsConfig {
    fn default() -> Self {
        Self { maps: None }
    }
}

fn empty<T>() -> Option<T> {
    None
}
