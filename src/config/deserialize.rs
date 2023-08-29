// Implement Deserialize for structs used elsewhere in the crate:
// i.e. Map and Key, so that they can loaded from config.

use std::fmt::Debug;
use std::str::FromStr;

use crate::device::Key;
use crate::errors::ConfigError;
