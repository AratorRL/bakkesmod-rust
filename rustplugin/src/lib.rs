#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

use std::rc::Rc;
use std::cell::RefCell;

use bakkesmod::{self, log_console, plugin_init};
use bakkesmod::wrappers::{Car, CarWrapper, Actor, Vector};

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

    bakkesmod::register_notifier("rust_set_loc", Box::new(move |_: Vec<String>| {
        match bakkesmod::get_local_car() {
            Some(car) => {
                car.set_location(Vector::new(0.0, 0.0, 0.0));
            }
            None => log_console!("Car is NULL")
        };
    }));

    bakkesmod::register_cvar("rust_cvar");

    let counter_base = Rc::new(RefCell::new(0));
    let counter_ref = Rc::clone(&counter_base);

    bakkesmod::hook_event("Function Engine.GameViewportClient.Tick", Box::new(move || {
        let mut counter = counter_ref.borrow_mut();
        *counter += 1;
        if (*counter % 120) == 0 {
            log_console!("viewport client tick");

            match bakkesmod::get_local_car() {
                Some(car) => {
                    let location = car.get_location();
                    log_console!("{}", &location.to_string());
                }
                None => log_console!("Car is NULL")
            };
        }
    }));

    // bakkesmod::hook_event_with_caller(
    //     "Function TAGame.Car_TA.SetVehicleInput",
    //     Box::new(move |car: Box<CarWrapper>| {
    //         car.demolish();
    //     }));
}