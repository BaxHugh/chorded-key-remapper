use super::key::Key;
use crate::errors::{DeviceError, VirtualDeviceCreationError};
use log;
use std::{io, path::PathBuf};
// Structs which wrap structs provided by another device interface library, currently evdev, but
// this library could be changed if compiling for a different OS, or if another library is later preferred.

pub struct Device(evdev::Device);

pub struct VirtualDevice(evdev::uinput::VirtualDevice);

pub trait DeviceInfo: ToString {
    fn supported_keys(&self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError>;
    fn name(&self) -> Option<&str>;
}

pub trait VirtualDeviceInfo {
    fn enumerate_dev_nodes(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = io::Result<PathBuf>>>, DeviceError>;
}

impl Device {
    #[inline]
    pub const fn new(device: evdev::Device) -> Self {
        Self(device)
    }
}

impl VirtualDevice {
    #[inline]
    pub const fn new(virtual_device: evdev::uinput::VirtualDevice) -> Self {
        Self(virtual_device)
    }

    pub fn from_template_device<T: DeviceInfo>(
        name: &str,
        template_device: &mut T,
    ) -> Result<VirtualDevice, VirtualDeviceCreationError> {
        let mut keys = evdev::AttributeSet::<evdev::Key>::new();
        for key in template_device.supported_keys()? {
            keys.insert(key.0);
        }

        let mut device = VirtualDevice(
            evdev::uinput::VirtualDeviceBuilder::new()?
                .name(name)
                .with_keys(&keys)?
                .build()?,
        );

        for path in device.enumerate_dev_nodes()? {
            let path = path?;
            println!("Virtual device available as {}", path.display());
        }
        return Ok(device);
    }
}

impl ToString for Device {
    fn to_string(&self) -> String {
        return format!(
            "{}",
            match self.name() {
                None => "UNNAMED",
                Some(name) => name,
            }
        );
    }
}

impl DeviceInfo for Device {
    fn supported_keys(&self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError> {
        return Ok(Box::new(match self.0.supported_keys() {
            Some(evdev_keys) => evdev_keys.iter().into_iter().map(|k| Key(k)),
            None => {
                return Err(DeviceError::SupportedKeysEmpty(format!(
                    "No supported keys found on template device: {:?}",
                    self.name()
                )))
            }
        }));
    }

    fn name(&self) -> Option<&str> {
        return self.0.name();
    }
}

impl VirtualDeviceInfo for VirtualDevice {
    fn enumerate_dev_nodes(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = io::Result<PathBuf>>>, DeviceError> {
        match self.0.enumerate_dev_nodes_blocking() {
            Ok(iter) => Ok(Box::new(iter)),
            Err(err) => Err(DeviceError::IO(err)),
        }
    }
}

pub mod device_getting {
    use super::{Device, DeviceInfo};
    use crate::{device, errors::DeviceError};
    use std::{
        path::{Path, PathBuf},
        str::FromStr,
    };

    fn enumerate_devices() -> Box<dyn Iterator<Item = (PathBuf, Device)>> {
        Box::new(
            evdev::enumerate()
                .into_iter()
                .map(|(path, device)| (path, Device::new(device))),
        )
    }

    pub fn get_all_devices() -> Option<Vec<Device>> {
        let devices: Vec<Device> = enumerate_devices().map(|(_, device)| device).collect();
        match devices.len() {
            0 => None,
            _ => Some(devices),
        }
    }

    pub fn filter_keyboards<T: DeviceInfo>(devices: Vec<T>) -> Option<Vec<T>> {
        // TODO: Work out how to write this properly, I tried a bit with iter_mut but didn't get far.
        let devices: Vec<T> = devices
            .into_iter()
            .filter(|device| is_keyboard(device))
            .collect();
        match devices.len() {
            0 => None,
            _ => Some(devices),
        }
    }

    pub fn extract_named_device<T: DeviceInfo>(
        devices: Vec<T>,
        name: &str,
    ) -> Result<T, DeviceError> {
        let mut devices: Vec<T> = devices
            .into_iter()
            .filter(|device| device.name() == Some(name))
            .collect();
        match devices.len() {
            0 => Err(DeviceError::DeviceNotFound(format!(
                "could not find device: {name}"
            ))),
            1 => Ok(devices.remove(0)),
            _ => {
                log::warn!("Multiple devices found with name: {name}, using the first result.");
                Ok(devices.remove(0))
            }
        }
    }

    pub fn extract_named_devices<T: DeviceInfo>(
        devices: Vec<T>,
        names: &Vec<String>,
    ) -> Option<Vec<T>> {
        let devices: Vec<T> = devices
            .into_iter()
            .filter(|device| match device.name() {
                None => false,
                Some(s) => names.contains(&String::from(s)),
            })
            .collect();
        match devices.len() {
            0 => None,
            _ => Some(devices),
        }
    }

    pub fn get_device_from_path(path: impl AsRef<Path>) -> Result<Device, DeviceError> {
        let device = evdev::Device::open(path)?;
        return Ok(Device::new(device));
    }

    fn is_keyboard<T: DeviceInfo>(device: &T) -> bool {
        return device.supported_keys().map_or(false, |mut keys| {
            // TODO: Currently just patched this with call to evdev, but need to wrap key types in this project's Key struct
            keys.any(|key| key.0 == evdev::Key::KEY_ENTER)
        });
    }
}
