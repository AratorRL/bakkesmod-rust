use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use std::ffi::CString;
use std::os::raw::c_char;

use super::wrappers::CarWrapper;

static mut BAKKESMOD: &dyn BakkesMod = &Dummy;
// static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn bakkesmod_init(id: u64) {
    let bm_wrapper = Box::new(BakkesModWrapper {
        id,
        callbacks: Mutex::new(Vec::new())
    });
    unsafe { BAKKESMOD = Box::leak(bm_wrapper); }
}

pub fn bakkesmod() -> &'static dyn BakkesMod {
    unsafe { BAKKESMOD }
}

// type NotifierCallback = Box<dyn FnMut(usize, u32)>;
type NotifierCallback = dyn FnMut(usize, u32);

pub fn log(text: &str) {
    bakkesmod().log(text);
}

pub fn register_notifier(name: &str, callback: Box<NotifierCallback>) {
    bakkesmod().register_notifier(name, callback);
}

pub fn get_local_car() -> Option<CarWrapper> {
    bakkesmod().get_local_car()
}

pub trait BakkesMod {
    fn log(&self, text: &str);
    fn get_local_car(&self) -> Option<CarWrapper>;
    fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>);
    fn call_callback(&self, addr: usize, params: *const *const c_char, len: u32);
}

struct Dummy;

impl BakkesMod for Dummy {
    fn log(&self, text: &str) {}
    fn get_local_car(&self) -> Option<CarWrapper> { None }
    fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>) {}
    fn call_callback(&self, addr: usize, params: *const *const c_char, len: u32) {}
}

struct BakkesModWrapper {
    pub id: u64,
    pub callbacks: Mutex<Vec<usize>>
}

impl BakkesMod for BakkesModWrapper {
    fn log(&self, text: &str) {
        info!("trying to log \"{}\"", text);
        let id = self.id;
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { LogConsole(id, c_text); }
    }

    fn get_local_car(&self) -> Option<CarWrapper> {
        info!("calling get_local_car()");
        let p_car = unsafe { GetLocalCar() };
        match p_car {
            0 => None,
            _ => Some(CarWrapper(p_car))
        }
    }

    fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>) {
        let callback = Box::new(callback);
        let addr = Box::into_raw(callback) as usize;

        {
            let mut callbacks = self.callbacks.lock().unwrap();
            callbacks.push(addr);
        }

        let id = self.id;
        let c_name = CString::new(name).unwrap();
        let c_name: *const c_char = c_name.as_ptr();

        let c_callback = notifier_callback as usize;
        let user_data = addr;

        let c_description: *const c_char = CString::new("").unwrap().as_ptr();
        unsafe { RegisterNotifier(id, user_data, c_name, c_callback, c_description, 0) }
    }

    fn call_callback(&self, addr: usize, params: *const *const c_char, len: u32) {
        let mut closure = unsafe { Box::from_raw(addr as *mut Box<NotifierCallback>) };
        closure(params as usize, len);
        let _ = Box::into_raw(closure);
    }
}

extern "C" fn notifier_callback(user_data: usize, params: *const *const c_char, len: u32) {
    info!("callback called!");
    info!("trying to find rust callback...");
    info!("user data: {:x?}", user_data);
    
    bakkesmod().call_callback(user_data, params, len);

    // let callback: fn(usize, u32) = unsafe { std::mem::transmute::<usize, fn(usize, u32)>(user_data) };
    // unsafe {
    //     let c_buf: *mut c_char = *params as *mut c_char;
    //     let c_str: CString = CString::from_raw(c_buf);
    //     let name = c_str.to_str().unwrap();
    //     info!("name = {}", name);
    // };
    // match get_global_struct().find_callback(name)
    // register_notifier("hello", { 
    //     fn inner_func(_: usize, _: u32) {}
    //     inner_func
    // });
}
    
extern "C" {
    fn LogConsole(id: u64, text: *const c_char);
    fn RegisterNotifier(id: u64, user_data: usize, cvar: *const c_char, callback: usize, description: *const c_char, permissions: u8);

    fn GetLocalCar() -> usize;
}