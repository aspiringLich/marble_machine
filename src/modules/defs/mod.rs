use std::sync::Arc;

use crate::*;
use strum::IntoEnumIterator;

use crate::modules::*;
use crate::modules::event::ModuleUpdate::*;
use strum_macros::EnumIter;
use serde::{Serialize, Deserialize};

/// basic: asic modules that do standard stuff
pub mod basic;

#[derive(Component)]
pub struct ModuleComponent {
    pub ty: ModuleType,
    pub module: Box<dyn Module<Storage = TableStorage>>,
}

impl ModuleComponent {
    pub fn new(ty: ModuleType) -> Self {
        Self {
            ty,
            module: ty.get_module()
        }
    }
}

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum ModuleType {
    Basic,
}

impl ModuleType {
    fn info(self) -> &'static ModuleInfo {
        unsafe {
            MODULE_INFO.get(self as usize).unwrap_or_else(|| {
                error!("Could not get ModuleInfo for ModuleType! Maybe it is unititialized?");
                &MODULE_INFO[0]
            })
        }
    }
    /// return instructions on spawning this module
    pub fn spawn_instructions(&self) -> &'static SpawnInstructions {
        &self.info().instructions
    }
    /// the name of the module
    pub fn get_name(&self) -> &'static str {
        self.info().name
    }
    /// the identifier of the module
    pub fn get_identifier(&self) -> &'static str {
        self.info().identifier
    }
    /// get the module
    pub fn get_module(&self) -> Box<dyn Module> {
        get_module(*self)
    }
}

pub(super) static mut MODULES: Vec<Box<dyn super::Module>> = vec![];
pub(super) static mut MODULE_INFO: Vec<ModuleInfo> = vec![];

pub fn init_modules() {
    unsafe {
        for module in ModuleType::iter() {
            MODULES.push(get_module(module));
            MODULE_INFO.push(module.get_module().info());
        }
    }
}

fn get_module(module: ModuleType) -> Box<dyn Module> {
    use ModuleType::*;
    Box::new(match module {
        Basic => basic::Basic::default(),
    })
}