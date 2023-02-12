use crate::device::Key;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Map {
    input: Vec<Key>,
    output: Vec<Key>,
}
