#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;
mod bakkesmod {
    #![allow(unused)]
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::atomic::AtomicBool;
    use std::sync::Mutex;
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;
    use super::wrappers::CarWrapper;
    use super::wrappers::Car;
    use super::wrappers::Object;
    static mut BAKKESMOD: &dyn BakkesMod = &Dummy;
    pub fn bakkesmod_init(id: u64) {
        let bm_wrapper = Box::new(BakkesModWrapper {
            id,
            notifier_callbacks: Mutex::new(Vec::new()),
            hook_callbacks: Mutex::new(Vec::new()),
        });
        unsafe {
            BAKKESMOD = Box::leak(bm_wrapper);
        }
    }
    pub fn bakkesmod() -> &'static dyn BakkesMod {
        unsafe { BAKKESMOD }
    }
    type NotifierCallback = dyn FnMut(Vec<String>);
    type HookCallback = dyn FnMut();
    type HookWithCallerCallback<T> = dyn FnMut(Box<T>);
    type HookWithCallerCallbackInternal = dyn FnMut(usize, usize);
    pub fn log(text: &str) {
        bakkesmod().log(text);
    }
    pub fn register_notifier(name: &str, callback: Box<NotifierCallback>) {
        bakkesmod().register_notifier(name, callback);
    }
    pub fn register_cvar(name: &str) {
        bakkesmod().register_cvar(name);
    }
    pub fn hook_event(name: &str, callback: Box<HookCallback>) {
        bakkesmod().register_hook(name, callback);
    }
    pub fn hook_event_with_caller<T: Object + 'static>(
        name: &str,
        mut callback: Box<HookWithCallerCallback<T>>,
    ) {
        let wrapper_callback = Box::new(move |caller: usize, params: usize| {
            let obj_wrapper = T::new(caller);
            callback(Box::new(obj_wrapper));
        });
        bakkesmod().register_hook_with_caller(name, wrapper_callback);
    }
    pub fn get_local_car() -> Option<CarWrapper> {
        bakkesmod().get_local_car()
    }
    pub trait BakkesMod {
        fn log(&self, text: &str);
        fn get_local_car(&self) -> Option<CarWrapper>;
        fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>);
        fn register_cvar(&self, name: &str);
        fn call_notifier_callback(&self, addr: usize, params: *const *const c_char, len: u32);
        fn register_hook(&self, name: &str, callback: Box<HookCallback>);
        fn call_hook_callback(&self, addr: usize);
        fn register_hook_with_caller(
            &self,
            name: &str,
            callback: Box<HookWithCallerCallbackInternal>,
        );
        fn call_hook_with_caller_callback(&self, addr: usize, caller: usize, params: usize);
    }
    struct Dummy;
    impl BakkesMod for Dummy {
        fn log(&self, text: &str) {}
        fn get_local_car(&self) -> Option<CarWrapper> {
            None
        }
        fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>) {}
        fn register_cvar(&self, name: &str) {}
        fn call_notifier_callback(&self, addr: usize, params: *const *const c_char, len: u32) {}
        fn register_hook(&self, name: &str, callback: Box<HookCallback>) {}
        fn call_hook_callback(&self, addr: usize) {}
        fn register_hook_with_caller(
            &self,
            name: &str,
            callback: Box<HookWithCallerCallbackInternal>,
        ) {
        }
        fn call_hook_with_caller_callback(&self, addr: usize, caller: usize, params: usize) {}
    }
    struct BakkesModWrapper {
        pub id: u64,
        pub notifier_callbacks: Mutex<Vec<usize>>,
        pub hook_callbacks: Mutex<Vec<usize>>,
    }
    impl BakkesMod for BakkesModWrapper {
        fn log(&self, text: &str) {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["trying to log \"", "\""],
                            &match (&text,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            115u32,
                        ),
                    );
                }
            };
            let id = self.id;
            let c_text = CString::new(text).unwrap();
            let c_text: *const c_char = c_text.as_ptr();
            unsafe {
                LogConsole(id, c_text);
            }
        }
        fn get_local_car(&self) -> Option<CarWrapper> {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["calling get_local_car()"],
                            &match () {
                                () => [],
                            },
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            123u32,
                        ),
                    );
                }
            };
            let p_car = unsafe { GetLocalCar() };
            match p_car {
                0 => None,
                _ => Some(CarWrapper(p_car)),
            }
        }
        fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>) {
            let callback = Box::new(callback);
            let addr = Box::into_raw(callback) as usize;
            {
                let mut notifier_callbacks = self.notifier_callbacks.lock().unwrap();
                notifier_callbacks.push(addr);
            }
            let id = self.id;
            let c_name = CString::new(name).unwrap();
            let c_name: *const c_char = c_name.as_ptr();
            let c_callback = notifier_callback as usize;
            let user_data = addr;
            let c_desc = CString::new("").unwrap();
            let c_desc: *const c_char = c_desc.as_ptr();
            unsafe {
                RegisterNotifier(id, user_data, c_name, c_callback, c_desc, 0);
            }
        }
        fn register_hook(&self, name: &str, callback: Box<HookCallback>) {
            let callback = Box::new(callback);
            let addr = Box::into_raw(callback) as usize;
            {
                let mut hook_callbacks = self.hook_callbacks.lock().unwrap();
                hook_callbacks.push(addr);
            }
            let id = self.id;
            let c_name = CString::new(name).unwrap();
            let c_name: *const c_char = c_name.as_ptr();
            let c_callback = hook_callback as usize;
            let user_data = addr;
            unsafe {
                HookEvent(id, user_data, c_name, c_callback);
            }
        }
        fn call_hook_callback(&self, addr: usize) {
            let mut closure = unsafe { Box::from_raw(addr as *mut Box<HookCallback>) };
            closure();
            let _ = Box::into_raw(closure);
        }
        fn register_hook_with_caller(
            &self,
            name: &str,
            callback: Box<HookWithCallerCallbackInternal>,
        ) {
            let callback = Box::new(callback);
            let addr = Box::into_raw(callback) as usize;
            {
                let mut hook_callbacks = self.hook_callbacks.lock().unwrap();
                hook_callbacks.push(addr);
            }
            let id = self.id;
            let c_name = CString::new(name).unwrap();
            let c_name: *const c_char = c_name.as_ptr();
            let c_callback = hook_with_caller_callback as usize;
            let user_data = addr;
            unsafe {
                HookEventWithCaller(id, user_data, c_name, c_callback);
            }
        }
        fn call_hook_with_caller_callback(&self, addr: usize, caller: usize, params: usize) {
            let mut closure =
                unsafe { Box::from_raw(addr as *mut Box<HookWithCallerCallbackInternal>) };
            closure(caller, params);
            let _ = Box::into_raw(closure);
        }
        fn register_cvar(&self, name: &str) {
            let id = self.id;
            let c_name = CString::new(name).unwrap();
            let c_name: *const c_char = c_name.as_ptr();
            let c_defval = CString::new("").unwrap();
            let c_defval: *const c_char = c_defval.as_ptr();
            let c_desc = CString::new("").unwrap();
            let c_desc: *const c_char = c_desc.as_ptr();
            unsafe {
                RegisterCvar(
                    id, c_name, c_defval, c_desc, true, false, 0.0, false, 0.0, false,
                );
            }
        }
        fn call_notifier_callback(&self, addr: usize, params: *const *const c_char, len: u32) {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["callback called!"],
                            &match () {
                                () => [],
                            },
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            215u32,
                        ),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &["user data: "],
                            &match (&addr,) {
                                (arg0,) => {
                                    [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                                }
                            },
                            &[::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 16u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            }],
                        ),
                        lvl,
                        &(
                            "bakkesmod::bakkesmod",
                            "bakkesmod::bakkesmod",
                            "src\\bakkesmod.rs",
                            216u32,
                        ),
                    );
                }
            };
            if len <= 0 {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["callback called but len is <= 0 !"],
                                &match () {
                                    () => [],
                                },
                            ),
                            lvl,
                            &(
                                "bakkesmod::bakkesmod",
                                "bakkesmod::bakkesmod",
                                "src\\bakkesmod.rs",
                                218u32,
                            ),
                        );
                    }
                };
                return;
            }
            let params_ptr_ptr = params as *const *const c_char;
            if params_ptr_ptr.is_null() {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["ptr to params ptr is null!"],
                                &match () {
                                    () => [],
                                },
                            ),
                            lvl,
                            &(
                                "bakkesmod::bakkesmod",
                                "bakkesmod::bakkesmod",
                                "src\\bakkesmod.rs",
                                221u32,
                            ),
                        );
                    }
                };
                return;
            }
            let mut rust_params: Vec<String> = Vec::new();
            for i in 0..len {
                let params_ptr = unsafe { *(params_ptr_ptr.offset(i as isize)) as *const c_char };
                if params_ptr.is_null() {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["params ptr is null!"],
                                    &match () {
                                        () => [],
                                    },
                                ),
                                lvl,
                                &(
                                    "bakkesmod::bakkesmod",
                                    "bakkesmod::bakkesmod",
                                    "src\\bakkesmod.rs",
                                    227u32,
                                ),
                            );
                        }
                    };
                    return;
                }
                let params_c_str = unsafe { CStr::from_ptr(params_ptr) };
                match params_c_str.to_str() {
                    Ok(s) => rust_params.push(String::from(s)),
                    Err(_) => {
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["params null"],
                                        &match () {
                                            () => [],
                                        },
                                    ),
                                    lvl,
                                    &(
                                        "bakkesmod::bakkesmod",
                                        "bakkesmod::bakkesmod",
                                        "src\\bakkesmod.rs",
                                        232u32,
                                    ),
                                );
                            }
                        };
                        return;
                    }
                };
            }
            let mut closure = unsafe { Box::from_raw(addr as *mut Box<NotifierCallback>) };
            closure(rust_params);
            let _ = Box::into_raw(closure);
        }
    }
    extern "C" fn notifier_callback(user_data: usize, params: *const *const c_char, len: u32) {
        bakkesmod().call_notifier_callback(user_data, params, len);
    }
    extern "C" fn hook_callback(user_data: usize) {
        bakkesmod().call_hook_callback(user_data);
    }
    extern "C" fn hook_with_caller_callback(user_data: usize, caller: usize, params: usize) {
        bakkesmod().call_hook_with_caller_callback(user_data, caller, params);
    }
    extern "C" {
        fn LogConsole(id: u64, text: *const c_char);
        fn RegisterNotifier(
            id: u64,
            user_data: usize,
            cvar: *const c_char,
            callback: usize,
            description: *const c_char,
            permissions: u8,
        );
        fn RegisterCvar(
            id: u64,
            cvar: *const c_char,
            default_value: *const c_char,
            desc: *const c_char,
            searchable: bool,
            has_min: bool,
            min: f32,
            has_max: bool,
            max: f32,
            save_to_cfg: bool,
        );
        fn HookEvent(id: u64, user_data: usize, event_name: *const c_char, callback: usize);
        fn HookEventWithCaller(
            id: u64,
            user_data: usize,
            event_name: *const c_char,
            callback: usize,
        );
        fn GetLocalCar() -> usize;
    }
}
pub use bakkesmod::*;
pub mod wrappers {
    #![allow(unused)]
    pub trait Object {
        fn new(addr: usize) -> Self;
    }
    pub trait Car {
        fn demolish(&self);
    }
    impl Object for CarWrapper {
        fn new(addr: usize) -> Self {
            Self(addr)
        }
    }
    pub struct CarWrapper(pub usize);
    impl Car for CarWrapper {
        fn demolish(&self) {
            unsafe {
                Car_Demolish(self.0);
            }
        }
    }
    extern "C" {
        fn Car_Demolish(p_car: usize);
        fn Actor_GetLocation(p_actor: usize) -> Vector;
        fn Actor_SetLocation(p_actor: usize, new_loc: Vector);
    }
    #[repr(C)]
    pub struct Vector {
        x: f32,
        y: f32,
        z: f32,
    }
}
pub use bakkesmod_macros::plugin_init;
