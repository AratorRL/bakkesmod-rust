#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
#[macro_use]
extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;
use bakkesmod_macros::plugin_init;
#[macro_use]
mod bakkesmod {
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::atomic::AtomicBool;
    use std::sync::Mutex;
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;
    use super::wrappers::CarWrapper;
    static mut BAKKESMOD: &dyn BakkesMod = &Dummy;
    pub fn bakkesmod_init(id: u64) {
        let bm_wrapper = Box::new(BakkesModWrapper {
            id,
            callbacks: Mutex::new(Vec::new()),
        });
        unsafe {
            BAKKESMOD = Box::leak(bm_wrapper);
        }
    }
    pub fn bakkesmod() -> &'static dyn BakkesMod {
        unsafe { BAKKESMOD }
    }
    type NotifierCallback = dyn FnMut(Vec<String>);
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
        fn get_local_car(&self) -> Option<CarWrapper> {
            None
        }
        fn register_notifier(&self, name: &str, callback: Box<NotifierCallback>) {}
        fn call_callback(&self, addr: usize, params: *const *const c_char, len: u32) {}
    }
    struct BakkesModWrapper {
        pub id: u64,
        pub callbacks: Mutex<Vec<usize>>,
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
                            "rustplugin::bakkesmod",
                            "rustplugin::bakkesmod",
                            "src\\bakkesmod.rs",
                            79u32,
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
                            "rustplugin::bakkesmod",
                            "rustplugin::bakkesmod",
                            "src\\bakkesmod.rs",
                            87u32,
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
                            "rustplugin::bakkesmod",
                            "rustplugin::bakkesmod",
                            "src\\bakkesmod.rs",
                            116u32,
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
                            "rustplugin::bakkesmod",
                            "rustplugin::bakkesmod",
                            "src\\bakkesmod.rs",
                            117u32,
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
                                "rustplugin::bakkesmod",
                                "rustplugin::bakkesmod",
                                "src\\bakkesmod.rs",
                                119u32,
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
                                "rustplugin::bakkesmod",
                                "rustplugin::bakkesmod",
                                "src\\bakkesmod.rs",
                                122u32,
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
                                    "rustplugin::bakkesmod",
                                    "rustplugin::bakkesmod",
                                    "src\\bakkesmod.rs",
                                    128u32,
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
                                        "rustplugin::bakkesmod",
                                        "rustplugin::bakkesmod",
                                        "src\\bakkesmod.rs",
                                        133u32,
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
        bakkesmod().call_callback(user_data, params, len);
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
        fn GetLocalCar() -> usize;
    }
}
pub mod wrappers {
    pub struct CarWrapper(pub usize);
    pub trait Car {
        fn demolish(&self);
    }
    impl Car for CarWrapper {
        fn demolish(&self) {
            unsafe {
                Car_Demolish(self.0);
            }
        }
    }
    extern "C" {
        fn Car_Demolish(p_car: usize);
    }
}
use wrappers::Car;
fn rust_demolish(params: Vec<String>) {
    match bakkesmod::get_local_car() {
        Some(car) => car.demolish(),
        None => bakkesmod::log("Car is NULL"),
    }
}
struct Plugin {
    my_data: u32,
}
pub fn on_load() {
    let plugin = Rc::new(RefCell::new(Plugin { my_data: 1337 }));
    {
        crate::bakkesmod::log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["henlo from rust"],
                &match () {
                    () => [],
                },
            ));
            res
        });
    };
    let plugin_ref = Rc::clone(&plugin);
    let cb: Box<dyn FnMut(Vec<String>)> = Box::new(move |params: Vec<String>| {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["Hello from a callback!"],
                        &match () {
                            () => [],
                        },
                    ),
                    lvl,
                    &("rustplugin", "rustplugin", "src\\lib.rs", 41u32),
                );
            }
        };
        bakkesmod::log("henlo from a rust callback!");
        let my_data = plugin_ref.borrow().my_data;
        bakkesmod::log(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["my data = "],
                &match (&my_data,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
            res
        });
    });
    bakkesmod::register_notifier("rust_callback", cb);
    bakkesmod::register_notifier(
        "rust_notifier",
        Box::new(move |params: Vec<String>| {
            bakkesmod::log("this is the callback for rust_notifier!");
            bakkesmod::log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
                    &["params = "],
                    &match (&params,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
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
                ));
                res
            });
            {
                crate::bakkesmod::log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(
                        &["params = "],
                        &match (&params,) {
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
                    ));
                    res
                });
            };
            for param in params {
                bakkesmod::log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["param: "],
                        &match (&param,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                });
            }
        }),
    );
    bakkesmod::register_notifier(
        "rust_demolish",
        Box::new(move |params: Vec<String>| {
            match bakkesmod::get_local_car() {
                Some(car) => car.demolish(),
                None => {
                    crate::bakkesmod::log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Car is NULL"],
                            &match () {
                                () => [],
                            },
                        ));
                        res
                    });
                }
            };
        }),
    );
}
#[no_mangle]
pub extern "C" fn InitPlugin(id: u64) {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("rustplugin.log").unwrap(),
    );
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["Hello from a Rust plugin!"],
                    &match () {
                        () => [],
                    },
                ),
                lvl,
                &("rustplugin", "rustplugin", "src\\lib.rs", 29u32),
            );
        }
    };
    bakkesmod::bakkesmod_init(id);
    on_load();
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["finished initialization"],
                    &match () {
                        () => [],
                    },
                ),
                lvl,
                &("rustplugin", "rustplugin", "src\\lib.rs", 29u32),
            );
        }
    };
}
