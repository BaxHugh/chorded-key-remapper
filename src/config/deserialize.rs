// Implement Deserialize for structs used elsewhere in the crate:
// i.e. Map and Key, so that they can loaded from config.

use std::fmt::Debug;
use std::str::FromStr;

use crate::device::Key;
use crate::errors::ConfigError;

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Key").field(&self.to_string()).finish()
    }
}

impl FromStr for Key {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match evdev::Key::from_str(s) {
            Ok(k) => Ok(Key::new(k)),
            Err(_) => Err(ConfigError::ParseKeyError(format!("{s}"))),
        }
    }
}

struct KeyVisitor {}

impl<'de> serde::de::Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string corresponding to a key i.e. 'KEY_A'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Key::from_str(v) {
            Ok(k) => Ok(k),
            Err(e) => Err((serde::de::Error::custom(e))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyVisitor {})
    }
}
