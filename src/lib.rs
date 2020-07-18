#![allow(unused)]
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

use bakkesmod_macros::plugin_init;

#[macro_use]
mod bakkesmod;

pub mod wrappers;
use wrappers::Car;

fn rust_demolish(params: Vec<String>) {
    match bakkesmod::get_local_car() {
        Some(car) => car.demolish(),
        None => bakkesmod::log("Car is NULL")
    }
}

struct Plugin {
    my_data: u32
}

#[plugin_init]
pub fn on_load() {

    let plugin = Rc::new(RefCell::new(Plugin {
        my_data: 1337
    }));

    log_console!("henlo from rust");

    let plugin_ref = Rc::clone(&plugin);

    let cb: Box<dyn FnMut(Vec<String>)> = Box::new(move |params: Vec<String>| {
        info!("Hello from a callback!");
        bakkesmod::log("henlo from a rust callback!");
        let my_data = plugin_ref.borrow().my_data;
        bakkesmod::log(&format!("my data = {}", my_data));
    });
    bakkesmod::register_notifier("rust_callback", cb);
    
    bakkesmod::register_notifier("rust_notifier", Box::new(move |params: Vec<String>| {
        bakkesmod::log("this is the callback for rust_notifier!");
        bakkesmod::log(&format!("params = {:x?}", params));
        log_console!("params = {:x?}", params);

        for param in params {
            bakkesmod::log(&format!("param: {}", param));
        }
    }));

    bakkesmod::register_notifier("rust_demolish", Box::new(move |params: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => car.demolish(),
            None => log_console!("Car is NULL")
        };
    }));
}
