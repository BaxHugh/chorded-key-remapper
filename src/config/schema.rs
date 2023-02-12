use crate::mapping::Map;
use serde_derive::Deserialize;

fn empty<T>() -> Option<T> {
    None
}

#[derive(Deserialize, Debug)]
pub struct DevicesConfig {
    #[serde(default = "empty")]
    include: Option<Vec<String>>,
    #[serde(default = "empty")]
    omit: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    devices: Option<DevicesConfig>,
    #[serde(default)]
    mappings: Option<MappingsConfig>,
}

#[derive(Deserialize, Debug)]
pub struct MappingsConfig {
    maps: Option<Vec<Map>>,
}

impl Default for DevicesConfig {
    fn default() -> Self {
        Self {
            include: None,
            omit: None,
        }
    }
}

impl Default for MappingsConfig {
    fn default() -> Self {
        Self { maps: None }
    }
}
