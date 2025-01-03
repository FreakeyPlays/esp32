#![no_std]
#![no_main]

mod lib {
    pub mod flash;
    pub mod model {
        pub mod store;
    }
}

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    prelude::*,
};
use lib::{
    buzzer,
    flash::FlashStorage,
    gpio,
    lcd::Lcd,
    led,
    model::store::{Chip, Store},
};
use log::info;

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let mut flash_storage = FlashStorage::new();
    let mut data_to_write = Store::new();
    let cp1 = Chip::new([1, 2, 3, 4], "chris");
    data_to_write.add_chip(cp1).unwrap();
    info!("Data to write: {:?}", data_to_write);

    let mut buffer = [0; 250];
    if let Err(e) = data_to_write.to_bytes(&mut buffer) {
        info!("Failed to convert data to bytes: {:?}", e);
    }
    info!("To Bytes: {:?}", buffer);
    flash_storage.write_bytes(0x3F0000, &buffer).unwrap();

    let mut buffer = [0; 250];
    if let Err(e) = flash_storage.read_bytes(0x3F0000, &mut buffer) {
        info!("Failed to read bytes from flash storage: {:?}", e);
    }
    info!("From Bytes: {:?}", buffer);
    let read_data = Store::new_from_bytes(&buffer);

    info!("Final Data: {:?}", read_data);

    info!("Start Loop");
    loop {
        delay.delay(2000.millis());
    }
}
