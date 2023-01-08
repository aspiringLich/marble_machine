pub mod body;
pub use body::*;
pub mod header;
pub use header::*;
pub mod standard;
pub use standard::*;

use crate::*;

#[derive(Copy, Clone, Component)]
pub enum ModuleType {
    Basic(standard::Basic),
}

impl Default for ModuleType {
    fn default() -> Self {
        Self::Basic(default())
    }
}

impl ModuleType {
    pub fn get_inner_mut(&mut self) -> &mut impl header::Module {
        match self {
            Self::Basic(x) => x,
        }
    }

    pub const fn get_inner(&self) -> &impl header::Module {
        match self {
            Self::Basic(x) => x,
        }
    }
}
