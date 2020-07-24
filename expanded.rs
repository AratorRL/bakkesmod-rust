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
use std::rc::Rc;
use std::cell::RefCell;
#[macro_use]
use bakkesmod::wrappers::*;
use bakkesmod::{vec2, vec3};
use bakkesmod::{self, log_console, plugin_init};
pub fn on_load() {
    bakkesmod::register_notifier(
        "rust_notifier",
        Box::new(move |params: Vec<String>| {
            {
                crate::bakkesmod::log(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["this is the callback for rust_notifier!"],
                        &match () {
                            () => [],
                        },
                    ));
                    res
                });
            };
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
        }),
    );
    bakkesmod::register_notifier(
        "rust_demolish",
        Box::new(move |_: Vec<String>| {
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
    bakkesmod::register_notifier(
        "rust_set_loc",
        Box::new(move |_: Vec<String>| {
            match bakkesmod::get_local_car() {
                Some(car) => {
                    let origin = Vector3::new(0.0, 0.0, 0.0);
                    let new_loc = origin + Vector3::new(200.0, 1000.0, 50.0);
                    car.set_location(new_loc);
                }
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
    bakkesmod::register_cvar("rust_cvar");
    let counter_base = Rc::new(RefCell::new(0));
    let counter_ref = Rc::clone(&counter_base);
    bakkesmod::hook_event(
        "Function Engine.GameViewportClient.Tick",
        Box::new(move || {
            let mut counter = counter_ref.borrow_mut();
            *counter += 1;
            if (*counter % 120) == 0 {
                {
                    crate::bakkesmod::log(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["viewport client tick"],
                            &match () {
                                () => [],
                            },
                        ));
                        res
                    });
                };
                match bakkesmod::get_local_car() {
                    Some(car) => {
                        let location = car.get_location();
                        {
                            crate::bakkesmod::log(&{
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &match (&location,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                                res
                            });
                        };
                    }
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
            }
        }),
    );
    bakkesmod::register_drawable(Box::new(move |canvas: Canvas| {
        let top_left = ::bakkesmod::wrappers::Vector2::new(100 as i32, 100 as i32);
        let width = ::bakkesmod::wrappers::Vector2::new(250 as i32, 0 as i32);
        let height = ::bakkesmod::wrappers::Vector2::new(0 as i32, 150 as i32);
        canvas.draw_line(top_left, top_left + width);
        canvas.draw_line(top_left + width, top_left + width + height);
        canvas.draw_line(top_left, top_left + height);
        canvas.draw_line(top_left + height, top_left + width + height);
    }));
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
                &("rustplugin", "rustplugin", "rustplugin\\src\\lib.rs", 15u32),
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
                &("rustplugin", "rustplugin", "rustplugin\\src\\lib.rs", 15u32),
            );
        }
    };
}
