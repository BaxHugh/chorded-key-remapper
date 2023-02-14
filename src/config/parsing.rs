use super::schema::{Config, DevicesConfig};
use crate::auxiliary::device_filtering::FilterableDevices;
use crate::auxiliary::utils::format_strings;
use crate::device::DeviceInfo;
use crate::errors::ConfigError;
use crate::errors::DeviceError;

use log::log_enabled;
use std::{fs, path::Path};
use toml;

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

impl DevicesConfig {
    pub fn extract_devices_to_remap<T: DeviceInfo>(
        self,
        all_devices: Vec<T>,
    ) -> Result<Vec<T>, DeviceError> {
        let keyboards = match self.include {
            None => match all_devices.extract_devices_whose_name_doesnt_contain("virtual") {
                None => Err(DeviceError::DevicesNotFound(format!(
                    "No non-virtual devices fond in existing devices."
                ))),

                Some(non_virtual_devs) => match non_virtual_devs.extract_keyboards() {
                    None => Err(DeviceError::DevicesNotFound(format!(
                        "No non-virtual keyboards found in existing devices."
                    ))),
                    Some(non_virtual_keyboards) => Ok(non_virtual_keyboards),
                },
            },

            Some(include_names) => match all_devices.extract_named_devices(&include_names) {
                None => Err(DeviceError::DevicesNotFound(format!(
                    "No devices found which match names: {}",
                    format_strings(include_names)
                ))),

                Some(devices) => Ok(devices),
            },
        }?;

        match self.exclude {
            None => Ok(keyboards),

            Some(exclude_names) => match keyboards.remove_named_devices(&exclude_names) {
                Some(keyboards) => {
                    if log_enabled!(log::Level::Info) {
                        if keyboards.len() != exclude_names.len() {
                            let found: Vec<&str> = keyboards
                                .iter()
                                .map(|dev| match dev.name() {
                                    Some(name) => name,
                                    None => "UNNAMED",
                                })
                                .collect();

                            let missing: Vec<String> = exclude_names
                                .iter()
                                .filter(|name| !found.contains(&name.as_str()))
                                .map(|name| name.to_owned())
                                .collect();

                            log::info!(
                                "Not all named devices where found. Couldn't find: {}",
                                format_strings(missing)
                            );
                        }
                    }
                    Ok(keyboards)
                }

                None => Err(DeviceError::DevicesNotFound(format!(
                    "No devices left after filtering out excluded devices: {}",
                    format_strings(exclude_names)
                ))),
            },
        }
    }
}
