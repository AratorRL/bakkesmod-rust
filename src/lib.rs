#[macro_use] pub mod macros;

mod bakkesmod;
pub use bakkesmod::*;

#[macro_use] pub mod wrappers;

pub mod prelude {
    pub use {log_console, vec2, vec3};
    pub use crate::wrappers::*;
    pub use bakkesmod_macros::plugin_init;
}