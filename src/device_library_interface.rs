use crate::errors::DeviceError;
use evdev::{uinput::VirtualDevice, Device, Key};
use std::{io, path::PathBuf};

pub trait DeviceWrapper {
    fn supported_keys(&self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError>;
    fn name(&self) -> Option<&str>;
}

pub trait VirtualDeviceWrapper {
    fn enumerate_dev_nodes(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = io::Result<PathBuf>>>, DeviceError>;
}

impl DeviceWrapper for Device {
    fn supported_keys(&self) -> Result<Box<dyn Iterator<Item = Key> + '_>, DeviceError> {
        return Ok(Box::new(
            match self.supported_keys() {
                Some(keys) => keys,
                None => {
                    return Err(DeviceError::SupportedKeysEmpty(format!(
                        "No supported keys found on template device: {:?}",
                        self.name()
                    )))
                }
            }
            .iter(),
        ));
    }

    fn name(&self) -> Option<&str> {
        return self.name();
    }
}

impl VirtualDeviceWrapper for VirtualDevice {
    fn enumerate_dev_nodes(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = io::Result<PathBuf>>>, DeviceError> {
        match self.enumerate_dev_nodes_blocking() {
            Ok(iter) => Ok(Box::new(iter)),
            Err(err) => Err(DeviceError::IO(err)),
        }
    }
}
