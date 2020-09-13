# Rust SDK for BakkesMod plugins
<a href="https://crates.io/crates/bakkesmod"><img src="https://img.shields.io/crates/v/bakkesmod.svg" alt="Crates.io version" /></a>
<a href="https://docs.rs/bakkesmod"><img src="https://img.shields.io/badge/docs-latest-blue.svg" alt="docs.rs docs" /></a>

[Go to the documentation](https://docs.rs/bakkesmod)

## Example
```rust
use bakkesmod::prelude::*;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::{game, console};

#[plugin_init]
pub fn on_load() {
    console::register_notifier("get_ball_location", Box::new(move |_: Vec<String>| {
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

View more examples in the [examples directory](https://github.com/AratorRL/bakkesmod-rust/tree/master/examples).