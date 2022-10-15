use std::path::Path;

use crate::device_library_interface::{DeviceWrapper, VirtualDeviceWrapper};
use crate::errors::{DeviceError, VirtualDeviceCreationError};

use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, Device, Key};

pub fn create_virtual_device(
    name: &str,
    template_device: &impl DeviceWrapper,
) -> Result<impl VirtualDeviceWrapper, VirtualDeviceCreationError> {
    let mut keys = AttributeSet::<Key>::new();
    for key in template_device.supported_keys()? {
        keys.insert(key);
    }

    let mut device = VirtualDeviceBuilder::new()?
        .name(name)
        .with_keys(&keys)?
        .build()?;

    for path in device.enumerate_dev_nodes_blocking()? {
        let path = path?;
        println!("Virtual device available as {}", path.display());
    }
    return Ok(device);
}

pub fn get_device_from_name(name: &str) -> Result<impl DeviceWrapper, DeviceError> {
    let mut dev_count = 0;
    for (_, device) in evdev::enumerate() {
        dev_count += 1;
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
        "Unble to find device with name '{name}'. Searched {dev_count} devices. \
        Make sure the program is running with sudo privileges."
    )));
}

pub fn get_device_from_path(path: impl AsRef<Path>) -> Result<impl DeviceWrapper, DeviceError> {
    let mut device = Device::open(path)?;
    return Ok(device);
}
