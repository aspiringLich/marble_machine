pub mod body;
pub use body::*;

pub mod common_impl;
pub use common_impl::*;

pub mod header;
pub use header::*;

pub mod standard;
pub use standard::*;

use bevy::prelude::Component;

trait_enum::trait_enum! {
    #[derive(Copy, Clone, Component)]
    pub enum ModuleType: Module {
        Basic,
    }
}