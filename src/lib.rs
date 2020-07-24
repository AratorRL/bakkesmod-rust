#![allow(unused)]

#[macro_use] extern crate log;
use simplelog::{WriteLogger, LevelFilter, Config};
use std::fs::File;

mod bakkesmod;
pub use bakkesmod::*;

pub mod wrappers;

pub use bakkesmod_macros::plugin_init;
