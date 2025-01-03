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

    let mut store = Store::new(0);
    let cp1 = Chip::new([1, 2, 3, 4], "chris");
    store.add_chip(cp1).unwrap();
    info!("Data to save: {:?}", store);

    let _ = store.save();

    let mut store2 = Store::new(0);
    let _ = store2.load();
    info!("Read data: {:?}", store2);

    info!("Start Loop");
    loop {
        delay.delay(2000.millis());
    }
}
