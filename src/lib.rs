#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

fn notifier_callback() {
    bakkesmod::log("this is the callback for rust_notifier!");
}

#[no_mangle]
pub extern "C" fn InitPlugin(id: u64) {
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("rustplugin.log").unwrap());
    info!("Hello from a Rust plugin!");

    bakkesmod::set_id(id);

    bakkesmod::log("henlo from rust");
    bakkesmod::register_notifier("rust_notifier", notifier_callback);

    info!("finished initialization");
}

mod bakkesmod {
    static mut GLOBAL_ID: u64 = 0;

    pub fn set_id(id: u64) {
        unsafe { GLOBAL_ID = id; }
    }

    pub fn get_id() -> u64 {
        unsafe { GLOBAL_ID }
    }

    use std::ffi::CString;
    use std::os::raw::c_char;

    pub fn log(text: &str) {
        info!("trying to log...");
        let id = get_id();
        info!("id: {}", id);
        let c_text: *const c_char = CString::new(text).unwrap().as_ptr();
        info!("c_text: {:?}", c_text);
        unsafe { LogConsole(id, c_text); }
    }

    pub fn register_notifier(name: &str, callback: fn() -> ()) {
        let id = get_id();
        let c_name: *const c_char = CString::new(name).unwrap().as_ptr();
        let c_callback = callback as usize;
        let c_description: *const c_char = CString::new("").unwrap().as_ptr();
        unsafe { RegisterNotifier(id, c_name, c_callback, c_description, 0) }
    }
    
    extern "C" {
        // fn BakkesModAPI_BM_GetLocalCar() -> usize;
        fn LogConsole(id: u64, text: *const c_char);
        fn RegisterNotifier(id: u64, cvar: *const c_char, callback: usize, description: *const c_char, permissions: u8);
    }
}