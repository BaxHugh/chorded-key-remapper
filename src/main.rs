use crate::{
    device::{create_virtual_device, get_device_from_name},
    device_library_interface::DeviceWrapper,
};

mod device;
mod device_library_interface;
mod errors;

fn main() {
    println!("Hello keyboard!");
    let keyboard_name = "Dell Keybd KB7120W Keyboard";
    let real_keyboard = get_device_from_name(keyboard_name).unwrap();
    let virtual_device = create_virtual_device("Virtual Keyboard", &real_keyboard);
    println!("Real keyboard {:?}", real_keyboard.name());
}
