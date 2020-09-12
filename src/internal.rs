use std::sync::Mutex;

use crate::wrappers::canvas::Canvas;
use crate::wrappers::cvar::CVar;

pub type HookCallback = dyn FnMut();
pub type HookWithCallerCallback<T> = dyn FnMut(Box<T>);
pub type HookWithCallerCallbackInternal = dyn FnMut(usize, usize);
pub type DrawableCallback = dyn FnMut(Canvas);
pub type TimeoutCallback = dyn FnMut();

pub type NotifierCallback = dyn FnMut(Vec<String>);
pub type OnValueChangedCallback = dyn FnMut(String, CVar);

pub trait BakkesMod {
    fn id(&self) -> u64;
    fn add_notifier_callback(&self, addr: usize);
    fn add_on_value_changed_callback(&self, addr: usize);
    fn add_hook_callback(&self, addr: usize);
    fn add_hook_caller_callback(&self, addr: usize);
    fn add_drawable_callback(&self, addr: usize);
    fn add_timeout_callback(&self, addr: usize);

    fn get_notifier_callbacks(&self) -> Vec<usize>;
    fn get_on_value_changed_callbacks(&self) -> Vec<usize>;
    fn get_hook_callbacks(&self) -> Vec<usize>;
    fn get_hook_caller_callbacks(&self) -> Vec<usize>;
    fn get_drawable_callbacks(&self) -> Vec<usize>;
    fn get_timeout_callbacks(&self) -> Vec<usize>;
}

static mut BAKKESMOD: &dyn BakkesMod = &Dummy;

pub fn bakkesmod_init(id: u64) {
    let bm_wrapper = Box::new(BakkesModWrapper {
        id,
        notifier_callbacks: Mutex::new(Vec::new()),
        on_value_changed_callbacks: Mutex::new(Vec::new()),
        hook_callbacks: Mutex::new(Vec::new()),
        hook_caller_callbacks: Mutex::new(Vec::new()),
        drawable_callbacks: Mutex::new(Vec::new()),
        timeout_callbacks: Mutex::new(Vec::new()),
    });
    unsafe { BAKKESMOD = Box::leak(bm_wrapper); }
}

pub fn bakkesmod_exit() {
    drop_notifier_callbacks();
    drop_on_value_changed_callbacks();
    drop_hook_callbacks();
    drop_hook_caller_callbacks();
    drop_drawable_callbacks();
    drop_timeout_callbacks();
}

pub fn bakkesmod() -> &'static dyn BakkesMod {
    unsafe { BAKKESMOD }
}


struct BakkesModWrapper {
    pub id: u64,
    pub notifier_callbacks: Mutex<Vec<usize>>,
    pub on_value_changed_callbacks: Mutex<Vec<usize>>,
    pub hook_callbacks: Mutex<Vec<usize>>,
    pub hook_caller_callbacks: Mutex<Vec<usize>>,
    pub drawable_callbacks: Mutex<Vec<usize>>,
    pub timeout_callbacks: Mutex<Vec<usize>>,
}

impl BakkesMod for BakkesModWrapper {
    fn id(&self) -> u64 {
        self.id
    }

    fn add_notifier_callback(&self, addr: usize) {
        let mut callbacks = self.notifier_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_on_value_changed_callback(&self, addr: usize) {
        let mut callbacks = self.on_value_changed_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_hook_callback(&self, addr: usize) {
        let mut callbacks = self.hook_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_hook_caller_callback(&self, addr: usize) {
        let mut callbacks = self.hook_caller_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_drawable_callback(&self, addr: usize) {
        let mut callbacks = self.drawable_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn add_timeout_callback(&self, addr: usize) {
        let mut callbacks = self.timeout_callbacks.lock().unwrap();
        callbacks.push(addr);
    }

    fn get_notifier_callbacks(&self) -> Vec<usize> {
        let callbacks = self.notifier_callbacks.lock().unwrap();
        callbacks.clone()
    }

    fn get_on_value_changed_callbacks(&self) -> Vec<usize> {
        let callbacks = self.on_value_changed_callbacks.lock().unwrap();
        callbacks.clone()
    }

    fn get_hook_callbacks(&self) -> Vec<usize> {
        let callbacks = self.hook_callbacks.lock().unwrap();
        callbacks.clone()
    }

    fn get_hook_caller_callbacks(&self) -> Vec<usize> {
        let callbacks = self.hook_caller_callbacks.lock().unwrap();
        callbacks.clone()
    }

    fn get_drawable_callbacks(&self) -> Vec<usize> {
        let callbacks = self.drawable_callbacks.lock().unwrap();
        callbacks.clone()
    }

    fn get_timeout_callbacks(&self) -> Vec<usize> {
        let callbacks = self.timeout_callbacks.lock().unwrap();
        callbacks.clone()
    }
}

fn drop_hook_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_hook_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<HookCallback>) };
    }
}

pub fn drop_hook_caller_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_hook_caller_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<HookWithCallerCallbackInternal>) };
    }
}

pub fn drop_drawable_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_drawable_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<DrawableCallback>) };
    }
}

pub fn drop_timeout_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_timeout_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<TimeoutCallback>) };
    }
}

pub fn drop_notifier_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_notifier_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<NotifierCallback>) };
    }
}

pub fn drop_on_value_changed_callbacks() {
    let bm = bakkesmod();
    let notifiers = bm.get_on_value_changed_callbacks();
    for addr in notifiers {
        let _ = unsafe { Box::from_raw(addr as *mut Box<OnValueChangedCallback>) };
    }
}

struct Dummy;

impl BakkesMod for Dummy {
    fn id(&self) -> u64 { 0 }
    fn add_notifier_callback(&self, _: usize) {}
    fn add_on_value_changed_callback(&self, _: usize) {}
    fn add_hook_callback(&self, _: usize) {}
    fn add_hook_caller_callback(&self, _: usize) {}
    fn add_drawable_callback(&self, _: usize) {}
    fn add_timeout_callback(&self, _: usize) {}

    fn get_notifier_callbacks(&self) -> Vec<usize> { Vec::new() }
    fn get_on_value_changed_callbacks(&self) -> Vec<usize> { Vec::new() }
    fn get_hook_callbacks(&self) -> Vec<usize> { Vec::new() }
    fn get_hook_caller_callbacks(&self) -> Vec<usize> { Vec::new() }
    fn get_drawable_callbacks(&self) -> Vec<usize> { Vec::new() }
    fn get_timeout_callbacks(&self) -> Vec<usize> { Vec::new() }
}