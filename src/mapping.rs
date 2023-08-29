use crate::Key;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]

pub struct HyperkeyGroupMap {
    pub hyper: Key,
    pub maps: Vec<OneToOneMap>,
}

#[derive(Deserialize, Debug)]
pub struct OneToOneMap {
    pub input: Key,
    pub output: Key,
}
