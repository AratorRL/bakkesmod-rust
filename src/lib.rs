#![allow(unused)]
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;

mod bakkesmod;
use bakkesmod::BakkesMod;
pub mod wrappers;
use wrappers::Car;

// fn notifier_callback(plugin: Rc<RefCell<Plugin>>, _: usize, _: u32) {
//     // let plugin: Plugin = unsafe { *Box::from_raw(user_data as * mut Plugin) };
//     let plugin = plugin.borrow_mut();
//     plugin.log("this is the callback for rust_notifier!");
//     // plugin.log(&format!("user_data = {:x?}", user_data));
// }

fn rust_demolish(_: usize, _: u32) {
    match bakkesmod::get_local_car() {
        Some(car) => car.demolish(),
        None => bakkesmod::log("Car is NULL")
    }
}

struct Plugin {
    my_data: u32
}

#[no_mangle]
pub extern "C" fn InitPlugin(id: u64) {
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("rustplugin.log").unwrap());
    info!("Hello from a Rust plugin!");

    let plugin = Rc::new(RefCell::new(Plugin {
        my_data: 1337
    }));

    bakkesmod::bakkesmod_init(id);
    bakkesmod::log("henlo from rust");

    let plugin_ref = Rc::clone(&plugin);

    let cb: Box<dyn FnMut(usize, u32)> = Box::new(move |params: usize, x: u32| {
        info!("Hello from a callback!");
        bakkesmod::log("henlo from a rust callback!");
        let my_data = plugin_ref.borrow().my_data;
        bakkesmod::log(&format!("my data = {}", my_data));
    });
    bakkesmod::register_notifier("rust_callback", cb);
    
    // bakkesmod::register_notifier("rust_notifier", notifier_callback);
    bakkesmod::register_notifier("rust_notifier", Box::new(move |params: usize, len: u32| {
        // let plugin: Plugin = unsafe { *Box::from_raw(user_data as * mut Plugin) };
        bakkesmod::log("this is the callback for rust_notifier!");
        // plugin.log(&format!("user_data = {:x?}", user_data));
        bakkesmod::log(&format!("params = {:x?}", params));

        if len <= 0 { return; }

        let params_ptr_ptr = params as *const *const c_char;
        if params_ptr_ptr.is_null() { info!("ptr to params ptr is null!"); return; }

        for i in 0..len {
            // let params_ptr = params_ptr_ptr.offset(i);
            let params_ptr = unsafe { *(params_ptr_ptr.offset(i as isize)) as *const c_char };
            if params_ptr.is_null() { info!("params ptr is null!"); return; }

            let params_c_str = unsafe { CStr::from_ptr(params_ptr) };
            match params_c_str.to_str() {
                Ok(s) => bakkesmod::log(&format!("param: {}", s)),
                Err(_) => bakkesmod::log("params null")
            };
        }
    }));
    // bakkesmod::register_notifier(Rc::clone(&plugin_rc), "rust_demolish", Box::new(rust_demolish));
    bakkesmod::register_notifier("rust_demolish", Box::new(rust_demolish));

    info!("finished initialization");
    // Box::into_raw(plugin)
}
