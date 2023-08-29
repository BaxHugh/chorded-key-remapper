use super::key::Key;
use crate::errors::{DeviceError, VirtualDeviceCreationError};
use evdev::{AttributeSet, AttributeSetRef};
use log;
use std::{io, path::PathBuf};
// Structs which wrap structs provided by another device interface library, currently evdev, but
// this library could be changed if compiling for a different OS, or if another library is later preferred.

pub struct Device(pub evdev::Device);

pub struct VirtualDevice(pub evdev::uinput::VirtualDevice);

pub trait DeviceInfo: ToString {
    type Iter<'a>: Iterator<Item = Key>
    where
        Self: 'a;
    fn supported_keys<'a>(&'a self) -> Result<Self::Iter<'a>, DeviceError>;
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
        let keys = AttributeSet::<evdev::Key>::from_iter(template_device.supported_keys()?);

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
    type Iter<'a> = Box<dyn Iterator<Item = Key> + 'a>;
    fn supported_keys<'a>(&'a self) -> Result<Self::Iter<'a>, DeviceError> {
        return Ok(match self.0.supported_keys() {
            Some(evdev_keys) => Box::new(evdev_keys.iter()),
            None => {
                return Err(DeviceError::SupportedKeysEmpty(format!(
                    "No supported keys found on template device: {:?}",
                    self.name()
                )))
            }
        });
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

fn enumerate_devices() -> Box<dyn Iterator<Item = (PathBuf, Device)>> {
    Box::new(
        evdev::enumerate()
            .into_iter()
            .map(|(path, device)| (path, Device::new(device))),
    )
}

pub fn get_all_devices() -> Result<Vec<Device>, DeviceError> {
    let devices = enumerate_devices()
        .map(|(_, device)| device)
        .collect::<Vec<Device>>();
    match devices.len() {
        0 => Err(DeviceError::DevicesNotFound(format!(
            "No devices found, make sure the program is running under sudo privileges."
        ))),
        _ => Ok(devices),
    }
}
