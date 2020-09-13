use bakkesmod::prelude::*;
use bakkesmod::wrappers::unreal::*;
use bakkesmod::{game, console};

#[plugin_init]
pub fn on_load() {
    console::register_notifier("set_ball_location", Box::new(move |_: Vec<String>| {
        let game = match game::get_game_event_as_server() {
            Some(g) => g,
            None => {
                log_console!("game is null!");
                return;
            }
        };
        
        match game.get_ball() {
            Some(ball) => log_console!("{}", ball.get_location()),
            None => log_console!("ball is null")
        };
    }));
}