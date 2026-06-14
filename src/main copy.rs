extern crate hidapi;

use inquire::{
    InquireError, Select, Text,
    validator::{StringValidator, Validation},
};

use hidapi::{HidApi, HidDevice};

fn main() {
    // let options: Vec<&str> = vec![
    //     "Banana",
    //     "Apple",
    //     "Strawberry",
    //     "Grapes",
    //     "Lemon",
    //     "Tangerine",
    //     "Watermelon",
    //     "Orange",
    //     "Pear",
    //     "Avocado",
    //     "Pineapple",
    // ];

    // let ans: Result<&str, InquireError> =
    //     Select::new("What's your favorite fruit?", options).prompt();

    // match ans {
    //     Ok(choice) => println!("{}! That's mine too!", choice),
    //     Err(_) => println!("There was an error, please try again"),
    // }

    let mut keyboards: Vec<HidDevice> = Vec::new();

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.product_id() == 0x6369 && device.usage_page() == 0xff60 {
                    println!(
                        "{:04x}:{:04x},{:04x},{:04x}",
                        device.vendor_id(),
                        device.product_id(),
                        device.usage_page(),
                        device.usage(),
                    );
                    match device.open_device(&api) {
                        Ok(hid_device) => {
                            keyboards.push(hid_device);
                            let mut data_write = [0u8; 33];
                            data_write[1] = 0x01;
                            match hid_device.write(&data_write) {
                                Ok(_) => {
                                    println!("data written succesfully");
                                    let mut read_data = [0u8; 32];
                                    match hid_device.read(&mut read_data) {
                                        Ok(_) => {
                                            print!("read data: {:#?}", read_data)
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
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
