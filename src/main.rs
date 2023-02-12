use std::path::Path;

use errors::Error;

use crate::device::{DeviceInfo, VirtualDevice};

mod config;
mod device;
mod errors;
mod mapping;

fn print_devices(devices: &Vec<impl DeviceInfo>) {
    for device in devices {
        println!("'{}'", device.to_string());
    }
}

fn main() -> Result<(), Error> {
    let config = config::parsing::read_config_file(Path::new("config.toml"))?;
    let keyboards = config.devices.extract_devices_to_remap()?;

    println!("Selected devices:");
    print_devices(&keyboards);

    // println!();
    // let virtual_keyboard_name = "My Virtual Keyboard";
    // let virtual_device =
    //     VirtualDevice::from_template_device(virtual_keyboard_name, &mut chosen_keyboard)?;
    // println!("Virtual keyboard {virtual_keyboard_name}");
    // println!();

    // let (keyboards, _) = get_all_keyboards();
    // println!("New keyboards found:");
    // print_devices(&keyboards);
    return Ok(());
}
