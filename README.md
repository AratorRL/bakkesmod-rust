# Rust SDK for BakkesMod plugins

## Example
```rust
use bakkesmod;

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