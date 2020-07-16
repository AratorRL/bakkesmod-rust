use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use std::ffi::CString;
use std::os::raw::c_char;

// type NotifierCallback = fn(params: usize, len: u32);
type NotifierCallback = dyn Fn(Plugin, usize, u32) -> ();

pub struct Plugin {
    pub id: u64,
    pub callbacks: HashMap<String, Box<NotifierCallback>>
}

use super::wrappers::CarWrapper;

impl Plugin {
    pub fn log(&self, text: &str) {
        info!("trying to log \"{}\"", text);
        let id = self.id;
        let c_text = CString::new(text).unwrap();
        let c_text: *const c_char = c_text.as_ptr();
        unsafe { LogConsole(id, c_text); }
    }

    fn call_callback(&self, name: &str) {
        match self.callbacks.get(name) {
            Some(callback) => callback(Plugin { id: 0, callbacks: HashMap::new() }, 0, 0),
            None => warn!("trying to call callback {}, but not found in map!", name)
        };
    }

    pub fn get_local_car(&self) -> Option<CarWrapper> {
        info!("calling get_local_car()");
        let p_car = unsafe { GetLocalCar() };
        match p_car {
            0 => None,
            _ => Some(CarWrapper(p_car))
        }
    }
}

pub fn register_notifier<F>(plugin: Rc<RefCell<Plugin>>, name: &str, callback: F)
where F: Fn(Rc<RefCell<Plugin>>, usize, u32) + 'static {
    info!("registering name {}", name);
    let plugin_brw = plugin.borrow_mut();
    let id = plugin_brw.id;
    let c_name = CString::new(name).unwrap();
    let c_name: *const c_char = c_name.as_ptr();

    // get_global_struct().add_callback(name, callback);

    let c_callback = notifier_callback as usize;
    // self.callbacks.insert(String::from(name), Box::new(callback));
    // let user_data = 1337 as usize;
    // let user_data = Box::into_raw(self) as usize;
    let user_data = Rc::into_raw(plugin) as usize;
    let c_description: *const c_char = CString::new("").unwrap().as_ptr();
    unsafe { RegisterNotifier(id, user_data, c_name, c_callback, c_description, 0) }
}


extern "C" fn notifier_callback(user_data: usize, params: *const *const c_char, len: u32) {
    info!("callback called!");
    info!("trying to find rust callback...");
    info!("user data: {:x?}", user_data);
    

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