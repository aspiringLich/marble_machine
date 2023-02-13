pub mod body;
pub use body::*;
pub mod header;
pub use header::*;
mod common_impl;

// Standard modules.
pub mod standard;
pub use standard::*;

use crate::*;

trait_enum::trait_enum! {
    #[derive(Copy, Clone, Component)]
    pub enum ModuleType: Module {
        Basic,
    }
}
