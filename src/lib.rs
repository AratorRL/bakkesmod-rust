#[macro_use] mod macros;
#[macro_use] mod errors;

mod bakkesmod;
pub use bakkesmod::*;

#[macro_use] pub mod wrappers;

pub mod prelude {
    pub use {log_console, vec2, vec3, color, lin_color};
    pub use bakkesmod_error;
    pub use crate::wrappers::*;
    pub use bakkesmod_macros::plugin_init;
}