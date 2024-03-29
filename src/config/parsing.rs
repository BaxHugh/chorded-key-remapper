use super::schema::{Config, DevicesConfig};
use crate::auxiliary::device_filtering::FilterableDevices;
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
            // Automatically select all non-virtual keyboards if no specific devices are specified in the config.
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

            // If specific devices are specified in the config, select those.
            Some(include_names) => match all_devices.extract_named_devices(&include_names) {
                None => Err(DeviceError::DevicesNotFound(format!(
                    "No devices found which match names: {}",
                    format_many_device_names(&include_names)
                ))),

                Some(devices) => {
                    if log_enabled!(log::Level::Info) {
                        if devices.len() != include_names.len() {
                            let found: Vec<&str> = devices
                                .iter()
                                .map(|dev| match dev.name() {
                                    Some(name) => name,
                                    None => "UNNAMED",
                                })
                                .collect();

                            let missing: Vec<String> = include_names
                                .iter()
                                .filter(|name| !found.contains(&name.as_str()))
                                .map(|name| name.to_owned())
                                .collect();

                            log::info!(
                                "Not all named devices where found. Couldn't find: {}",
                                format_many_device_names(&missing)
                            );
                        }
                    }
                    Ok(devices)
                }
            },
        }?;

        match self.exclude {
            None => Ok(keyboards),
            Some(exclude_names) => match keyboards.remove_named_devices(&exclude_names) {
                Some(keyboards) => Ok(keyboards),

                None => Err(DeviceError::DevicesNotFound(format!(
                    "No devices left after filtering out excluded devices: {}",
                    format_many_device_names(&exclude_names)
                ))),
            },
        }
    }
}

fn format_many_device_names(names: &Vec<String>) -> String {
    names
        .iter()
        .map(|name| format!("'{}'", name))
        .collect::<Vec<String>>()
        .join(", ")
}

#[cfg(test)]
mod test_DevicesConfig_extract_devices_to_remap {
    use super::*;
    use crate::Key;
    extern crate testing_logger;

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct MockDevice {
        name: Option<String>,
        is_keyboard: bool,
    }

    impl MockDevice {
        pub fn new(name: &str, is_keyboard: bool) -> MockDevice {
            MockDevice {
                name: Some(name.to_owned()),
                is_keyboard,
            }
        }
    }

    struct VecIterator<T> {
        vec: Vec<T>,
        index: usize,
    }

    impl<T> VecIterator<T> {
        pub fn new(vec: Vec<T>) -> VecIterator<T> {
            VecIterator { vec, index: 0 }
        }
    }

    impl<T> Iterator for VecIterator<T>
    where
        T: Clone,
    {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.vec.len() {
                let item = self.vec[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    impl DeviceInfo for MockDevice {
        type Iter<'a> = VecIterator<Key>;
        fn supported_keys(&self) -> Result<Self::Iter<'_>, DeviceError> {
            if self.is_keyboard {
                Ok(VecIterator::new(vec![Key::KEY_ENTER]))
            } else {
                Ok(VecIterator::new(vec![]))
            }
        }

        fn name(&self) -> Option<&str> {
            match &self.name {
                None => None,
                Some(name) => Some(name.as_str()),
            }
        }
    }

    impl ToString for MockDevice {
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

    struct Keyboard {}
    struct NotKeyboard {}

    impl Keyboard {
        pub fn new(name: &str) -> MockDevice {
            MockDevice {
                name: Some(name.to_owned()),
                is_keyboard: true,
            }
        }
    }
    impl NotKeyboard {
        pub fn new(name: &str) -> MockDevice {
            MockDevice {
                name: Some(name.to_owned()),
                is_keyboard: false,
            }
        }
    }

    fn check_selected_devs_are_expected(
        config: DevicesConfig,
        all_devs: Vec<MockDevice>,
        expect: Vec<MockDevice>,
    ) {
        assert_eq!(config.extract_devices_to_remap(all_devs).unwrap(), expect,)
    }

    fn mixed_devices() -> Vec<MockDevice> {
        vec![
            Keyboard::new("virtual keyboard"),
            Keyboard::new("keyboard VIRTUAL"),
            NotKeyboard::new("virtual device"),
            NotKeyboard::new("real device 1"),
            NotKeyboard::new("real device 2"),
            Keyboard::new("real keyboard 1"),
            Keyboard::new("real keyboard 2"),
        ]
    }

    #[cfg(test)]
    mod test_device_selection_when_include_is_empty {
        use super::*;

        #[test]
        fn only_real_keyboards_selected() {
            check_selected_devs_are_expected(
                DevicesConfig {
                    include: None,
                    exclude: None,
                },
                mixed_devices(),
                vec![
                    Keyboard::new("real keyboard 1"),
                    Keyboard::new("real keyboard 2"),
                ],
            )
        }
        #[test]
        fn excluded_devices_are_not_selected() {
            check_selected_devs_are_expected(
                DevicesConfig {
                    include: None,
                    exclude: Some(vec![
                        "special keyboard".to_owned(),
                        "special device".to_owned(),
                    ]),
                },
                [
                    mixed_devices(),
                    vec![
                        Keyboard::new("special keyboard"),
                        NotKeyboard::new("special device"),
                    ],
                ]
                .concat(),
                vec![
                    Keyboard::new("real keyboard 1"),
                    Keyboard::new("real keyboard 2"),
                ],
            )
        }
    }

    #[cfg(test)]
    mod test_device_selection_when_include_is_specified {
        use super::*;

        #[test]
        fn only_include_devices_selected() {
            check_selected_devs_are_expected(
                DevicesConfig {
                    include: Some(vec![
                        "real device 2".to_owned(),
                        "real keyboard 2".to_owned(),
                    ]),
                    exclude: None,
                },
                mixed_devices(),
                vec![
                    NotKeyboard::new("real device 2"),
                    Keyboard::new("real keyboard 2"),
                ],
            )
        }

        #[test]
        fn excluded_devices_are_not_selected_even_if_in_include_as_well() {
            check_selected_devs_are_expected(
                DevicesConfig {
                    include: Some(vec![
                        "real device 2".to_owned(),
                        "real keyboard 2".to_owned(),
                    ]),
                    exclude: Some(vec![
                        "special device".to_owned(),
                        "special keyboard".to_owned(),
                        "real keyboard 2".to_owned(), // also in the include
                    ]),
                },
                [
                    mixed_devices(),
                    vec![
                        Keyboard::new("special keyboard"),
                        NotKeyboard::new("special device"),
                    ],
                ]
                .concat(),
                vec![NotKeyboard::new("real device 2")], // 'real keyboard 2' in both include and exclude so shouldn't be here
            )
        }
    }

    #[cfg(test)]
    mod test_no_device_selected_gives_error {

        use super::*;

        #[test]
        fn expected_error_and_message_when_no_devices_at_all() {
            let result = DevicesConfig {
                include: None,
                exclude: None,
            }
            .extract_devices_to_remap(Vec::<MockDevice>::new());
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert!(matches!(err, DeviceError::DevicesNotFound(_)));
            assert!(err
                .to_string()
                .contains("No non-virtual devices fond in existing devices."));
        }

        #[test]
        fn expected_error_and_message_when_no_devices_left_after_excluded() {
            let result = DevicesConfig {
                include: None,
                exclude: Some(vec!["real keyboard 2".to_owned()]),
            }
            .extract_devices_to_remap(vec![Keyboard::new("real keyboard 2")]);
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert!(matches!(err, DeviceError::DevicesNotFound(_)));
            assert!(err
                .to_string()
                .contains("No devices left after filtering out excluded devices"));
        }
    }

    #[test]
    fn test_no_error_and_info_logged_when_not_all_include_devices_are_present() {
        testing_logger::setup();

        let result = DevicesConfig {
            include: Some(vec![
                "real keyboard 1".to_owned(),
                "not present keyboard".to_owned(),
                "not present keyboard 2".to_owned(),
            ]),
            exclude: None,
        }
        .extract_devices_to_remap(vec![Keyboard::new("real keyboard 1")]);
        assert!(result.is_ok());
        testing_logger::validate(|captured_logs| {
            assert_eq!(
                captured_logs.len(),
                1,
                "Expected 1 log message but got {}.",
                captured_logs.len()
            );
            assert!(captured_logs[0].body.contains(
                "Not all named devices where found. Couldn't find: 'not present keyboard', 'not present keyboard 2'"
            ));
        })
    }
    #[test]
    fn test_no_error_when_not_all_exclude_devices_are_present() {
        let result = DevicesConfig {
            include: None,
            exclude: Some(vec![
                "real keyboard 1".to_owned(),
                "not present keyboard".to_owned(),
            ]),
        }
        .extract_devices_to_remap(vec![
            Keyboard::new("real keyboard 1"),
            Keyboard::new("real keyboard 2"),
        ]);
        assert!(result.is_ok());
    }
}
