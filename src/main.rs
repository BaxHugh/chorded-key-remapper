use std::path::Path;

use errors::Error;
use evdev::{EventType, InputEventKind};

use crate::device::{get_all_devices, Device, DeviceInfo, VirtualDevice};

mod auxiliary;
mod config;
mod device;
mod errors;
mod mapping;
mod remapper;

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
    let mut keyboards = config
        .devices
        .extract_devices_to_remap(get_all_devices()?)?;

    println!("Selected devices:");
    print_devices(&keyboards);

    let mut chosen_keyboard = keyboards.remove(0);

    let virtual_keyboard_name = "My Virtual Keyboard";
    let mut virtual_device =
        VirtualDevice::from_template_device(virtual_keyboard_name, &mut chosen_keyboard)?;
    println!("Virtual keyboard '{virtual_keyboard_name}'");
    println!();

    chosen_keyboard.0.grab()?;
    loop {
        for ev in chosen_keyboard.0.fetch_events().unwrap() {
            println!("{ev:?}");
            if ev.event_type() == EventType::KEY {
                let vev = evdev::InputEvent::new(ev.event_type(), ev.code() + 1, ev.value());
                virtual_device.0.emit(&[vev])?;
            }
            virtual_device.0.emit(&[ev])?;
        }
    }
    // loop {
    //     for ev in chosen_keyboard.0.fetch_events()? {
    //         println!("{:?}", ev);
    //         // virtual_device.0.write_event(ev)?;
    //     }
    // }
    // let (keyboards, _) = get_all_keyboards();
    // println!("New keyboards found:");
    // print_devices(&keyboards);
    return Ok(());
}
