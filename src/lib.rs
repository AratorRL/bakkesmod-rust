//! SDK for writing plugins for [BakkesMod](https://bakkesmod.com), a mod framework for Rocket League.
//! 
//! This SDK is an alternative to the C++ SDK found [here](https://github.com/bakkesmodorg/BakkesModSDK),
//! allowing to write plugins in Rust. Plugins built with the Rust SDK can be loaded into the game
//! in the exact same way as C++ plugins.
//! 
//! Note however that not all functionality in the C++ SDK is available here yet, and **importantly,
//! the Rust SDK has had very little testing, so some things might be completely broken**.
//! 
//! ### Prerequisites
//! Make sure you have installed [Rust](https://www.rust-lang.org/tools/install) and [BakkesMod](https://bakkesmod.com).
//! 
//! Also, add an environment variable called `BAKKESMOD_LIB_PATH` containing the path to `pluginsdk.lib` (e.g. `C:\Program Files (x86)\Steam\steamapps\common\rocketleague\Binaries\Win64\bakkesmod\bakkesmodsdk\lib`).
//! 
//! 
//! ### Create the project
//! Create a new Rust library project with `cargo new --lib <pluginname>`.
//! 
//! Add the following to the generated `Cargo.toml`:
//! ```toml
//! [dependencies]
//! bakkesmod = "0.2"
//! 
//! [lib]
//! name = "mycoolplugin"
//! crate_type = ["cdylib"]
//! ```
//! 
//! Write your plugin code in `src/lib.rs` (and possibly add more files).
//! Make sure you have exactly one function with the `#[plugin_init]` attribute. This function will be called when the plugin is loaded.
//! 
//! ### Building
//! Use `cargo build` or `cargo build --release` to build. A `<pluginname>.dll` file is created in `target/debug` or `target/release`.
//! Copy this file to your `bakkesmod/plugins` folder. It can now be loaded in-game with `plugin load <pluginname>`.
//! 
//! 
//! # Guide
//! 
//! See also [the examples directory](https://github.com/AratorRL/bakkesmod-rust/tree/master/examples) on GitHub for some examples.
//! 
//! BakkesMod plugins are loaded in-game using the `plugin load <pluginname>` command in the console, or automatically when listed in
//! the `plugins.cfg` file. When a plugin is loaded, the function with the `plugin_init` attribute is executed. It makes sense to
//! call this function something like `on_load`.
//! After this, plugin code is only executed through callbacks, which can trigger when e.g. a game event happens, or a console command
//! is executed. Typically, the `on_load` function will register these callbacks, and the rest of the plugin's execution happens
//! inside these callbacks.
//! 
//!  As an example:
//! ```rust
//! use bakkesmod::{game, console};
//! 
//! #[plugin_init]
//! pub fn on_load() {
//!     // register a 'notifier' (a console command)
//!     console::register_notifier("my_notifier", Box::new(move |params: Vec<String>| {
//!         // callback code
//!     }));
//! 
//!     // register a hook on a game event
//!     game::hook_event("Function Engine.GameViewportClient.Tick", Box::new(move || {
//!         // callback code
//!     }));
//! 
//!     // use a normal function instead of a lambda
//!     game::hook_event("Function TAGame.Car_TA.ApplyBallImpactForces", Box::new(my_callback));
//! }
//! 
//! fn my_callback() {
//!     // callback code
//! }
//! ```
//!
//! BakkesMod-specific functionality like hooking, registering notifiers or drawables, is found in the
//! `game` and `console` modules.
//! 
//! The `wrappers::unreal` module contains auto-generated wrappers around game classes, that are written in unrealscript.
//! Since Rust doesn't have struct inheritance, and since BakkesMod provides only methods on these classes anyway,
//! *traits* are used to model functionality on these wrappers.
//! 
//! This means that each game object is represented as simple `XXXWrapper` struct, without any methods.
//! Methods on these wrappers are implemented as part of traits, with names `XXX` (without Wrapper).
//! For example, the `CarWrapper` struct implementents the `Car` trait, which allows you to call e.g. `demolish`
//! (defined in `Car`) on a `CarWrapper` instance. `CarWrapper` also implements the `Vehicle`, `RBActor` and `Actor` traits,
//! so all methods defined in those traits are also callable on a `CarWrapper` instance. This is how the unrealscript
//! inheritance is mimicked.
//! 
//! It's also recommended to import the `prelude` module (`use bakkesmod::prelude::*;`)
//! so that all macros are brought into scope.
//! Macros include `log_console!` (print to the in-game console using the same formatting as Rust's `println!`),
//! and `vec3!` and `vec2!` for creating vectors.
//! 


#[macro_use] mod macros;
#[macro_use] mod errors;

pub mod game;
pub mod console;
pub mod wrappers;

mod internal;

pub mod prelude {
    pub use {log_console, vec2, vec3, color, lin_color};
    pub use bakkesmod_error;
    pub use crate::console::console_print;
    pub use crate::internal::{bakkesmod_init, bakkesmod_exit};
    pub use crate::wrappers::*;
    pub use bakkesmod_macros::plugin_init;
}