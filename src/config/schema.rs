use crate::mapping::Map;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Devices {
    include: Vec<String>,
    omit: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    devices: Devices,
    mappings: Mappings,
}

#[derive(Deserialize, Debug)]
pub struct Mappings {
    maps: Vec<Map>,
}
