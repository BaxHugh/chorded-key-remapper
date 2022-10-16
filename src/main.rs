use errors::Error;

use crate::{
    device::{create_virtual_device, get_all_keyboards, get_device_from_name},
    device_library_interface::DeviceWrapper,
};

mod device;
mod device_library_interface;
mod errors;

fn print_devices(devices: &Vec<impl DeviceWrapper>) {
    for device in devices {
        println!("'{}'", device.to_string());
    }
}

fn filter_out_virtual_devices(devices: Vec<impl DeviceWrapper>) -> Vec<impl DeviceWrapper> {
    return devices
        .into_iter()
        .filter(|device| !device.to_string().to_lowercase().contains("virtual"))
        .collect();
}

fn main() -> Result<(), Error> {
    println!("Hello keyboard!");
    let (keyboards, num_of_devices_found) = get_all_keyboards();
    if num_of_devices_found == 0 {
        return Err(Error::from(errors::DeviceError::DevicesNotFound(
            "No devices found, Make sure the program is running with sudo privileges.",
        )));
    }
    let keyboards = filter_out_virtual_devices(keyboards);
    println!("Found Keyboards:");
    print_devices(&keyboards);

    let chosen_keyboard = &keyboards[0];

    println!();
    println!(
        "Real keyboard being used: '{}'",
        chosen_keyboard.to_string()
    );
    println!();
    let virtual_keyboard_name = "My Virtual Keyboard";
    let virtual_device = create_virtual_device(virtual_keyboard_name, chosen_keyboard);
    println!("Virtual keyboard {virtual_keyboard_name}");
    println!();

    let (keyboards, _) = get_all_keyboards();
    println!("New keyboards found:");
    print_devices(&keyboards);
    return Ok(());
}
