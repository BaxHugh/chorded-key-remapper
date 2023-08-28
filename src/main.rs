use std::path::Path;

use errors::Error;

use crate::device::{get_all_devices, DeviceInfo, VirtualDevice};

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

// #[derive(Debug, Clone, Copy)]
// struct MyStruct {
//     val: u8,
// }

// impl Default for MyStruct {
//     fn default() -> Self {
//         Self { val: 0 }
//     }
// }

// static DEFAULT_STRUCT: MyStruct = MyStruct { val: 0 };

// // static mut DATA: [MyStruct::default(); 5];
// static mut DATA: [MyStruct; 5] = [DEFAULT_STRUCT; 5];

// fn main() {
//     let vals = vec![
//         MyStruct { val: 1 },
//         MyStruct { val: 2 },
//         MyStruct { val: 3 },
//         MyStruct { val: 4 },
//         MyStruct { val: 5 },
//     ];
//     // let _n: u16 = 5;
//     // let n: usize = usize::from(_n);
//     for val in vals {
//         let ind = usize::from(val.val);
//         DATA[ind] = val;
//         // DATA[4] = val;
//     }
//     print!("{:?}", DATA);
// }
