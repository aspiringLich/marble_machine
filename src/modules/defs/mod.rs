use crate::*;
use strum::IntoEnumIterator;

use crate::modules::*;
use crate::modules::event::ModuleUpdate::*;
use strum_macros::EnumIter;

/// basic: asic modules that do standard stuff
pub mod basic;

#[derive(EnumIter, Clone, Copy, Component, Debug)]
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
}

impl std::ops::Deref for ModuleType {
    type Target = dyn Module;

    fn deref(&self) -> &Self::Target {
        unsafe {
            MODULES.get(*self as usize)
                .unwrap_or_else(|| {
                    error!("Could not deref ModuleType into Module! Maybe it is unititialized?");
                    &MODULES[0]
                })
                .as_ref()
        }
    }
}

impl std::ops::DerefMut for ModuleType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            MODULES.get_mut(*self as usize)
                .unwrap_or_else(|| {
                    error!("Could not deref_mut ModuleType into Module! Maybe it is unititialized?");
                    &mut MODULES[0]
                })
                .as_mut()
        }
    }
}

pub(super) static mut MODULES: Vec<Box<dyn super::Module>> = vec![];
pub(super) static mut MODULE_INFO: Vec<ModuleInfo> = vec![];

pub fn init_modules() {
    unsafe {
        for module in ModuleType::iter() {
            MODULES.push(get_module(module));
            MODULE_INFO.push((*module).info());
        }
    }
}

fn get_module(module: ModuleType) -> Box<dyn Module> {
    use ModuleType::*;
    Box::new(match module {
        Basic => basic::Basic::default(),
    })
}