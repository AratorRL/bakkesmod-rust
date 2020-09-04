# Rust SDK for BakkesMod plugins
<a href="https://crates.io/crates/bakkesmod"><img src="https://img.shields.io/crates/v/bakkesmod.svg" alt="Crates.io version" /></a>

## Example
```rust
use bakkesmod;
use bakkesmod::prelude::*;

#[plugin_init]
pub fn on_load() {
    bakkesmod::register_notifier("get_ball_location", Box::new(move |_: Vec<String>| {
        let game = match bakkesmod::get_game_event_as_server() {
            Some(g) => g,
            None => {
                log_console!("game is null!");
                return;
            }
        };
        
        match game.get_ball() {
            Some(ball) => log_console!("{}", ball.get_location()),
            None => log_console!("ball is NULL")
        };
    }));
}
```

# How to use

## Prerequisites
Make sure you have installed [Rust](https://www.rust-lang.org/tools/install) and [BakkesMod](https://bakkesmod.com).

Also, add an environment variable called `BAKKESMOD_LIB_PATH` containing the path to `pluginsdk.lib` (e.g. `C:\Program Files (x86)\Steam\steamapps\common\rocketleague\Binaries\Win64\bakkesmod\bakkesmodsdk\lib`).


## Write the plugin
Create a new Rust library project with `cargo new --lib <pluginname>`.

Add the following to the generated `Cargo.toml`:
```toml
[dependencies]
bakkesmod = "0.1.0"

[lib]
name = "pluginname"
crate_type = ["cdylib"]
```

Write your plugin code in `src/lib.rs` (and possibly add more files).
Make sure you have exactly one function with the `#[plugin_init]` attribute. This function will be called when the plugin is loaded.

## Building
Use `cargo build` or `cargo build --release` to build. A `<pluginname>.dll` file is created in `target/debug` or `target/release`.
Copy this file to your `bakkesmod/plugins` folder. It can now be loaded in-game with `plugin load <pluginname>`.