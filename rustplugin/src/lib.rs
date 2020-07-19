#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

use bakkesmod::{self, log_console, plugin_init};
use bakkesmod::wrappers::Car;

#[plugin_init]
pub fn on_load() {
    bakkesmod::register_notifier("rust_notifier", Box::new(move |params: Vec<String>| {
        log_console!("this is the callback for rust_notifier!");
        log_console!("params = {:x?}", params);
    }));

    bakkesmod::register_notifier("rust_demolish", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => car.demolish(),
            None => log_console!("Car is NULL")
        };
    }));
}