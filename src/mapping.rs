use crate::device::Key;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Map {
    pub input: Vec<Key>,
    pub output: Vec<Key>,
}
