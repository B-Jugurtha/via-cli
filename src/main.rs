extern crate hidapi;

use core::fmt;
use std::collections::HashMap;

use inquire::{InquireError, Select};

use hidapi::{HidApi, HidDevice};

fn main() {
    let usage_hex: u16 = 0x0061;
    let usage_page_hex: u16 = 0xff60;
    let mut keyboards: Vec<KeyboardDevice> = Vec::new();
    let mut information_operations =
        HashMap::from([(String::from("test"), 0x01), (String::from("test"), 0x01)]);
    struct KeyboardDevice {
        product_string: String, // A field holding text
        product_id: u16,        // A field holding a 16-bit unsigned integer
        vendor_id: u16,
    }

    impl fmt::Display for KeyboardDevice {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.product_string)
        }
    }

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.usage() == usage_hex && device.usage_page() == usage_page_hex {
                    // println!(
                    //     "{:?}:{:04x},{:04x}",
                    //     device.product_string().unwrap_or("Unknown HID Device"),
                    //     device.product_id(),
                    //     device.usage(),
                    // );
                    match device.open_device(&api) {
                        Ok(hid_device) => {
                            let mut data_write = [0u8; 33];
                            data_write[1] = 0x01;
                            match hid_device.write(&data_write) {
                                Ok(_) => {
                                    let mut read_data = [0u8; 32];
                                    match hid_device.read(&mut read_data) {
                                        Ok(_) => {
                                            keyboards.push(KeyboardDevice {
                                                product_string: device
                                                    .product_string()
                                                    .unwrap_or("Unknown HID Device")
                                                    .to_string(),
                                                product_id: device.product_id(),
                                                vendor_id: device.vendor_id(),
                                            });
                                        }
                                        Err(e) => {
                                            println!("Error:{}", e)
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
            let ans: Result<KeyboardDevice, InquireError> =
                Select::new("Please Select Your Keyboard:", keyboards).prompt();

            match ans {
                Ok(choice) => {
                    let generalChoice: Vec<&str> = vec!["Device Informations", "Exit"];
                    let ans2: Result<&str, InquireError> =
                        Select::new("Please Select Your Keyboard:", generalChoice).prompt();
                    match ans2 {
                        Ok(choicez) => {
                            println!("{}", choicez)
                        }
                        Err(_) => println!("There was an error, please try again"),
                    }
                }
                Err(_) => println!("There was an error, please try again"),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
