use super::schema::{Config, DevicesConfig};
use crate::device::{device_getting as dg, Device, DeviceInfo};
use crate::errors::ConfigError;
use crate::errors::DeviceError;
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

fn get_all_devices() -> Result<Vec<Device>, DeviceError> {
    match dg::get_all_devices() {
        Some(devices) => Ok(devices),
        None => Err(DeviceError::DevicesNotFound(format!(
            "No devices found, make sure the program is running under sudo privileges."
        ))),
    }
}

fn filter_out_virtual_devices<T: DeviceInfo>(devices: Vec<T>) -> Result<Vec<T>, DeviceError> {
    let devices: Vec<T> = devices
        .into_iter()
        .filter(|device| !device.to_string().to_lowercase().contains("virtual"))
        .collect();
    match devices.len() {
        0 => Err(DeviceError::DevicesNotFound(format!(
            "No non-virtual devices found."
        ))),
        _ => Ok(devices),
    }
}

fn filter_keyboards<T: DeviceInfo>(devices: Vec<T>) -> Result<Vec<T>, DeviceError> {
    match dg::filter_keyboards(devices) {
        Some(devices) => Ok(devices),
        None => Err(DeviceError::DevicesNotFound(format!("No keyboards found."))),
    }
}

fn get_non_virtual_keyboards() -> Result<Vec<Device>, DeviceError> {
    match filter_keyboards(filter_out_virtual_devices(get_all_devices()?)?) {
        Ok(keyboards) => Ok(keyboards),
        Err(no_keyboard_error) => Err(DeviceError::DevicesNotFound(format!(
            "No non-virtual keyboards found in existing devices."
        ))),
    }
}

fn filter_by_names<T: DeviceInfo>(
    devices: Vec<T>,
    names: Vec<String>,
) -> Result<Vec<T>, DeviceError> {
    match dg::extract_named_devices(devices, &names) {
        Some(devices) => Ok(devices),
        None => Err(DeviceError::DevicesNotFound(format!(
            "No devices found which match names: {:?}",
            names
        ))),
    }
}

fn filter_out_named_devices<T: DeviceInfo>(devices: Vec<T>, names: &Vec<String>) -> Option<Vec<T>> {
    let devices: Vec<T> = devices
        .into_iter()
        .filter(|device| match device.name() {
            None => true,
            Some(s) => !names.contains(&String::from(s)),
        })
        .collect();

    match devices.len() {
        0 => None,
        _ => Some(devices),
    }
}

fn filter_out_excluded_devices<T: DeviceInfo>(
    devices: Vec<T>,
    excluded: Option<Vec<String>>,
) -> Result<Vec<T>, DeviceError> {
    match excluded {
        None => Ok(devices),
        Some(excluded) => match filter_out_named_devices(devices, &excluded) {
            Some(devices) => Ok(devices),
            None => Err(DeviceError::DevicesNotFound(format!(
                "No devices left after filtering out excluded devices."
            ))),
        },
    }
}

impl DevicesConfig {
    pub fn extract_devices_to_remap(self) -> Result<Vec<Device>, DeviceError> {
        let keyboards = match self.include {
            None => get_non_virtual_keyboards()?,
            Some(names) => filter_by_names(filter_out_virtual_devices(get_all_devices()?)?, names)?,
        };
        filter_out_excluded_devices(keyboards, self.exclude)
    }
}
