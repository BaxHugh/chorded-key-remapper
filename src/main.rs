use std::path::Path;

use errors::Error;

use crate::device::{get_all_devices, DeviceInfo};

mod auxiliary;
mod config;
mod device;
mod errors;
mod key;
mod mapping;

pub use crate::key::Key;

fn print_devices(devices: &Vec<impl DeviceInfo>) {
    for device in devices {
        println!("'{}'", device.to_string());
    }
}

extern crate env_logger;
extern crate log;

fn main() -> Result<(), Error> {
    env_logger::init();

    let config = config::parsing::read_config_file(Path::new("config.toml"))?;
    let keyboards = config
        .devices
        .extract_devices_to_remap(get_all_devices()?)?;

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
