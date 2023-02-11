use crate::errors::{DeviceError, VirtualDeviceCreationError};
use std::{io, path::PathBuf};

// Structs which wrap structs provided by another device interface library, currently evdev, but
// this library could be changed if compiling for a different OS, or if another library is later preferred.

pub struct Key(evdev::Key);

pub struct Device(evdev::Device);

pub struct VirtualDevice(evdev::uinput::VirtualDevice);

pub trait DeviceInfo: ToString {
    fn supported_keys(&mut self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError>;
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
    fn supported_keys(&mut self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError> {
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
    use crate::device::{Device, DeviceInfo};
    use crate::errors::DeviceError;
    use std::path::{Path, PathBuf};

    pub fn enumerate_devices() -> Box<dyn Iterator<Item = (PathBuf, Device)>> {
        Box::new(
            evdev::enumerate()
                .into_iter()
                .map(|(path, device)| (path, Device::new(device))),
        )
    }

    pub fn get_all_keyboards() -> (Vec<Device>, i32) {
        let mut found_device_count = 0;
        let mut keyboards = Vec::<Device>::new();
        for (_, mut device) in enumerate_devices() {
            found_device_count += 1;
            if is_keyboard(&mut device) {
                keyboards.push(device);
            }
        }
        return (keyboards, found_device_count);
    }

    pub fn get_device_from_name(name: &str) -> Result<Device, DeviceError> {
        let mut found_device_count = 0;
        for (_, device) in enumerate_devices() {
            found_device_count += 1;
            match device.name() {
                None => continue,
                Some(dev_name) => {
                    if dev_name == name {
                        return Ok(device);
                    }
                }
            }
        }
        return Err(DeviceError::DeviceNotFound(format!(
            "Unble to find device with name '{name}'. Searched {found_device_count} devices. \
        Make sure the program is running with sudo privileges."
        )));
    }

    pub fn get_device_from_path(path: impl AsRef<Path>) -> Result<Device, DeviceError> {
        let device = evdev::Device::open(path)?;
        return Ok(Device::new(device));
    }

    fn is_keyboard(device: &mut impl DeviceInfo) -> bool {
        return device.supported_keys().map_or(false, |mut keys| {
            // TODO: Currently just patched this with call to evdev, but need to wrap key types in this project's Key struct
            keys.any(|key| key.0 == evdev::Key::KEY_ENTER)
        });
    }
}
