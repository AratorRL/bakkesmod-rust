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
use bakkesmod::Plugin;
pub mod wrappers;
use wrappers::Car;

fn notifier_callback(plugin: Rc<RefCell<Plugin>>, _: usize, _: u32) {
    // let plugin: Plugin = unsafe { *Box::from_raw(user_data as * mut Plugin) };
    let plugin = plugin.borrow_mut();
    plugin.log("this is the callback for rust_notifier!");
    // plugin.log(&format!("user_data = {:x?}", user_data));
}

fn rust_demolish(plugin: Rc<RefCell<Plugin>>, _: usize, _: u32) {
    // let plugin: Plugin = unsafe { *Box::from_raw(user_data as * mut Plugin) };
    let plugin = plugin.borrow_mut();
    match plugin.get_local_car() {
        Some(car) => car.demolish(),
        None => plugin.log("Car is NULL")
    }
}

#[no_mangle]
pub extern "C" fn InitPlugin(id: u64) -> *mut Plugin {
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("rustplugin.log").unwrap());
    info!("Hello from a Rust plugin!");

    let mut plugin_obj = Plugin {
        id,
        callbacks: HashMap::new()
    };
    let mut plugin_rc = Rc::new(RefCell::new(plugin_obj));
    let mut plugin = plugin_rc.borrow_mut();

    plugin.log("henlo from rust");
    // bakkesmod::register_notifier("rust_notifier", notifier_callback);
    // plugin.register_notifier("rust_notifier", |plugin: Plugin, params: usize, len: u32| {
    //     // let plugin: Plugin = unsafe { *Box::from_raw(user_data as * mut Plugin) };
    //     plugin.log("this is the callback for rust_notifier!");
    //     // plugin.log(&format!("user_data = {:x?}", user_data));
    //     plugin.log(&format!("params = {:x?}", params));

    //     if len <= 0 { return; }

    //     let params_ptr_ptr = params as *const *const c_char;
    //     if params_ptr_ptr.is_null() { info!("ptr to params ptr is null!"); return; }

    //     for i in 0..len {
    //         // let params_ptr = params_ptr_ptr.offset(i);
    //         let params_ptr = unsafe { *(params_ptr_ptr.offset(i as isize)) as *const c_char };
    //         if params_ptr.is_null() { info!("params ptr is null!"); return; }

    //         let params_c_str = unsafe { CStr::from_ptr(params_ptr) };
    //         match params_c_str.to_str() {
    //             Ok(s) => plugin.log(&format!("param: {}", s)),
    //             Err(_) => plugin.log("params null")
    //         };
    //     }
    // });
    bakkesmod::register_notifier(Rc::clone(&plugin_rc), "rust_demolish", Box::new(rust_demolish));
    bakkesmod::register_notifier(Rc::clone(&plugin_rc), "rust_demolish", Box::new(rust_demolish));

    info!("finished initialization");
    Box::into_raw(plugin)
}
