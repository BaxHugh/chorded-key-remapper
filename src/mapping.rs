use std::default;

use crate::device::Key;
use serde_derive::Deserialize;

fn true_() -> bool {
    true
}

#[derive(Deserialize, Debug)]
pub struct Map {
    pub input: Vec<Key>,
    pub output: Vec<Key>,
    #[serde(default = "true_")]
    pub press_left_first: bool,
}
