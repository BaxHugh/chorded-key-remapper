use crate::device::DeviceInfo;

pub trait FilterableDevices<T> {
    fn extract_keyboards(self) -> Option<T>;
    fn extract_named_devices(self, names: &Vec<String>) -> Option<T>;
    fn remove_named_devices(self, names: &Vec<String>) -> Option<T>;
    fn extract_devices_whose_name_doesnt_contain(self, substring: &str) -> Option<T>;
}

impl<T> FilterableDevices<Vec<T>> for Vec<T>
where
    T: DeviceInfo,
{
    fn extract_keyboards(self) -> Option<Vec<T>> {
        let devices: Vec<T> = self
            .into_iter()
            .filter(|device| is_keyboard(device))
            .collect();
        match devices.len() {
            0 => None,
            _ => Some(devices),
        }
    }

    fn extract_named_devices(self, names: &Vec<String>) -> Option<Vec<T>> {
        let devices: Vec<T> = self
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

    fn remove_named_devices(self, names: &Vec<String>) -> Option<Vec<T>> {
        let devices: Vec<T> = self
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

    fn extract_devices_whose_name_doesnt_contain(self, substring: &str) -> Option<Vec<T>> {
        let devices: Vec<T> = self
            .into_iter()
            .filter(|device| !device.to_string().to_lowercase().contains(substring))
            .collect();
        match devices.len() {
            0 => None,
            _ => Some(devices),
        }
    }
}

fn is_keyboard<T: DeviceInfo>(device: &T) -> bool {
    return device.supported_keys().map_or(false, |mut keys| {
        // TODO: Currently just patched this with call to evdev, but need to wrap key types in this project's Key struct
        keys.any(|key| key == evdev::Key::KEY_ENTER)
    });
}
