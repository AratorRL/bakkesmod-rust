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