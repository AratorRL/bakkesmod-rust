#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]

#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

mod bakkesmod;
pub use bakkesmod::*;

#[macro_use] pub mod wrappers;
pub use wrappers::*;

mod custom;
pub use custom::*;

pub use bakkesmod_macros::plugin_init;

mod generated;
pub use generated::*;